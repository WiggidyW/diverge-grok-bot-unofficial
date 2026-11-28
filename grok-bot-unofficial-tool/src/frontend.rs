//! The frontend, beneath `/frontend`: the tool's desktop, as a page.
//!
//! The page is a remote desktop. It shows the one display the
//! [`desktop`](crate::desktop) runs, with the Grok Bot app's window and
//! a browser's on it, and carries the person's keys and pointer to it.
//!
//! The rules:
//!
//! 1. Every request is a person's: one the daemon stamped as a client.
//!    One with no stamp, or one stamped as an agent or as a tool, is
//!    `403`. See [`person`].
//! 2. `GET /frontend` sends the page on to `/frontend/`, where it is
//!    served: one HTML file.
//! 3. `GET /frontend/novnc/…` is the noVNC web client's files, as the
//!    Debian package installed them at [`NOVNC`].
//! 4. `GET /frontend/status` is the desktop's
//!    [`Status`](crate::desktop::Status), as JSON.
//! 5. `GET /frontend/vnc` is answered `101`, and from then the socket
//!    is the display's VNC connection: each binary message the page
//!    sends is written to it, and what it sends comes back as binary
//!    messages, until either side ends. With the display down the
//!    socket is closed with nothing sent.

use axum::Router;
use axum::extract::Request;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::routing::get;
use futures_util::{SinkExt as _, StreamExt as _};
use serde_json::json;
use tokio::io::{AsyncReadExt as _, AsyncWriteExt as _};
use tower_http::services::ServeDir;

use crate::caller::person;
use crate::desktop;

/// The page, whole.
const PAGE: &str = include_str!("page.html");

/// Where the Debian package keeps the noVNC web client.
const NOVNC: &str = "/usr/share/novnc";

/// The routes beneath `/frontend`.
pub fn routes() -> Router {
    Router::new()
        .route("/frontend", get(|| async { Redirect::permanent("/frontend/") }))
        .route("/frontend/", get(|| async { Html(PAGE) }))
        .route("/frontend/status", get(|| async { axum::Json(desktop::status()) }))
        .route("/frontend/vnc", get(socket))
        .nest_service("/frontend/novnc", ServeDir::new(NOVNC))
        .layer(axum::middleware::from_fn(guard))
}

/// Rule 1.
async fn guard(request: Request, next: Next) -> Response {
    match person(request.headers()) {
        Ok(()) => next.run(request).await,
        Err(why) => (StatusCode::FORBIDDEN, axum::Json(json!({ "error": why }))).into_response(),
    }
}

/// Rule 5.
async fn socket(upgrade: WebSocketUpgrade) -> Response {
    upgrade.protocols(["binary"]).on_upgrade(bridge)
}

/// Rule 5: the socket and the display's VNC connection, joined.
async fn bridge(socket: WebSocket) {
    let Ok(display) = tokio::net::TcpStream::connect(("127.0.0.1", desktop::VNC_PORT)).await else {
        return;
    };
    let (mut from_display, mut to_display) = display.into_split();
    let (mut to_page, mut from_page) = socket.split();
    let up = async {
        while let Some(Ok(message)) = from_page.next().await {
            match message {
                Message::Binary(bytes) => {
                    if to_display.write_all(&bytes).await.is_err() {
                        break;
                    }
                }
                Message::Close(_) => break,
                _ => {}
            }
        }
    };
    let down = async {
        let mut buffer = vec![0u8; 64 * 1024];
        loop {
            match from_display.read(&mut buffer).await {
                Ok(0) | Err(_) => break,
                Ok(read) => {
                    if to_page.send(Message::Binary(buffer[..read].to_vec().into())).await.is_err() {
                        break;
                    }
                }
            }
        }
    };
    // Either side ending ends both.
    tokio::select! {
        () = up => {}
        () = down => {}
    }
}
