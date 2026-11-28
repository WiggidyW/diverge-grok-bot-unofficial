//! `grok-bot-unofficial-tool`, as a tool container: where a Grok Bot
//! is reached, for the `grok-bot-unofficial-agent` agent it is
//! deployed for.
//!
//! The agent is a thin wrapper. When a run comes to it, it posts the
//! run's messages to this tool's API; the tool hands their text to the
//! Bot through one routine's webhook, the one way in the vendor
//! documents, and answers with the chunks of the agent's run.
//!
//! The program is an HTTP server on the container's loopback, at the
//! port [`diverge_sdk::container_proxy::inside::port()`] names, and the
//! proxy beside it is its only caller: `POST /register` once
//! ([`register`]), the MCP server at `/mcp`, which serves no tool
//! ([`mcp`]), the agent's turn beneath `/api` ([`api`]), and the
//! frontend beneath `/frontend` ([`frontend`]).
//! `--schema`, the program's one argument, prints the JSON Schema of
//! the arguments it takes, which is the `schema` of the image's
//! `/.diverge/DIVERGE.md` frontmatter.
//!
//! # The app
//!
//! The Grok Bot desktop app is fetched from the vendor at startup and
//! is not in the image: see [`install`]. It runs on the tool's
//! [`desktop`], beside a browser, and the frontend shows both: that is
//! where a person signs in to it. Everything the app sends and
//! receives is recorded by a proxy beside it: see [`traffic`].

mod api;
mod arguments;
mod caller;
mod desktop;
mod frontend;
mod install;
mod mcp;
mod register;
mod traffic;
mod webhook;

use std::io::Write as _;
use std::sync::Arc;

use rmcp::transport::streamable_http_server::session::local::LocalSessionManager;
use rmcp::transport::{StreamableHttpServerConfig, StreamableHttpService};

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
            let _ = writeln!(std::io::stderr(), "usage: grok-bot-unofficial-tool [--schema]");
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

/// The server, for the container's life, bound on the loopback; or,
/// run [by hand](caller::by_hand), on every address. The install and
/// the desktop start beside it; the desktop waits on the install, and
/// the server on neither.
async fn serve() {
    tokio::spawn(async {
        match install::installed().await {
            Ok(installed) => eprintln!("Grok Bot {} is installed", installed.version),
            Err(why) => eprintln!("Grok Bot was not installed: {why}"),
        }
    });
    tokio::spawn(desktop::run());
    let served = StreamableHttpService::new(|| Ok(mcp::Nothing), Arc::new(LocalSessionManager::default()), StreamableHttpServerConfig::default());
    let app = axum::Router::new()
        .route("/register", axum::routing::post(register::register))
        .nest_service("/mcp", served)
        .route(api::PATH, axum::routing::post(api::run))
        .merge(frontend::routes());
    let address = if caller::by_hand() { "0.0.0.0" } else { "127.0.0.1" };
    let listener = tokio::net::TcpListener::bind((address, diverge_sdk::container_proxy::inside::port())).await.expect("the port could not be bound");
    axum::serve(listener, app).await.expect("the server stopped unexpectedly");
}
