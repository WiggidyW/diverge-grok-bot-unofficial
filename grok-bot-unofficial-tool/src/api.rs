//! The API, beneath `/api`: the agent's turn.
//!
//! The rules:
//!
//! 1. `POST` [`PATH`] is the agent's alone. A request before the
//!    tool's registration is `409`, `unregistered`. A request with no
//!    stamp, one stamped as a client or as a tool, or one stamped as
//!    any agent but the one the tool is deployed for, is `403`,
//!    `forbidden`.
//! 2. The body is JSON, `{"messages":[{"key":…,"content":[…]}]}`: the
//!    messages of the agent's run. No message, a message with no
//!    content, or content that is not text is `400`, `content`: the
//!    webhook carries text and nothing else, so images, audio and
//!    files are refused and not dropped.
//! 3. The messages' text, joined by blank lines in their order, is
//!    handed to the Bot through the webhook before the request is
//!    answered. A delivery that fails is `502`, `webhook`, with why.
//! 4. Otherwise the answer is `200`, server-sent events, each event's
//!    data one chunk of the agent's run: the messages' parts, then one
//!    `assistant_text_content` chunk stating that the message was
//!    delivered and where the Bot's answer appears. The webhook
//!    returns no reply, so the turn carries none.
//! 5. The stream stays open for [`LINGER`] after that chunk, and then
//!    ends. Nothing is read in that time: it stands in for the reading
//!    of the Bot's reply, which is not built.
//!
//! Every refusal's body is JSON, `{"kind":…,"error":…}`.

use std::convert::Infallible;
use std::time::Duration;

use axum::Json;
use axum::http::{HeaderMap, StatusCode};
use axum::response::sse::{Event, KeepAlive, Sse};
use diverge_sdk::container_proxy::inside::agent::enqueue::request::Request;
use diverge_sdk::container_proxy::inside::agent::run::request::Message;
use diverge_sdk::provider::endpoints::containers::agents::run::server::response::{AgenticLoopChunk, AssistantTextContentChunk, user_parts};
use diverge_sdk::shared::mcp::attestation::Caller;
use futures_util::Stream;
use rmcp::model::ContentBlock;
use serde_json::{Value, json};

use crate::register::registered;
use crate::webhook;

/// The path of the turn.
pub const PATH: &str = "/api/run";

/// What the turn says once the message is delivered (rule 4).
const DELIVERED: &str = "Delivered to Grok Bot. The webhook returns no reply: the Bot's answer appears in the Grok Bot app.";

/// How long the turn stays open after its delivery (rule 5).
const LINGER: Duration = Duration::from_secs(10);

/// A refusal: the status, and a JSON reason.
type Refusal = (StatusCode, Json<Value>);

/// A refusal of `kind`, with why.
fn refused(status: StatusCode, kind: &str, error: &str) -> Refusal {
    (status, Json(json!({ "kind": kind, "error": error })))
}

/// The caller, from the `diverge.network` header the daemon stamped
/// the request with, or why it has none.
fn caller(headers: &HeaderMap) -> Result<Caller, String> {
    let stamp = headers.get(diverge_sdk::shared::mcp::KEY).ok_or_else(|| "the request carries no stamp".to_string())?;
    let text = stamp.to_str().map_err(|error| error.to_string())?;
    serde_json::from_str::<Caller>(text).map_err(|error| error.to_string())
}

/// Rule 2's check of the content, and rule 3's text.
fn text(messages: &[Message]) -> Result<String, Refusal> {
    if messages.is_empty() || messages.iter().any(|message| message.content.is_empty()) {
        return Err(refused(StatusCode::BAD_REQUEST, "content", "a turn needs messages, each with content"));
    }
    let mut parts = Vec::new();
    for block in messages.iter().flat_map(|message| &message.content) {
        match block {
            ContentBlock::Text(text) => parts.push(text.text.as_str()),
            _ => return Err(refused(StatusCode::BAD_REQUEST, "content", "this agent takes text only: the webhook carries no image, audio or file")),
        }
    }
    Ok(parts.join("\n\n"))
}

/// Rules 1 to 5.
pub async fn run(headers: HeaderMap, Json(request): Json<Request>) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, Refusal> {
    // Rule 1.
    let Some(registered) = registered() else {
        return Err(refused(StatusCode::CONFLICT, "unregistered", "the tool is not registered"));
    };
    match caller(&headers) {
        Ok(Caller::Agent(agent)) if agent.key.template == registered.owner.template && agent.key.index == registered.owner.index => {}
        Ok(Caller::Agent(_)) => return Err(refused(StatusCode::FORBIDDEN, "forbidden", "another agent's tool")),
        Ok(_) => return Err(refused(StatusCode::FORBIDDEN, "forbidden", "not the tool's agent")),
        Err(why) => return Err(refused(StatusCode::FORBIDDEN, "forbidden", &why)),
    }
    // Rules 2 and 3.
    let text = text(&request.messages)?;
    if let Err(why) = webhook::deliver(&registered.arguments, &text).await {
        return Err(refused(StatusCode::BAD_GATEWAY, "webhook", &why.to_string()));
    }
    // Rule 4.
    let mut chunks: Vec<AgenticLoopChunk> = request.messages.into_iter().flat_map(|message| user_parts(&message.key, message.content)).collect();
    chunks.push(AgenticLoopChunk::AssistantTextContent(AssistantTextContentChunk {
        r#type: Default::default(),
        parent_tool_call_id: None,
        logprobs: None,
        inner: rmcp::model::TextContent::new(DELIVERED.to_string()),
    }));
    let events = async_stream::stream! {
        for chunk in chunks {
            if let Ok(data) = serde_json::to_string(&chunk) {
                yield Ok(Event::default().data(data));
            }
        }
        // Rule 5.
        tokio::time::sleep(LINGER).await;
    };
    Ok(Sse::new(events).keep_alive(KeepAlive::default()))
}
