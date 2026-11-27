//! `POST /register`: the arguments, once, held for the container's
//! life.

use std::sync::OnceLock;

use axum::Json;
use axum::http::StatusCode;
use diverge_sdk::container_proxy::inside::register;
use diverge_sdk::container_proxy::inside::register::response::Response;
use serde_json::{Value, json};

use crate::arguments::Arguments;

/// The arguments of the one registration.
static ARGUMENTS: OnceLock<Arguments> = OnceLock::new();

/// The arguments the agent was registered with, or none yet: a run
/// before the registration is refused.
pub fn arguments() -> Option<&'static Arguments> {
    ARGUMENTS.get()
}

/// A `400` over the arguments, with why.
fn refused(error: &str) -> (StatusCode, Json<Value>) {
    (StatusCode::BAD_REQUEST, Json(json!({ "kind": "arguments", "error": error })))
}

/// `POST /register`: arguments that are not [`Arguments`], or that
/// name an empty address or an empty key, are `400`; a second
/// registration, whatever it carries, is `409`; otherwise `200`,
/// declaring no dependency and no permission.
pub async fn register(Json(request): Json<register::request::Request>) -> Result<(StatusCode, Json<Response>), (StatusCode, Json<Value>)> {
    let arguments = serde_json::from_value::<Arguments>(request.arguments).map_err(|error| refused(&error.to_string()))?;
    if arguments.webhook_url.is_empty() || arguments.webhook_key.is_empty() {
        return Err(refused("`webhook_url` and `webhook_key` are both required, and neither is empty"));
    }
    match ARGUMENTS.set(arguments) {
        Ok(()) => Ok((StatusCode::OK, Json(Response::default()))),
        Err(_) => Err((StatusCode::CONFLICT, Json(json!({ "kind": "registered", "error": "the agent is registered, and it never changes" })))),
    }
}
