//! The one install: the Grok Bot desktop app, fetched at startup.
//!
//! The image ships WITHOUT the app. The app is the vendor's software,
//! and this image does not redistribute it: the program fetches it from
//! the vendor the moment it starts, so a starting container needs the
//! vendor's download hosts and the Debian mirrors reachable.
//!
//! The rules:
//!
//! 1. The vendor's release feed names the current release for this
//!    machine's architecture; its `debUrl` is the package fetched. No
//!    version is pinned here: the vendor disables old builds, so the
//!    current one is the only one sure to run.
//! 2. The package is installed with `apt-get`, which resolves what the
//!    package itself says it depends on.
//! 3. The outcome is memoized and final: one install per process, and
//!    no retry. A container whose install failed is replaced.
//! 4. Nothing waits on the install yet. The webhook needs no app; the
//!    app is here for the frontend, which is not built.

use tokio::io::AsyncWriteExt as _;
use tokio::process::Command;
use tokio::sync::OnceCell;

/// Where the package is written before it is installed.
const PACKAGE: &str = "/tmp/grok-bot.deb";

/// The install's memoized outcome: the version installed, or why not.
static INSTALL: OnceCell<Result<String, String>> = OnceCell::const_new();

/// The install's outcome, starting the install if nobody has (rule 3).
pub async fn installed() -> &'static Result<String, String> {
    INSTALL.get_or_init(install).await
}

/// The feed's platform name for this machine, or none for an
/// architecture the vendor publishes no Linux build for.
fn platform() -> Option<&'static str> {
    match std::env::consts::ARCH {
        "x86_64" => Some("linux-x64"),
        "aarch64" => Some("linux-arm64"),
        _ => None,
    }
}

/// Rules 1 and 2.
async fn install() -> Result<String, String> {
    let platform = platform().ok_or_else(|| format!("no Grok Bot build exists for {}", std::env::consts::ARCH))?;
    let feed = format!("https://api2.cursor.sh/updates/api/download/stable/{platform}/sand");
    let release: serde_json::Value = reqwest::get(&feed)
        .await
        .and_then(reqwest::Response::error_for_status)
        .map_err(|error| format!("the release feed could not be read: {error}"))?
        .json()
        .await
        .map_err(|error| format!("the release feed did not parse: {error}"))?;
    let url = release.get("debUrl").and_then(serde_json::Value::as_str).ok_or("the release feed names no `debUrl`")?;
    let version = release.get("version").and_then(serde_json::Value::as_str).unwrap_or("unknown").to_string();
    let mut download = reqwest::get(url).await.and_then(reqwest::Response::error_for_status).map_err(|error| format!("the package could not be fetched: {error}"))?;
    let mut file = tokio::fs::File::create(PACKAGE).await.map_err(|error| format!("{PACKAGE} could not be made: {error}"))?;
    while let Some(piece) = download.chunk().await.map_err(|error| format!("the package's download broke off: {error}"))? {
        file.write_all(&piece).await.map_err(|error| format!("{PACKAGE} could not be written: {error}"))?;
    }
    file.flush().await.map_err(|error| format!("{PACKAGE} could not be written: {error}"))?;
    drop(file);
    apt(&["update"]).await?;
    apt(&["install", "-y", "--no-install-recommends", PACKAGE]).await?;
    let _ = tokio::fs::remove_file(PACKAGE).await;
    Ok(version)
}

/// One `apt-get` run, its failure rendered as prose.
async fn apt(arguments: &[&str]) -> Result<(), String> {
    let output = Command::new("apt-get")
        .args(arguments)
        .env("DEBIAN_FRONTEND", "noninteractive")
        .output()
        .await
        .map_err(|error| format!("apt-get could not be spawned: {error}"))?;
    if output.status.success() {
        Ok(())
    } else {
        Err(format!("apt-get {} failed ({}): {}", arguments[0], output.status, String::from_utf8_lossy(&output.stderr)))
    }
}
