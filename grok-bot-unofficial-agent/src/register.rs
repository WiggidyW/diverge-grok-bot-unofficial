//! `POST /register`: the arguments, once; the tool it depends on and
//! the one thing it asks of the daemon, answered.

use std::sync::OnceLock;

use axum::Json;
use axum::http::StatusCode;
use diverge_sdk::container_proxy::inside::register;
use diverge_sdk::container_proxy::inside::register::response::Response;
use diverge_sdk::daemon::reference;
use diverge_sdk::shared::canonical;
use diverge_sdk::shared::containers::dependencies::Template;
use diverge_sdk::shared::containers::request::Image;
use diverge_sdk::shared::permission::{Grant, dependency_tools};
use serde_json::{Value, json};
use sha2::{Digest as _, Sha256};

use crate::arguments::Arguments;

/// The agent's tool, as the daemon's requests name it, kept at the one
/// registration.
static TOOL: OnceLock<reference::Dependency> = OnceLock::new();

/// The agent's tool; `None` before the registration, which a run is
/// refused for.
pub fn tool() -> Option<&'static reference::Dependency> {
    TOOL.get()
}

/// The dependency the agent declares: the tool's image, with nothing
/// beyond what the image needs, no mount, and the account as its
/// arguments, so every agent naming the same image and the same
/// account declares the same tool.
fn dependency(image: Image, account: &str) -> Template {
    Template { image, extra_memory: 0, extra_disk: 0, arguments: json!({ "account": account }), fuse_file_mounts: Vec::new(), fuse_directory_mounts: Vec::new() }
}

/// A dependency template's id, as the daemon names it: the lowercase
/// hexadecimal SHA-256 of the canonical bytes of the template as it is
/// hashed, its image's references absent.
fn id(template: &Template) -> Result<String, serde_json::Error> {
    Ok(hex::encode(Sha256::digest(canonical::bytes(&template.hashed())?)))
}

/// What the agent asks of the daemon: `api` over the dependency tools
/// attached to it, which is its tool and nothing else.
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
///    account, are `400`.
/// 2. The agent's tool is the dependency tool of the template it
///    declares, owned by that template, since the tool's image is of
///    scope `template`: one tool for every agent that declares it
///    with the same account. It
///    is kept by that name: the dependency template's id.
/// 3. A second registration, whatever it carries, is `409`.
/// 4. Otherwise `200`, declaring the one dependency and
///    [`permissions`].
pub async fn register(Json(request): Json<register::request::Request>) -> Result<(StatusCode, Json<Response>), (StatusCode, Json<Value>)> {
    // Rule 1.
    let arguments = serde_json::from_value::<Arguments>(request.arguments).map_err(|error| refused("arguments", &error.to_string()))?;
    if arguments.account.is_empty() {
        return Err(refused("arguments", "`account` is required, and is not empty"));
    }
    // Rule 2.
    let template = dependency(arguments.tool, &arguments.account);
    let id = id(&template).map_err(|error| refused("arguments", &error.to_string()))?;
    let tool = reference::Dependency { template: id, owner: reference::Owner::Template };
    // Rules 3 and 4.
    if TOOL.set(tool).is_err() {
        return Err((StatusCode::CONFLICT, Json(json!({ "kind": "registered", "error": "the agent is registered, and it never changes" }))));
    }
    Ok((StatusCode::OK, Json(Response { dependencies: vec![template], permissions: permissions() })))
}
