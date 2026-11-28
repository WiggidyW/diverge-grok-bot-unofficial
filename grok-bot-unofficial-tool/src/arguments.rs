//! What the tool takes as its arguments: which account it is.

use serde::{Deserialize, Serialize};

/// The arguments a Grok Bot tool is made with.
///
/// One tool is one Grok Bot account: the app signs in once, so each
/// account has a container of its own, and every agent made with the
/// same `account` shares it. Which Bot of the account an agent's turns
/// reach is said by the webhook set for that agent, through the tool's
/// MCP server. Any other
/// member is refused, so a misspelled setting is never silently
/// ignored.
///
/// # How it runs
///
/// As a dependency tool of the `grok-bot-unofficial-agent` agent: an
/// eager tool whose scope is `template`, so one container serves every
/// agent that declares it with the same account.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Arguments {
    /// A name for the Grok Bot account, chosen by whoever makes the
    /// agents: any text that is not empty. It is a label and not a
    /// credential, and nothing checks it against the vendor. It tells
    /// one account's tool from another's, and nothing else is done
    /// with it yet.
    #[schemars(length(min = 1))]
    pub account: String,
}
