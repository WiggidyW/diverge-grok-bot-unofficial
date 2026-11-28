//! The traffic log: a proxy beside the app that records everything the
//! app sends and receives.
//!
//! The webhook is one-way, so a run cannot yet carry the Bot's reply.
//! How the app itself hears the reply is learned from this log. It is
//! on always: there is no switch.
//!
//! The rules:
//!
//! 1. The proxy is `mitmdump`, on the loopback at [`PORT`], with the
//!    image's addon ([`ADDON`]), which writes one JSON line for each
//!    event to a new file beneath `/var/log/grok-bot-tool` at every
//!    start. A proxy that exits is started again after [`AGAIN`].
//! 2. The proxy ends TLS with a certificate authority it makes itself,
//!    in the running container, on its first start: no private key is
//!    in the image. The authority's certificate, [`CERTIFICATE`], is
//!    added to the store the app's Chromium side reads, [`STORE`]; the
//!    app's Node side is told of it by the app's wrapper.
//! 3. The hosts of [`UNREAD`] are passed through and not read: the
//!    vendor states that the link to the Bot's computer does not
//!    survive being re-signed.
//! 4. Only the app is sent through the proxy, by its wrapper. The
//!    tool's browser and the program's own calls are not, and are not
//!    logged.
//! 5. The log holds the sign-in token and every message, in the clear.
//!    No route serves it.
//! 6. [`ready`] is the proxy up and trusted, or why not, memoized. The
//!    app is not started without it.

use std::process::Stdio;
use std::time::Duration;

use tokio::process::Command;
use tokio::sync::OnceCell;

/// The loopback port the proxy listens on.
pub const PORT: u16 = 8082;

/// The proxy's certificate authority, as the proxy writes it.
pub const CERTIFICATE: &str = "/root/.mitmproxy/mitmproxy-ca-cert.pem";

/// The addon that writes the log, in the image.
const ADDON: &str = "/usr/local/share/grok-bot-tool/traffic.py";

/// The certificate store the app's Chromium side reads.
const STORE: &str = "sql:/root/.pki/nssdb";

/// The hosts passed through unread (rule 3), as a pattern over
/// `host:port`.
const UNREAD: &str = r"(^|\.)cursorvm\.com:\d+$";

/// How long after the proxy exits it is started again.
const AGAIN: Duration = Duration::from_secs(3);

/// Whether the proxy came up and is trusted, or why not.
static READY: OnceCell<Result<(), String>> = OnceCell::const_new();

/// Rule 6.
pub async fn ready() -> &'static Result<(), String> {
    READY.get_or_init(start).await
}

/// Rules 1 and 2.
async fn start() -> Result<(), String> {
    // Rule 1: the proxy, for the container's life.
    tokio::spawn(async {
        loop {
            let proxy = Command::new("mitmdump")
                .args(["--quiet", "--listen-host", "127.0.0.1", "--listen-port", &PORT.to_string(), "--scripts", ADDON, "--ignore-hosts", UNREAD])
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::inherit())
                .spawn();
            match proxy {
                Ok(mut proxy) => match proxy.wait().await {
                    Ok(exit) => eprintln!("the traffic proxy exited: {exit}"),
                    Err(error) => eprintln!("the traffic proxy could not be waited on: {error}"),
                },
                Err(error) => eprintln!("the traffic proxy could not be started: {error}"),
            }
            tokio::time::sleep(AGAIN).await;
        }
    });
    let mut tries = 0;
    while tokio::net::TcpStream::connect(("127.0.0.1", PORT)).await.is_err() || tokio::fs::metadata(CERTIFICATE).await.is_err() {
        tries += 1;
        if tries > 300 {
            return Err("the traffic proxy did not come up".to_string());
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    // Rule 2: the store, made where there is none, and the certificate
    // in it, trusted to sign servers.
    tokio::fs::create_dir_all("/root/.pki/nssdb").await.map_err(|error| format!("the certificate store could not be made: {error}"))?;
    let _ = Command::new("certutil").args(["-d", STORE, "-N", "--empty-password"]).stdin(Stdio::null()).output().await;
    let added = Command::new("certutil")
        .args(["-d", STORE, "-A", "-t", "C,,", "-n", "grok-bot-tool-traffic", "-i", CERTIFICATE])
        .stdin(Stdio::null())
        .output()
        .await
        .map_err(|error| format!("certutil could not be run: {error}"))?;
    if !added.status.success() {
        return Err(format!("the proxy's certificate could not be trusted: {}", String::from_utf8_lossy(&added.stderr)));
    }
    Ok(())
}
