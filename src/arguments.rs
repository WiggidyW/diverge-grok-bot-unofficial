//! What the agent takes as its arguments: nothing.

use serde::{Deserialize, Serialize};

/// The arguments a Grok Bot agent is made with.
///
/// None: which Bot the agent speaks to is decided by the webhook its
/// secrets name. `{}` is the one value; any member is refused, so a
/// misspelled setting is never silently ignored.
///
/// # Secrets
///
/// `GROK_BOT_WEBHOOK_URL` and `GROK_BOT_WEBHOOK_KEY`: the address and
/// the key of one routine's webhook, as the Grok Bot app shows them.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Arguments {}
