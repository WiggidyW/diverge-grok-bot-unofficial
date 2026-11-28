//! What the tool takes as its arguments: one routine's webhook.

use serde::{Deserialize, Serialize};

/// The arguments a Grok Bot tool is made with: the webhook of one
/// routine, which decides which Bot the tool speaks to.
///
/// The `grok-bot-unofficial-agent` image declares this tool as its
/// dependency and hands its own `webhook_url` and `webhook_key` on as
/// these. Both members are required, and any other member is refused,
/// so a misspelled setting is never silently ignored. The arguments
/// are stored with the agent's template and are not hidden: whoever
/// can read the template can read the key.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Arguments {
    /// The address the routine's webhook is called at, shown as
    /// "POST to" in the Grok Bot app.
    #[schemars(length(min = 1))]
    pub webhook_url: String,
    /// The key of the same webhook, sent as its bearer token.
    #[schemars(length(min = 1))]
    pub webhook_key: String,
}
