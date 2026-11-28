"""The traffic log: a mitmproxy addon that records everything the Grok
Bot app sends and receives, one JSON object per line.

Every line has `ts` (seconds since the epoch, with fractions), `flow`
(an id joining the lines of one exchange) and `event`:

- `connect`: a tunnel the app asked for, by host and port. A host the
  proxy passes through unread shows this line and no other.
- `request`: method, URL, HTTP version, headers, body.
- `response_head`: status, headers.
- `response_chunk`: one piece of a response body, as it passed.
- `response_end`: the body's total bytes, and the seconds it took.
- `ws_message`: one WebSocket message, and which side sent it.
- `error`: an exchange that failed, and why.
- `tls_refused`: the app refused the proxy's certificate for a host,
  which is what certificate pinning looks like.

A body is `{"text": …}` where it is valid UTF-8, and `{"base64": …}`
otherwise. Nothing is redacted: the log holds the sign-in token and
every message. A response's body is logged piece by piece and passed on
at once, never buffered, since the time between the pieces is what the
log is for.
"""

import base64
import json
import os
import threading
import time

DIRECTORY = "/var/log/grok-bot-tool"


def body(data):
    """A body, as text where it is text, and as base64 otherwise."""
    if not data:
        return {"text": ""}
    try:
        return {"text": data.decode("utf-8")}
    except UnicodeDecodeError:
        return {"base64": base64.b64encode(data).decode("ascii")}


def headers(message):
    """Headers as pairs, in their order; a name may repeat."""
    return [[name, value] for name, value in message.headers.items(multi=True)]


class Traffic:
    def __init__(self):
        os.makedirs(DIRECTORY, exist_ok=True)
        name = time.strftime("traffic-%Y%m%dT%H%M%SZ.jsonl", time.gmtime())
        self.file = open(os.path.join(DIRECTORY, name), "a", encoding="utf-8")
        self.lock = threading.Lock()

    def write(self, flow, event, **fields):
        line = json.dumps({"ts": time.time(), "flow": flow, "event": event, **fields}, ensure_ascii=False)
        with self.lock:
            self.file.write(line + "\n")
            self.file.flush()

    def http_connect(self, flow):
        self.write(flow.id, "connect", host=flow.request.host, port=flow.request.port)

    def request(self, flow):
        request = flow.request
        self.write(
            flow.id,
            "request",
            method=request.method,
            url=request.pretty_url,
            http_version=request.http_version,
            headers=headers(request),
            body=body(request.raw_content),
        )

    def responseheaders(self, flow):
        response = flow.response
        self.write(flow.id, "response_head", status=response.status_code, http_version=response.http_version, headers=headers(response))
        started = time.time()
        total = [0]

        def passing(chunk):
            if chunk:
                total[0] += len(chunk)
                self.write(flow.id, "response_chunk", body=body(chunk))
            else:
                self.write(flow.id, "response_end", bytes=total[0], seconds=time.time() - started)
            return chunk

        response.stream = passing

    def websocket_message(self, flow):
        message = flow.websocket.messages[-1]
        self.write(
            flow.id,
            "ws_message",
            url=flow.request.pretty_url,
            sender="app" if message.from_client else "server",
            is_text=message.is_text,
            body=body(message.content),
        )

    def error(self, flow):
        self.write(flow.id, "error", url=flow.request.pretty_url, error=str(flow.error))

    def tls_failed_client(self, data):
        connection = data.conn
        self.write(None, "tls_refused", host=getattr(connection, "sni", None), error=str(getattr(connection, "error", None)))


addons = [Traffic()]
