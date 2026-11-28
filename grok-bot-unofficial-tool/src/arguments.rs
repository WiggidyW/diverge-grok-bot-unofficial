//! What the tool takes as its arguments: nothing.

use serde::{Deserialize, Serialize};

/// The arguments a Grok Bot tool is made with.
///
/// None: the tool is every agent's that declares it, and which Bot a
/// turn reaches is said by the turn itself, which names the webhook.
/// `{}` is the one value; any member is refused, so a misspelled
/// setting is never silently ignored.
///
/// # How it runs
///
/// As a dependency tool of the `grok-bot-unofficial-agent` agent: an
/// eager tool whose scope is `template`, so one container serves every
/// agent that declares it.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Arguments {}
