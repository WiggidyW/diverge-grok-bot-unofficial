//! What the agent takes as its arguments: its tool's image, and one
//! routine's webhook.

use diverge_sdk::shared::containers::request::Image;
use serde::{Deserialize, Serialize};

/// The arguments a Grok Bot agent is made with.
///
/// The agent is a thin wrapper: Grok Bot is reached from a tool
/// container of its own, the `grok-bot-unofficial-tool` image, which
/// the agent declares as its one dependency. The agent must be told
/// which image that is and which account it is for, and names the
/// webhook to it on every run. One tool is one Grok Bot account: every
/// agent made with the same `tool` and `account` shares one container,
/// and agents of another account have another. Every member
/// is required, and any other member is refused, so a misspelled
/// setting is never silently ignored. The arguments are stored with
/// the agent's template and are not hidden: whoever can read the
/// template can read the key.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Arguments {
    /// The `grok-bot-unofficial-tool` image the agent's tool is made
    /// from: its digest, and the references under which its bytes may
    /// be fetched. For testing; a released agent image will name it
    /// itself.
    pub tool: Image,
    /// A name for the Grok Bot account the agent's Bot belongs to,
    /// chosen by whoever makes the agents: any text that is not empty.
    /// It is a label and not a credential. Agents with the same
    /// account share one tool; a different account is a different
    /// tool.
    #[schemars(length(min = 1))]
    pub account: String,
    /// The address a routine's webhook is called at, shown as
    /// "POST to" in the Grok Bot app.
    #[schemars(length(min = 1))]
    pub webhook_url: String,
    /// The key of the same webhook, sent as its bearer token.
    #[schemars(length(min = 1))]
    pub webhook_key: String,
}
