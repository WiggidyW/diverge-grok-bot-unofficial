//! `POST /register`: the arguments, once.

use axum::Json;
use axum::http::StatusCode;
use diverge_sdk::container_proxy::inside::register;
use diverge_sdk::container_proxy::inside::register::response::Response;
use diverge_sdk::shared::containers::secrets::{Declaration, Mode};
use serde_json::{Value, json};
use tokio::sync::OnceCell;

use crate::arguments::Arguments;
use crate::secrets::{WEBHOOK_KEY, WEBHOOK_URL};

/// Whether the one registration has happened.
static REGISTERED: OnceCell<()> = OnceCell::const_new();

/// Whether the agent is registered: a run before it is refused.
pub fn registered() -> bool {
    REGISTERED.initialized()
}

/// `POST /register`: arguments that are not [`Arguments`] are `400`; a
/// second registration, whatever it carries, is `409`; otherwise `200`,
/// declaring no dependency, no permission, and the webhook's two
/// secrets, both `fixed`.
pub async fn register(Json(request): Json<register::request::Request>) -> Result<(StatusCode, Json<Response>), (StatusCode, Json<Value>)> {
    if let Err(error) = serde_json::from_value::<Arguments>(request.arguments) {
        return Err((StatusCode::BAD_REQUEST, Json(json!({ "kind": "arguments", "error": error.to_string() }))));
    }
    let secrets = Declaration::from([(WEBHOOK_URL.to_string(), Mode::Fixed), (WEBHOOK_KEY.to_string(), Mode::Fixed)]);
    match REGISTERED.set(()) {
        Ok(()) => Ok((StatusCode::OK, Json(Response { dependencies: Vec::new(), secrets, permissions: Vec::new() }))),
        Err(_) => Err((StatusCode::CONFLICT, Json(json!({ "kind": "registered", "error": "the agent is registered, and it never changes" })))),
    }
}
