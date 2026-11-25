//! `POST /secrets` and `POST /secrets/collect`: the webhook's address
//! and key, as the proxy feeds them, held in memory.
//!
//! Every feed replaces the whole set. No value is ever written into an
//! error or a log: a failure names the secret, and nothing else.

use axum::Json;
use axum::http::StatusCode;
use diverge_sdk::container_proxy::inside::secrets::{collect, feed};
use diverge_sdk::shared::containers::secrets::Values;
use serde_json::{Value, json};
use tokio::sync::RwLock;

/// The secret naming the webhook's address.
pub const WEBHOOK_URL: &str = "GROK_BOT_WEBHOOK_URL";
/// The secret naming the webhook's key.
pub const WEBHOOK_KEY: &str = "GROK_BOT_WEBHOOK_KEY";

/// The values last fed, or none yet.
static VALUES: RwLock<Option<Values>> = RwLock::const_new(None);

/// One secret's value as text, or the name of the one that is missing
/// or is not a string.
pub async fn text(name: &'static str) -> Result<String, &'static str> {
    let held = VALUES.read().await;
    match held.as_ref().and_then(|values| values.get(name)) {
        Some(Value::String(value)) if !value.is_empty() => Ok(value.clone()),
        _ => Err(name),
    }
}

/// `POST /secrets`: the values fed, held; `204`.
pub async fn feed(Json(request): Json<feed::request::Request>) -> StatusCode {
    *VALUES.write().await = Some(request.0);
    StatusCode::NO_CONTENT
}

/// `POST /secrets/collect`: the rotating secrets asked back. Both of
/// this image's are `fixed`, so the proxy never asks; served as the
/// contract all the same: every name asked, with the value it was fed,
/// and `500` naming the first one not held.
pub async fn collect(Json(request): Json<collect::request::Request>) -> Result<Json<collect::response::Response>, (StatusCode, Json<Value>)> {
    let held = VALUES.read().await;
    let mut collected = Values::with_capacity(request.names.len());
    for name in &request.names {
        match held.as_ref().and_then(|values| values.get(name)) {
            Some(value) => {
                collected.insert(name.clone(), value.clone());
            }
            None => return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "kind": "secrets", "error": format!("the agent holds no secret named {name}") })))),
        }
    }
    Ok(Json(collect::response::Response(collected)))
}
