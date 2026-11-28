//! `POST /register`: the arguments, and whose tool this is, once.

use std::sync::OnceLock;

use axum::Json;
use axum::http::StatusCode;
use diverge_sdk::container_proxy::inside::register;
use diverge_sdk::container_proxy::inside::register::response::Response;
use diverge_sdk::daemon::key;
use diverge_sdk::shared::containers::request::Which;
use serde_json::{Value, json};

use crate::arguments::Arguments;

/// What the one registration said.
pub struct Registered {
    /// The webhook the tool calls.
    pub arguments: Arguments,
    /// The agent the tool is deployed for.
    pub owner: key::Agent,
}

/// The one registration.
static REGISTERED: OnceLock<Registered> = OnceLock::new();

/// The registration; `None` before it.
pub fn registered() -> Option<&'static Registered> {
    REGISTERED.get()
}

/// A `400`, with its kind and why.
fn refused(kind: &str, error: &str) -> (StatusCode, Json<Value>) {
    (StatusCode::BAD_REQUEST, Json(json!({ "kind": kind, "error": error })))
}

/// `POST /register`. The rules:
///
/// 1. Arguments that are not [`Arguments`], or that name an empty
///    address or an empty key, are `400`.
/// 2. The tool is one agent's: a registration that says the tool is
///    owned by an agent keeps that agent; one that says anything else
///    is `400`, since the image's scope is `instance`.
/// 3. A second registration, whatever it carries, is `409`.
/// 4. Otherwise `200`, declaring no dependency and no permission.
pub async fn register(Json(request): Json<register::request::Request>) -> Result<(StatusCode, Json<Response>), (StatusCode, Json<Value>)> {
    // Rule 1.
    let arguments = serde_json::from_value::<Arguments>(request.arguments).map_err(|error| refused("arguments", &error.to_string()))?;
    if arguments.webhook_url.is_empty() || arguments.webhook_key.is_empty() {
        return Err(refused("arguments", "`webhook_url` and `webhook_key` are both required, and neither is empty"));
    }
    // Rule 2.
    let Which::Owner(key::Owner::Agent(owner)) = request.which else {
        return Err(refused("owner", "the tool is one agent's, and this registration names none"));
    };
    // Rules 3 and 4.
    if REGISTERED.set(Registered { arguments, owner }).is_err() {
        return Err((StatusCode::CONFLICT, Json(json!({ "kind": "registered", "error": "the tool is registered, and it never changes" }))));
    }
    Ok((StatusCode::OK, Json(Response::default())))
}
