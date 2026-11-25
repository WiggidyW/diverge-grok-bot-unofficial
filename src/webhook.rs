//! The one official way into a Grok Bot from outside: a routine's
//! webhook.
//!
//! The rules, as the Grok Bot documentation states them:
//!
//! 1. The call is a `POST` to the routine's address with
//!    `Authorization: Bearer <key>` and a JSON body.
//! 2. `200` states that the call was accepted and a run started. It
//!    does not state that the Bot finished, and it carries no reply.
//! 3. Any other status states that no run started.
//!
//! The Bot receives the body together with the routine's own
//! instruction. Nothing comes back by this route: the Bot's answer
//! appears in the Grok Bot app.

use std::sync::OnceLock;

use serde_json::json;

use crate::secrets::{WEBHOOK_KEY, WEBHOOK_URL, text};

/// The one HTTP client, made on first use.
fn client() -> &'static reqwest::Client {
    static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();
    CLIENT.get_or_init(reqwest::Client::new)
}

/// Why a delivery did not happen. No variant carries a secret's value.
#[derive(Debug)]
pub enum Refused {
    /// A secret the call needs was not fed, or is not text: its name.
    Secret(&'static str),
    /// The call could not be made at all.
    Unreachable(&'static str),
    /// The webhook answered, and not with `200`: no run started.
    Status(u16),
}

impl std::fmt::Display for Refused {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Refused::Secret(name) => write!(f, "the secret `{name}` was not fed, or is not text"),
            Refused::Unreachable(why) => write!(f, "the webhook could not be reached: {why}"),
            Refused::Status(status) => write!(f, "the webhook answered {status}: no run started"),
        }
    }
}

/// Hand `message` to the Bot (rules 1 to 3): `{"text": <message>}`.
pub async fn deliver(message: &str) -> Result<(), Refused> {
    let url = text(WEBHOOK_URL).await.map_err(Refused::Secret)?;
    let key = text(WEBHOOK_KEY).await.map_err(Refused::Secret)?;
    let answer = client().post(url).bearer_auth(key).json(&json!({ "text": message })).send().await.map_err(|error| {
        // The error's own text may quote the address, which is a
        // secret: only its kind is kept.
        Refused::Unreachable(if error.is_timeout() {
            "timed out"
        } else if error.is_connect() {
            "no connection"
        } else {
            "the request failed"
        })
    })?;
    match answer.status().as_u16() {
        200 => Ok(()),
        status => Err(Refused::Status(status)),
    }
}
