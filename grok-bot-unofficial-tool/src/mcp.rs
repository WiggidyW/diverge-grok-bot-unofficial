//! The MCP server: the tools by which an agent's webhook is set.
//!
//! An agent's turns go to one routine's webhook, and which one is not
//! an argument of anything: it is set here, by a call made as that
//! agent, and kept in the tool's database ([`store`](crate::store)).
//!
//! The rules:
//!
//! 1. Every tool acts on the calling agent's own webhook, and on no
//!    other's. The agent is the one the daemon attests in the call's
//!    `_meta`, never one the arguments name. A call attested as a
//!    client or as a tool, or not attested, is the tool's error result.
//! 2. [`SET`] takes `url` and `key`, neither empty, and keeps them as
//!    the agent's webhook, in place of any it had.
//! 3. [`GET`] answers whether the agent has a webhook, and its address.
//!    The key is never answered.
//! 4. [`CLEAR`] forgets the agent's webhook.
//! 5. A tool's own failure is its error result, for the caller to
//!    read; only a tool the server does not have is a protocol error.

use std::sync::Arc;

use diverge_sdk::daemon::key;
use diverge_sdk::shared::mcp::attestation::{self, Caller};
use rmcp::model::{CallToolRequestParams, CallToolResponse, CallToolResult, ContentBlock, Implementation, JsonObject, ListToolsResult, MetaObject, PaginatedRequestParams, ServerCapabilities, ServerConfig, Tool};
use rmcp::service::RequestContext;
use rmcp::{ErrorData, RoleServer, ServerHandler};
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{Value, json};

use crate::store::{self, Webhook};

/// What the server tells a client of itself.
const INSTRUCTIONS: &str = "Reaches a Grok Bot for the agent that calls it. Before an agent can be messaged, its webhook must be set with set_webhook: the address and the key of one routine's webhook, as the Grok Bot app shows them.";

/// Rule 2's tool.
const SET: &str = "set_webhook";
/// Rule 3's tool.
const GET: &str = "get_webhook";
/// Rule 4's tool.
const CLEAR: &str = "clear_webhook";

/// What [`SET`] takes.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct Set {
    /// The address the routine's webhook is called at, shown as
    /// "POST to" in the Grok Bot app.
    url: String,
    /// The key of the same webhook, sent as its bearer token.
    key: String,
}

/// What [`GET`] and [`CLEAR`] take: nothing.
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct Nothing {}

/// A tool, its input schema the JSON Schema of its parameters.
fn tool<P: JsonSchema>(name: &'static str, description: &'static str) -> Tool {
    let schema = serde_json::to_value(schemars::schema_for!(P)).ok().and_then(|value| value.as_object().cloned()).unwrap_or_default();
    Tool::new(name, description, Arc::new(schema))
}

/// The call's arguments, read as the tool's parameters.
fn parameters<P: serde::de::DeserializeOwned>(arguments: JsonObject) -> Result<P, String> {
    serde_json::from_value(Value::Object(arguments)).map_err(|error| format!("the parameters are not what this takes: {error}"))
}

/// Rule 1: the agent calling, from the call's `_meta`.
fn agent(meta: &MetaObject) -> Result<key::Agent, String> {
    match attestation::request(meta) {
        Ok(Caller::Agent(agent)) => Ok(agent.key),
        Ok(Caller::Client { .. }) => Err("a webhook is an agent's: call this as the agent, not as a client".to_string()),
        Ok(Caller::Tool { .. }) => Err("a webhook is an agent's: call this as the agent, not as a tool".to_string()),
        Err(error) => Err(error.to_string()),
    }
}

/// Rules 2 to 4.
async fn call(name: &str, agent: &key::Agent, arguments: JsonObject) -> Result<Value, String> {
    match name {
        SET => {
            let Set { url, key } = parameters(arguments)?;
            if url.is_empty() || key.is_empty() {
                return Err("`url` and `key` are both required, and neither is empty".to_string());
            }
            store::set(agent, &Webhook { url: url.clone(), key }).await?;
            Ok(json!({ "set": true, "url": url }))
        }
        GET => {
            let Nothing {} = parameters(arguments)?;
            Ok(match store::get(agent).await? {
                Some(webhook) => json!({ "set": true, "url": webhook.url }),
                None => json!({ "set": false }),
            })
        }
        CLEAR => {
            let Nothing {} = parameters(arguments)?;
            Ok(json!({ "cleared": store::clear(agent).await? }))
        }
        _ => Err(format!("no tool named {name}")),
    }
}

/// The server: nothing to hold.
#[derive(Debug, Clone, Default)]
pub struct Server;

impl ServerHandler for Server {
    fn get_info(&self) -> ServerConfig {
        ServerConfig::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(Implementation::new("grok-bot-unofficial-tool", env!("CARGO_PKG_VERSION")))
            .with_instructions(INSTRUCTIONS)
    }

    async fn list_tools(&self, _request: Option<PaginatedRequestParams>, _context: RequestContext<RoleServer>) -> Result<ListToolsResult, ErrorData> {
        Ok(ListToolsResult::with_all_items(vec![
            tool::<Set>(SET, "Set the calling agent's webhook: the address (\"POST to\") and the key of one routine's webhook, as the Grok Bot app shows them. Replaces any it had. The agent's messages go to this routine's Bot."),
            tool::<Nothing>(GET, "Whether the calling agent has a webhook set, and its address. The key is never answered."),
            tool::<Nothing>(CLEAR, "Forget the calling agent's webhook. Its messages are refused until one is set again."),
        ]))
    }

    /// Rules 1 and 5.
    async fn call_tool(&self, request: CallToolRequestParams, context: RequestContext<RoleServer>) -> Result<CallToolResponse, ErrorData> {
        if ![SET, GET, CLEAR].contains(&request.name.as_ref()) {
            return Err(ErrorData::invalid_params(format!("the tool has no tool named {}", request.name), None));
        }
        let outcome = async {
            // rmcp moves a request's `_meta` into the context before the
            // handler runs; the params' own is read only when the
            // context holds no attestation.
            let attested = |meta: &MetaObject| meta.0.contains_key(diverge_sdk::shared::mcp::KEY);
            let meta = match request.meta.as_ref() {
                Some(meta) if !attested(&context.meta.0) && attested(&meta.0) => &meta.0,
                _ => &context.meta.0,
            };
            let agent = agent(meta)?;
            call(&request.name, &agent, request.arguments.unwrap_or_default()).await
        }
        .await;
        Ok(CallToolResponse::from(match outcome {
            Ok(value) => CallToolResult::structured(value),
            Err(why) => CallToolResult::error(vec![ContentBlock::text(why)]),
        }))
    }
}
