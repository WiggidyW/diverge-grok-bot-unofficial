//! An unofficial Diverge agent container that hands its messages to a
//! Grok Bot.
//!
//! The program is an HTTP server on the container's loopback, at the
//! port [`diverge_sdk::container_proxy::inside::port()`] names, and the
//! proxy beside it is its only caller: `POST /register` once, and
//! `POST /run` and `POST /enqueue`. `--schema`, the program's one
//! argument, prints the JSON Schema of the arguments it takes, which is
//! the `schema` of the image's `/.diverge/DIVERGE.md` frontmatter.
//!
//! # What it does today
//!
//! A run hands its messages' text to the Bot through one routine's
//! webhook, the one way in the vendor documents, named by the
//! arguments. The webhook returns
//! no reply, so the run carries none: see [`run`].
//!
//! # The app
//!
//! The Grok Bot desktop app is fetched from the vendor at startup and
//! is not in the image: see [`install`]. Nothing uses it yet.

mod arguments;
mod install;
mod register;
mod run;
mod webhook;

use std::io::Write as _;

use crate::arguments::Arguments;

/// The program: the server, or, on `--schema`, the schema.
///
/// `--schema` alone prints the JSON Schema of [`Arguments`] on stdout
/// and exits 0; no argument is the server, for the container's life;
/// anything else is a usage line on stderr and exit 2.
fn main() {
    let mut arguments = std::env::args_os().skip(1);
    match (arguments.next(), arguments.next()) {
        (None, None) => {}
        (Some(argument), None) if argument == "--schema" => return schema(),
        _ => {
            let _ = writeln!(std::io::stderr(), "usage: diverge-grok-bot-unofficial [--schema]");
            std::process::exit(2);
        }
    }
    let runtime = tokio::runtime::Builder::new_multi_thread().enable_all().build().expect("the runtime could not be built");
    runtime.block_on(serve());
}

/// `--schema`: the JSON Schema of [`Arguments`], pretty-printed on
/// stdout with a trailing newline.
fn schema() {
    let text = match serde_json::to_string_pretty(&schemars::schema_for!(Arguments)) {
        Ok(text) => text,
        Err(error) => {
            let _ = writeln!(std::io::stderr(), "{error}");
            std::process::exit(1);
        }
    };
    if let Err(error) = writeln!(std::io::stdout(), "{text}") {
        let _ = writeln!(std::io::stderr(), "{error}");
        std::process::exit(1);
    }
}

/// The server, for the container's life, bound on the loopback. The
/// install starts beside it and is waited on by nothing; its outcome
/// goes to stderr, which is the container's log.
async fn serve() {
    tokio::spawn(async {
        match install::installed().await {
            Ok(version) => eprintln!("Grok Bot {version} is installed"),
            Err(why) => eprintln!("Grok Bot was not installed: {why}"),
        }
    });
    let app = axum::Router::new()
        .route("/register", axum::routing::post(register::register))
        .route("/run", axum::routing::post(run::run))
        .route("/enqueue", axum::routing::post(run::enqueue));
    let listener = tokio::net::TcpListener::bind(("127.0.0.1", diverge_sdk::container_proxy::inside::port())).await.expect("the port could not be bound");
    axum::serve(listener, app).await.expect("the server stopped unexpectedly");
}
