//! `POST /run` and `POST /enqueue`: a run is one turn of the agent's
//! tool.
//!
//! The rules:
//!
//! 1. A run that is not a chat run is `400`, `content`. A run before
//!    the registration is `409`, `unregistered`.
//! 2. The run's messages are posted to the tool, and the tool's answer
//!    is had before the run is answered. A tool that refuses with
//!    `400` is the run's `400`; a tool that refuses with anything
//!    else, or is not reached, is the run's `502`. Either carries the
//!    tool's reason: `{"kind":"tool","status":…,"reason":…}`.
//! 3. Otherwise the run is `200`, server-sent events: every event the
//!    tool sends, its data unchanged, in the tool's order, until the
//!    tool ends its stream, which ends the run. The agent adds no
//!    chunk of its own and reads none.
//! 4. A comment goes out on the stream when it is quiet, so the
//!    connection is never idle for long; the proxy reads only data.
//! 5. An enqueue is `409`, `closed`, at any time: the tool takes
//!    nothing after a turn's messages, so the proxy starts the next
//!    run on it.

use std::convert::Infallible;

use axum::Json;
use axum::http::StatusCode;
use axum::response::sse::{Event, KeepAlive, Sse};
use diverge_sdk::container_proxy::inside::agent::enqueue;
use diverge_sdk::container_proxy::inside::agent::run;
use diverge_sdk::daemon::endpoints::dependency_tools::api::server::response::Frame;
use futures_util::{Stream, StreamExt as _};
use serde_json::{Value, json};

use crate::tool::{self, Refused};

/// A refusal: the status, and a JSON reason.
type Refusal = (StatusCode, Json<Value>);

/// The data of one server-sent event: its `data` lines, joined by
/// newlines; `None` where it has none, as a comment has none.
fn data(event: &[u8]) -> Option<String> {
    let text = String::from_utf8_lossy(event);
    let lines: Vec<&str> = text.lines().filter_map(|line| line.strip_prefix("data:")).map(|line| line.strip_prefix(' ').unwrap_or(line)).collect();
    (!lines.is_empty()).then(|| lines.join("\n"))
}

/// Rules 1 to 4.
pub async fn run(Json(request): Json<run::request::Request>) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, Refusal> {
    // Rule 1.
    let messages = match request {
        run::request::Request::Chat { messages } => messages,
        run::request::Request::StructuredDecisions(_) => return Err((StatusCode::BAD_REQUEST, Json(json!({ "kind": "content", "error": "structured decisions are not supported by this agent" })))),
    };
    // Rule 2.
    let mut answers = match tool::begin(messages).await {
        Ok(answers) => answers,
        Err(Refused::Unregistered) => return Err((StatusCode::CONFLICT, Json(json!({ "kind": "unregistered", "error": "the agent is not registered" })))),
        Err(Refused::Answered(status, reason)) => {
            let answer = if status == 400 { StatusCode::BAD_REQUEST } else { StatusCode::BAD_GATEWAY };
            return Err((answer, Json(json!({ "kind": "tool", "status": status, "reason": reason }))));
        }
        Err(Refused::Unreached(why)) => return Err((StatusCode::BAD_GATEWAY, Json(json!({ "kind": "tool", "reason": { "error": why } })))),
    };
    // Rule 3.
    let events = async_stream::stream! {
        // What has come down and is not yet a whole event.
        let mut buffer: Vec<u8> = Vec::new();
        while let Some(Ok(Frame::Body(bytes))) = answers.next().await {
            buffer.extend_from_slice(&bytes);
            while let Some(at) = buffer.windows(2).position(|pair| pair == b"\n\n") {
                let event: Vec<u8> = buffer.drain(..at + 2).collect();
                if let Some(data) = data(&event) {
                    yield Ok(Event::default().data(data));
                }
            }
        }
    };
    Ok(Sse::new(events).keep_alive(KeepAlive::default()))
}

/// Rule 5.
pub async fn enqueue(Json(_request): Json<enqueue::request::Request>) -> Result<StatusCode, Refusal> {
    Err((StatusCode::CONFLICT, Json(json!({ "kind": "closed", "error": "no turn is open" }))))
}
