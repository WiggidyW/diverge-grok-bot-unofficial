//! `POST /register`: the arguments, once.

use std::sync::OnceLock;

use axum::Json;
use axum::http::StatusCode;
use diverge_sdk::container_proxy::inside::register;
use diverge_sdk::container_proxy::inside::register::response::Response;
use diverge_sdk::daemon::key;
use diverge_sdk::shared::containers::request::Which;
use serde_json::{Value, json};

use crate::arguments::Arguments;

/// Whether the one registration has happened.
static REGISTERED: OnceLock<()> = OnceLock::new();

/// Whether the tool is registered: a turn before it is refused.
pub fn registered() -> bool {
    REGISTERED.get().is_some()
}

/// A `400`, with its kind and why.
fn refused(kind: &str, error: &str) -> (StatusCode, Json<Value>) {
    (StatusCode::BAD_REQUEST, Json(json!({ "kind": kind, "error": error })))
}

/// `POST /register`. The rules:
///
/// 1. Arguments that are not [`Arguments`] are `400`.
/// 2. The tool is every agent's that declares it: a registration that
///    says the tool is owned by its template is taken; one that says
///    anything else is `400`, since the image's scope is `template`.
/// 3. A second registration, whatever it carries, is `409`.
/// 4. Otherwise `200`, declaring no dependency and no permission.
pub async fn register(Json(request): Json<register::request::Request>) -> Result<(StatusCode, Json<Response>), (StatusCode, Json<Value>)> {
    // Rule 1.
    serde_json::from_value::<Arguments>(request.arguments).map_err(|error| refused("arguments", &error.to_string()))?;
    // Rule 2.
    let Which::Owner(key::Owner::Template) = request.which else {
        return Err(refused("owner", "the tool is its template's, and this registration says otherwise"));
    };
    // Rules 3 and 4.
    if REGISTERED.set(()).is_err() {
        return Err((StatusCode::CONFLICT, Json(json!({ "kind": "registered", "error": "the tool is registered, and it never changes" }))));
    }
    Ok((StatusCode::OK, Json(Response::default())))
}
