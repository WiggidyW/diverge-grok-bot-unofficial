//! `grok-bot-unofficial-agent`, as an agent container: a thin wrapper
//! over a Grok Bot.
//!
//! Grok Bot is not here. It is reached from a tool container of the
//! agent's own, the `grok-bot-unofficial-tool` image, which the agent
//! declares as its one dependency and reaches over the daemon, through
//! the tool's API.
//!
//! A run comes in as any agent's does: the proxy posts the messages to
//! `/run`. The agent posts them to its tool, and the events the tool
//! answers with are the run's.
//!
//! The program is an HTTP server on the container's loopback, at the
//! port [`diverge_sdk::container_proxy::inside::port()`] names, and the
//! proxy beside it is its only caller: `POST /register` once
//! ([`register`]), and `POST /run` and `POST /enqueue` ([`run`]). What
//! it asks of its tool goes out through the same proxy, on its daemon
//! connection ([`tool`]). `--schema`, the program's one argument,
//! prints the JSON Schema of the arguments it takes, which is the
//! `schema` of the image's `/.diverge/DIVERGE.md` frontmatter.
//!
//! # Nothing of the proxy's before it is needed
//!
//! The host injects the proxy at runtime, and it may not be up until a
//! request comes. So nothing here touches it at registration: the
//! daemon is reached on the first run, and kept.

mod arguments;
mod register;
mod run;
mod tool;

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
            let _ = writeln!(std::io::stderr(), "usage: grok-bot-unofficial-agent [--schema]");
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

/// The server, for the container's life, bound on the loopback.
async fn serve() {
    let app = axum::Router::new()
        .route("/register", axum::routing::post(register::register))
        .route("/run", axum::routing::post(run::run))
        .route("/enqueue", axum::routing::post(run::enqueue));
    let listener = tokio::net::TcpListener::bind(("127.0.0.1", diverge_sdk::container_proxy::inside::port())).await.expect("the port could not be bound");
    axum::serve(listener, app).await.expect("the server stopped unexpectedly");
}
