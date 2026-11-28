//! The tool, as the agent reaches it: one request on the tool's API,
//! through the daemon, for each run.
//!
//! The tool is the agent's own dependency tool, a container beside it,
//! where Grok Bot is reached. The rules:
//!
//! 1. A turn is `POST` [`PATH`] with a JSON body, the run's messages:
//!    `{"messages":[{"key":…,"content":[…]}]}`.
//! 2. The tool answers `200` with server-sent events, or any other
//!    status with a JSON reason and no events. Which it is, is known
//!    before the run is answered.
//! 3. A tool that cannot be reached at all is a refusal too, with why.

use std::sync::OnceLock;

use diverge_sdk::container_proxy::inside::Client;
use diverge_sdk::daemon::endpoints::dependency_tools::api;
use diverge_sdk::daemon::endpoints::dependency_tools::api::server::response::Frame;
use diverge_sdk::shared::containers::api::request::Head;
use diverge_sdk::wire::client::handle::Handle;
use futures_util::StreamExt as _;
use serde_json::{Value, json};
use tokio::sync::Mutex;

use crate::register;

/// The path of the tool's API a turn is posted to.
const PATH: &str = "/api/run";

/// The most of a refusal's body that is read.
const REASON: usize = 64 * 1024;

/// The proxy beside the program, which the daemon is dialled through.
fn client() -> &'static Client {
    static CLIENT: OnceLock<Client> = OnceLock::new();
    CLIENT.get_or_init(Client::new)
}

/// The connection to the daemon: dialled on first use, kept, and
/// dialled again when a turn finds it gone.
static HANDLE: Mutex<Option<Handle>> = Mutex::const_new(None);

/// Why a turn was not opened.
pub enum Refused {
    /// The agent is not registered, so it has no tool yet.
    Unregistered,
    /// The tool answered, and not with `200` (rule 2): its status, and
    /// its reason.
    Answered(u16, Value),
    /// The tool was not reached (rule 3), with why.
    Unreached(String),
}

/// The connection held, or a new one dialled.
async fn handle() -> Result<Handle, String> {
    let mut held = HANDLE.lock().await;
    if let Some(handle) = held.as_ref() {
        return Ok(handle.clone());
    }
    let handle = client().daemon().await.map_err(|error| error.to_string())?;
    *held = Some(handle.clone());
    Ok(handle)
}

/// Rules 1 to 3: the turn opened on `body`, and the stream its events
/// come down.
pub async fn begin(body: Vec<u8>) -> Result<api::client::execute::ExecuteStream, Refused> {
    let Some(dependency) = register::tool() else {
        return Err(Refused::Unregistered);
    };
    let handle = handle().await.map_err(Refused::Unreached)?;
    // Rule 1.
    let request = api::client::request::Frame {
        dependency: dependency.clone(),
        head: Head {
            method: "POST".to_string(),
            path: PATH.to_string(),
            headers: vec![("content-type".to_string(), "application/json".to_string()), ("content-length".to_string(), body.len().to_string())],
        },
        relay: None,
    };
    let (mut answers, way) = match api::client::execute::execute(&handle, &request).await {
        Ok(opened) => opened,
        Err(error) => {
            // The next turn dials again.
            *HANDLE.lock().await = None;
            return Err(Refused::Unreached(error.to_string()));
        }
    };
    if way.body(&body).await.is_err() || way.end().await.is_err() {
        *HANDLE.lock().await = None;
        return Err(Refused::Unreached("the connection to the daemon was lost".to_string()));
    }
    // Rules 2 and 3.
    match answers.next().await {
        Some(Ok(Frame::Head(head))) if head.status == 200 => Ok(answers),
        Some(Ok(Frame::Head(head))) => Err(Refused::Answered(head.status, reason(&mut answers).await)),
        Some(Ok(Frame::NotFound)) => Err(Refused::Unreached("the daemon finds no tool of this agent's".to_string())),
        Some(Ok(Frame::Forbidden)) => Err(Refused::Unreached("the agent may not reach its tool".to_string())),
        Some(Ok(Frame::NoApi)) => Err(Refused::Unreached("the tool's image declares no API".to_string())),
        Some(Ok(Frame::Error(error))) => Err(Refused::Unreached(error.0.to_string())),
        Some(Ok(Frame::Body(_))) => Err(Refused::Unreached("the tool answered a body before a head".to_string())),
        Some(Err(error)) => Err(Refused::Unreached(error.to_string())),
        None => Err(Refused::Unreached("the tool did not answer".to_string())),
    }
}

/// A refusal's body: the JSON it is, or, where it is not JSON, its
/// text under `error`.
async fn reason(answers: &mut api::client::execute::ExecuteStream) -> Value {
    let mut body: Vec<u8> = Vec::new();
    while let Some(Ok(Frame::Body(bytes))) = answers.next().await {
        body.extend_from_slice(&bytes);
        if body.len() > REASON {
            break;
        }
    }
    serde_json::from_slice(&body).unwrap_or_else(|_| json!({ "error": String::from_utf8_lossy(&body) }))
}
