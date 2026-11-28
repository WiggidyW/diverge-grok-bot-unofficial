//! The webhooks, kept: for each agent, the routine's webhook its turns
//! are sent to.
//!
//! They are in the tool's `template` database, which is this tool's
//! across every agent that declares it with the same account, and
//! outlives the container. A tool whose scope is `template` has no
//! `instance` database.
//!
//! An agent is named by its template and its index, which name it once
//! and for all. A key is kept in the clear: it is a credential the tool
//! has to send as it is.

use std::sync::OnceLock;

use diverge_sdk::container_proxy::inside::{Client, Database};
use diverge_sdk::daemon::key;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tokio::sync::OnceCell;

/// The table, made on first use: one row for each agent.
const CREATE: &str = "CREATE TABLE IF NOT EXISTS grok_bot_webhooks (template text NOT NULL, index bigint NOT NULL, url text NOT NULL, key text NOT NULL, PRIMARY KEY (template, index))";

/// One routine's webhook, as the Grok Bot app shows it.
#[derive(Debug, Clone)]
pub struct Webhook {
    /// The address it is called at.
    pub url: String,
    /// Its key, sent as the bearer token.
    pub key: String,
}

/// The proxy beside the program, which the database is reached through.
fn client() -> &'static Client {
    static CLIENT: OnceLock<Client> = OnceLock::new();
    CLIENT.get_or_init(Client::new)
}

/// The pool, made and the table ensured on first use. A failure leaves
/// it unmade, so the next use tries again.
async fn pool() -> Result<&'static PgPool, String> {
    static POOL: OnceCell<PgPool> = OnceCell::const_new();
    POOL.get_or_try_init(|| async {
        let pool = PgPoolOptions::new().max_connections(2).connect(&client().postgres_url(Database::Template)).await.map_err(|error| error.to_string())?;
        sqlx::query(CREATE).execute(&pool).await.map_err(|error| error.to_string())?;
        Ok::<_, String>(pool)
    })
    .await
}

/// The agent's webhook, or none where none is set.
pub async fn get(agent: &key::Agent) -> Result<Option<Webhook>, String> {
    let row: Option<(String, String)> = sqlx::query_as("SELECT url, key FROM grok_bot_webhooks WHERE template = $1 AND index = $2")
        .bind(&agent.template)
        .bind(agent.index as i64)
        .fetch_optional(pool().await?)
        .await
        .map_err(|error| error.to_string())?;
    Ok(row.map(|(url, key)| Webhook { url, key }))
}

/// The agent's webhook, set: the one it had, if any, is replaced.
pub async fn set(agent: &key::Agent, webhook: &Webhook) -> Result<(), String> {
    sqlx::query("INSERT INTO grok_bot_webhooks (template, index, url, key) VALUES ($1, $2, $3, $4) ON CONFLICT (template, index) DO UPDATE SET url = EXCLUDED.url, key = EXCLUDED.key")
        .bind(&agent.template)
        .bind(agent.index as i64)
        .bind(&webhook.url)
        .bind(&webhook.key)
        .execute(pool().await?)
        .await
        .map_err(|error| error.to_string())?;
    Ok(())
}

/// The agent's webhook, forgotten; whether it had one.
pub async fn clear(agent: &key::Agent) -> Result<bool, String> {
    let done = sqlx::query("DELETE FROM grok_bot_webhooks WHERE template = $1 AND index = $2")
        .bind(&agent.template)
        .bind(agent.index as i64)
        .execute(pool().await?)
        .await
        .map_err(|error| error.to_string())?;
    Ok(done.rows_affected() > 0)
}
