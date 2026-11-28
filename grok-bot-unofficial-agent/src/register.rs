//! `POST /register`: the arguments and who the agent is, once; the
//! tool it depends on and the one thing it asks of the daemon,
//! answered.

use std::sync::OnceLock;

use axum::Json;
use axum::http::StatusCode;
use diverge_sdk::container_proxy::inside::register;
use diverge_sdk::container_proxy::inside::register::response::Response;
use diverge_sdk::daemon::reference;
use diverge_sdk::shared::canonical;
use diverge_sdk::shared::containers::dependencies::Template;
use diverge_sdk::shared::containers::request::Which;
use diverge_sdk::shared::permission::{Grant, dependency_tools};
use serde_json::{Value, json};
use sha2::{Digest as _, Sha256};

use crate::arguments::Arguments;

/// The agent's own tool, as the daemon's requests name it, kept at the
/// one registration.
static TOOL: OnceLock<reference::Dependency> = OnceLock::new();

/// The agent's own tool; `None` before the registration, which a run
/// is refused for.
pub fn tool() -> Option<&'static reference::Dependency> {
    TOOL.get()
}

/// The dependency the agent declares: the tool's image, with nothing
/// beyond what the image needs, no mount, and the webhook as its
/// arguments.
fn dependency(arguments: Arguments) -> Template {
    Template {
        image: arguments.tool,
        extra_memory: 0,
        extra_disk: 0,
        arguments: json!({ "webhook_url": arguments.webhook_url, "webhook_key": arguments.webhook_key }),
        fuse_file_mounts: Vec::new(),
        fuse_directory_mounts: Vec::new(),
    }
}

/// A dependency template's id, as the daemon names it: the lowercase
/// hexadecimal SHA-256 of the canonical bytes of the template as it is
/// hashed, its image's references absent.
fn id(template: &Template) -> Result<String, serde_json::Error> {
    Ok(hex::encode(Sha256::digest(canonical::bytes(&template.hashed())?)))
}

/// What the agent asks of the daemon: `api` over the dependency tools
/// attached to it, which is its own tool and nothing else.
fn permissions() -> Vec<Grant> {
    vec![Grant::DependencyTools(dependency_tools::Permission::Over { actions: vec![dependency_tools::Over::Api], within: dependency_tools::Within::Attached })]
}

/// A `400`, with its kind and why.
fn refused(kind: &str, error: &str) -> (StatusCode, Json<Value>) {
    (StatusCode::BAD_REQUEST, Json(json!({ "kind": kind, "error": error })))
}

/// `POST /register`. The rules:
///
/// 1. Arguments that are not [`Arguments`], or that name an empty
///    address or an empty key, are `400`.
/// 2. The registration says which agent of its template this is, by
///    its index; one that says anything else is `400`.
/// 3. The agent's tool is the dependency tool of the template it
///    declares, owned by this agent, since the tool's image is of
///    scope `instance`. It is kept by that name: the dependency
///    template's id, and the agent's own template and index.
/// 4. A second registration, whatever it carries, is `409`.
/// 5. Otherwise `200`, declaring the one dependency and
///    [`permissions`].
pub async fn register(Json(request): Json<register::request::Request>) -> Result<(StatusCode, Json<Response>), (StatusCode, Json<Value>)> {
    // Rule 1.
    let arguments = serde_json::from_value::<Arguments>(request.arguments).map_err(|error| refused("arguments", &error.to_string()))?;
    if arguments.webhook_url.is_empty() || arguments.webhook_key.is_empty() {
        return Err(refused("arguments", "`webhook_url` and `webhook_key` are both required, and neither is empty"));
    }
    // Rule 2.
    let Which::Index(index) = request.which else {
        return Err(refused("which", "the registration names no agent's index"));
    };
    // Rule 3.
    let template = dependency(arguments);
    let id = id(&template).map_err(|error| refused("arguments", &error.to_string()))?;
    let tool = reference::Dependency { template: id, owner: reference::Owner::Agent(reference::Agent::TemplateIndex { template: request.template, index }) };
    // Rules 4 and 5.
    if TOOL.set(tool).is_err() {
        return Err((StatusCode::CONFLICT, Json(json!({ "kind": "registered", "error": "the agent is registered, and it never changes" }))));
    }
    Ok((StatusCode::OK, Json(Response { dependencies: vec![template], permissions: permissions() })))
}
