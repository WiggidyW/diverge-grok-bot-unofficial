//! Who is asking: the agent behind an API request, the person behind a
//! frontend request, each as the daemon attests it and never as the
//! request says.

use axum::http::HeaderMap;
use diverge_sdk::shared::mcp::attestation::Caller;

/// The variable that, set to `1`, runs the tool by hand: see
/// [`by_hand`].
pub const BY_HAND: &str = "GROK_BOT_TOOL_UNSTAMPED";

/// Whether the tool is run by hand, with no Diverge around it: the
/// variable [`BY_HAND`] is `1`. Then the server binds every address
/// and not the loopback alone, and a frontend request with no stamp is
/// taken as a person's. For trying the image with a container engine
/// alone, and for nothing else: under Diverge it is never set.
pub fn by_hand() -> bool {
    std::env::var(BY_HAND).is_ok_and(|value| value == "1")
}

/// The caller, from the `diverge.network` header the daemon stamped
/// the request with, or why it has none.
pub fn caller(headers: &HeaderMap) -> Result<Caller, String> {
    let stamp = headers.get(diverge_sdk::shared::mcp::KEY).ok_or_else(|| "the request carries no stamp".to_string())?;
    let text = stamp.to_str().map_err(|error| error.to_string())?;
    serde_json::from_str::<Caller>(text).map_err(|error| error.to_string())
}

/// Whether a frontend request is a person's, or why it is refused: one
/// stamped as a client is; one stamped as an agent or as a tool is
/// not; one with no stamp is only where the tool is run [`by_hand`].
pub fn person(headers: &HeaderMap) -> Result<(), String> {
    if headers.get(diverge_sdk::shared::mcp::KEY).is_none() && by_hand() {
        return Ok(());
    }
    match caller(headers)? {
        Caller::Client { .. } => Ok(()),
        Caller::Agent(_) => Err("an agent, not a person".to_string()),
        Caller::Tool { .. } => Err("a tool, not a person".to_string()),
    }
}
