//! `POST /run` and `POST /enqueue`: a turn is one webhook call.
//!
//! The rules:
//!
//! 1. A run before the registration is `409`, `unregistered`. A run
//!    that is not a chat run is `400`. A run with no message, a message
//!    with no content, or content that is not text is `400`: the
//!    webhook carries text and nothing else, so images, audio and
//!    files are refused and not dropped.
//! 2. The messages' text, joined by blank lines in their order, is
//!    handed to the Bot through the webhook before the run is
//!    answered. A delivery that fails is `502`, with why, and no run.
//! 3. Otherwise the run is `200`, server-sent events: the messages'
//!    parts, then one `assistant_text_content` chunk stating that the
//!    message was delivered and where the Bot's answer appears. The
//!    webhook returns no reply, so the run carries none.
//! 4. The stream stays open for [`LINGER`] after that chunk, and then
//!    ends. Nothing is read in that time: it stands in for the reading
//!    of the Bot's reply, which is not built.
//! 5. An enqueue is `409`, `closed`, at any time: the run takes nothing
//!    after its delivery, so the proxy starts the next run on it.

use std::convert::Infallible;
use std::time::Duration;

use axum::Json;
use axum::http::StatusCode;
use axum::response::sse::{Event, KeepAlive, Sse};
use diverge_sdk::container_proxy::inside::agent::enqueue;
use diverge_sdk::container_proxy::inside::agent::run;
use diverge_sdk::container_proxy::inside::agent::run::request::Message;
use diverge_sdk::provider::endpoints::containers::agents::run::server::response::{AgenticLoopChunk, AssistantTextContentChunk, user_parts};
use futures_util::Stream;
use rmcp::model::ContentBlock;
use serde_json::{Value, json};

use crate::register::registered;
use crate::webhook;

/// What the run says once the message is delivered (rule 3).
const DELIVERED: &str = "Delivered to Grok Bot. The webhook returns no reply: the Bot's answer appears in the Grok Bot app.";

/// How long the run stays open after its delivery (rule 4).
const LINGER: Duration = Duration::from_secs(10);

/// A refusal: the status, and a JSON reason.
type Refusal = (StatusCode, Json<Value>);

/// A `400` over the content, with why.
fn content(error: &str) -> Refusal {
    (StatusCode::BAD_REQUEST, Json(json!({ "kind": "content", "error": error })))
}

/// Rule 1's check of the content, and rule 2's text: every message's
/// text blocks, joined by blank lines.
fn text(messages: &[Message]) -> Result<String, Refusal> {
    if messages.is_empty() || messages.iter().any(|message| message.content.is_empty()) {
        return Err(content("a run needs messages, each with content"));
    }
    let mut parts = Vec::new();
    for block in messages.iter().flat_map(|message| &message.content) {
        match block {
            ContentBlock::Text(text) => parts.push(text.text.as_str()),
            _ => return Err(content("this agent takes text only: the webhook carries no image, audio or file")),
        }
    }
    Ok(parts.join("\n\n"))
}

/// Rules 1 to 4.
pub async fn run(Json(request): Json<run::request::Request>) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, Refusal> {
    let messages = match request {
        run::request::Request::Chat { messages } => messages,
        run::request::Request::StructuredDecisions(_) => return Err(content("structured decisions are not supported by this agent")),
    };
    if !registered() {
        return Err((StatusCode::CONFLICT, Json(json!({ "kind": "unregistered", "error": "the agent is not registered" }))));
    }
    let text = text(&messages)?;
    if let Err(refused) = webhook::deliver(&text).await {
        return Err((StatusCode::BAD_GATEWAY, Json(json!({ "kind": "webhook", "error": refused.to_string() }))));
    }
    let mut chunks: Vec<AgenticLoopChunk> = messages.into_iter().flat_map(|message| user_parts(&message.key, message.content)).collect();
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
        // Rule 4.
        tokio::time::sleep(LINGER).await;
    };
    Ok(Sse::new(events).keep_alive(KeepAlive::default()))
}

/// Rule 5.
pub async fn enqueue(Json(request): Json<enqueue::request::Request>) -> Result<StatusCode, Refusal> {
    text(&request.messages)?;
    Err((StatusCode::CONFLICT, Json(json!({ "kind": "closed", "error": "no turn is open" }))))
}
