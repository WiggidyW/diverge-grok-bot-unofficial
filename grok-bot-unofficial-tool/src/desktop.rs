//! The desktop: one virtual display in the container, holding the
//! Grok Bot app's window and a browser's, which the frontend streams.
//!
//! The person signs in to the app through the frontend. Sign-in opens
//! a web page, and the page's last step hands a link back to the app,
//! which only works where the browser and the app are on one machine.
//! So both run here.
//!
//! The rules:
//!
//! 1. The display is `Xvnc`, display [`DISPLAY`], serving VNC on the
//!    loopback at [`VNC_PORT`] with no password: the frontend is the
//!    only way to it, and the daemon's stamp is its gate. It is shared,
//!    so every open page sees the one desktop.
//! 2. A session bus, a window manager (`openbox`) and a taskbar
//!    (`tint2`) are started on it, then the browser.
//! 3. The browser is Debian's Chromium behind the image's
//!    `tool-browser` wrapper, and is the default for web addresses, so
//!    a link the app opens lands in it.
//! 4. The app is started once its install has ended well. Its launcher
//!    and the link schemes it claims are read from the `.desktop` file
//!    its own package installed, and never guessed. The vendor's files
//!    are not changed: a wrapper, [`APP`], starts the vendor's program
//!    with the flags a root container needs, and a `.desktop` file of
//!    ours claims the same schemes for that wrapper, so the link a
//!    sign-in page hands back reaches the app.
//! 5. An app that exits is started again after [`AGAIN`].
//! 6. What has happened so far is the [`Status`], which the frontend
//!    shows.

use std::process::Stdio;
use std::sync::RwLock;
use std::time::Duration;

use serde::Serialize;
use tokio::process::Command;

use crate::install;

/// The display's name.
pub const DISPLAY: &str = ":1";

/// The loopback port the display serves VNC on.
pub const VNC_PORT: u16 = 5901;

/// The wrapper the app is started through (rule 4).
const APP: &str = "/usr/local/bin/tool-app";

/// The wrapper the browser is started through (rule 3), in the image.
const BROWSER: &str = "/usr/local/bin/tool-browser";

/// The flags a Chromium program needs as root in a container: no
/// sandbox of its own, and no reliance on a large `/dev/shm`.
const FLAGS: &str = "--no-sandbox --disable-dev-shm-usage";

/// How long after the app exits it is started again (rule 5).
const AGAIN: Duration = Duration::from_secs(3);

/// The desktop so far (rule 6).
#[derive(Debug, Clone, Serialize)]
pub struct Status {
    /// Whether the display is up, so the page has something to show.
    pub display: bool,
    /// `starting`, `installing`, `running`, `restarting` or `failed`.
    pub state: &'static str,
    /// What there is to say of the state, where anything.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

/// The status, as last set.
static STATUS: RwLock<Status> = RwLock::new(Status { display: false, state: "starting", detail: None });

/// The status, as last set.
pub fn status() -> Status {
    STATUS.read().map(|status| status.clone()).unwrap_or(Status { display: false, state: "failed", detail: Some("the status is lost".to_string()) })
}

/// Set the state, and its detail.
fn state(state: &'static str, detail: Option<String>) {
    if let Ok(mut status) = STATUS.write() {
        status.state = state;
        status.detail = detail;
    }
}

/// A command on the display, its output discarded.
fn command(program: &str, bus: &str) -> Command {
    let mut command = Command::new(program);
    command.env("DISPLAY", DISPLAY).env("DBUS_SESSION_BUS_ADDRESS", bus).stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null());
    command
}

/// The desktop, for the container's life: rules 1 to 5. Returns only
/// on a failure nothing here recovers from, which the status states.
pub async fn run() {
    if let Err(why) = up().await {
        eprintln!("the desktop failed: {why}");
        state("failed", Some(why));
    }
}

/// Rules 1 to 5.
async fn up() -> Result<(), String> {
    // Rule 1.
    Command::new("Xvnc")
        .args([DISPLAY, "-geometry", "1280x800", "-depth", "24", "-SecurityTypes", "None", "-localhost", "-AlwaysShared", "-rfbport", &VNC_PORT.to_string()])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|error| format!("Xvnc could not be started: {error}"))?;
    let mut tries = 0;
    while tokio::net::TcpStream::connect(("127.0.0.1", VNC_PORT)).await.is_err() {
        tries += 1;
        if tries > 100 {
            return Err("the display did not come up".to_string());
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    if let Ok(mut status) = STATUS.write() {
        status.display = true;
    }
    // Rule 2.
    let bus = Command::new("dbus-daemon").args(["--session", "--fork", "--print-address"]).output().await.map_err(|error| format!("the session bus could not be started: {error}"))?;
    let bus = String::from_utf8_lossy(&bus.stdout).trim().to_string();
    command("openbox", &bus).spawn().map_err(|error| format!("openbox could not be started: {error}"))?;
    command("tint2", &bus).spawn().map_err(|error| format!("tint2 could not be started: {error}"))?;
    // Rule 3.
    defaults(&[]).await?;
    command(BROWSER, &bus).spawn().map_err(|error| format!("the browser could not be started: {error}"))?;
    // Rule 4.
    state("installing", None);
    let installed = match install::installed().await {
        Ok(installed) => installed,
        Err(why) => return Err(format!("Grok Bot was not installed: {why}")),
    };
    let launcher = launcher(&installed.package).await?;
    write(APP, &format!("#!/bin/sh\nexec \"{}\" {FLAGS} \"$@\"\n", launcher.program), true).await?;
    let schemes: Vec<String> = launcher.schemes.iter().map(|scheme| format!("x-scheme-handler/{scheme}")).collect();
    write(
        "/root/.local/share/applications/tool-app.desktop",
        &format!("[Desktop Entry]\nType=Application\nName=Grok Bot\nExec={APP} %U\nTerminal=false\nMimeType={};\n", schemes.join(";")),
        false,
    )
    .await?;
    defaults(&schemes).await?;
    // Rules 4 and 5.
    loop {
        let mut app = command(APP, &bus).spawn().map_err(|error| format!("Grok Bot could not be started: {error}"))?;
        state("running", Some(format!("Grok Bot {}", installed.version)));
        let exit = app.wait().await.map_err(|error| format!("Grok Bot could not be waited on: {error}"))?;
        eprintln!("Grok Bot exited: {exit}");
        state("restarting", Some(format!("Grok Bot exited ({exit}) and is being started again")));
        tokio::time::sleep(AGAIN).await;
    }
}

/// What the app's own `.desktop` file says of it.
struct Launcher {
    /// The program its `Exec` line starts.
    program: String,
    /// The link schemes it claims, each without `x-scheme-handler/`.
    schemes: Vec<String>,
}

/// Rule 4's reading: the launcher of the package named, from the
/// `.desktop` file the package installed.
async fn launcher(package: &str) -> Result<Launcher, String> {
    let listed = Command::new("dpkg").args(["-L", package]).output().await.map_err(|error| format!("dpkg could not be run: {error}"))?;
    let listed = String::from_utf8_lossy(&listed.stdout).to_string();
    let path = listed.lines().find(|line| line.starts_with("/usr/share/applications/") && line.ends_with(".desktop")).ok_or_else(|| format!("the package `{package}` installed no launcher"))?;
    let text = tokio::fs::read_to_string(path).await.map_err(|error| format!("{path} could not be read: {error}"))?;
    let exec = text.lines().find_map(|line| line.strip_prefix("Exec=")).ok_or_else(|| format!("{path} names no program"))?;
    // The program is the line's first word, or its first quoted run.
    let program = match exec.strip_prefix('"') {
        Some(quoted) => quoted.split('"').next().unwrap_or_default(),
        None => exec.split_whitespace().next().unwrap_or_default(),
    };
    if program.is_empty() {
        return Err(format!("{path} names no program"));
    }
    let schemes = text
        .lines()
        .find_map(|line| line.strip_prefix("MimeType="))
        .map(|types| types.split(';').filter_map(|kind| kind.strip_prefix("x-scheme-handler/")).map(str::to_string).collect())
        .unwrap_or_default();
    Ok(Launcher { program: program.to_string(), schemes })
}

/// The defaults: the browser for web addresses and pages (rule 3), and
/// the app's wrapper for each of `schemes` (rule 4).
async fn defaults(schemes: &[String]) -> Result<(), String> {
    let mut text = "[Default Applications]\n".to_string();
    for kind in ["x-scheme-handler/http", "x-scheme-handler/https", "text/html"] {
        text.push_str(&format!("{kind}=tool-browser.desktop\n"));
    }
    for scheme in schemes {
        text.push_str(&format!("{scheme}=tool-app.desktop\n"));
    }
    write("/root/.config/mimeapps.list", &text, false).await
}

/// A file written whole, its directory made; executable where asked.
async fn write(path: &str, text: &str, executable: bool) -> Result<(), String> {
    if let Some(parent) = std::path::Path::new(path).parent() {
        tokio::fs::create_dir_all(parent).await.map_err(|error| format!("{} could not be made: {error}", parent.display()))?;
    }
    tokio::fs::write(path, text).await.map_err(|error| format!("{path} could not be written: {error}"))?;
    // The image is Linux; the gate only lets the crate be checked
    // elsewhere.
    #[cfg(unix)]
    if executable {
        use std::os::unix::fs::PermissionsExt as _;
        tokio::fs::set_permissions(path, std::fs::Permissions::from_mode(0o755)).await.map_err(|error| format!("{path} could not be made executable: {error}"))?;
    }
    #[cfg(not(unix))]
    let _ = executable;
    Ok(())
}
