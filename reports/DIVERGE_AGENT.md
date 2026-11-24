# The Diverge Agent API

What a Diverge **agent container** has to serve so that the Diverge container proxy can drive it.

Sources, in the `objectiveai` repository:

- `diverge-container-proxy/` is the proxy that runs beside the agent inside the container. It is the only caller of the agent's API. See `src/agent/` and `src/program/`.
- `diverge-sdk-rs/src/container_proxy/inside/` is the contract itself: request and response types, ports, and the rules. See `agent/`, `register/` and `secrets/`.
- `diverge-sdk-rs/src/shared/containers/` holds the shared shapes: `enqueue/` (messages), `structured_decisions/`, `secrets/` and `dependencies/`.
- `diverge-sdk-rs/src/provider/endpoints/containers/agents/run/server/response/` defines the chunk types the agent streams back.
- `diverge-agents/` has existing agent implementations to compare against (`cc`, `codex`, `hermes`, `lone`, `openrouter`, `python`, `Objective-Superintelligence`).

---

## 1. Topology

```
 provider's server ──WebSocket :14979──► diverge-container-proxy ──HTTP 127.0.0.1:$PORT──► AGENT PROGRAM
                                            ▲         ▲                                       │
                                            │         └── :81  pgwire (caller's Postgres) ◄───┤
                                            └──────────── :80  /mcp (caller's tools)     ◄────┘
```

There are two directions on the loopback:

| Direction | Who listens | Address | What it is |
|---|---|---|---|
| proxy → agent | **the agent** | `http://127.0.0.1:$PORT` (`PORT` env var, default **8080**) | The agent's HTTP API: the five calls below. |
| agent → proxy | the proxy | `http://127.0.0.1:80/mcp` | A compliant MCP server (Streamable HTTP) that relays to the caller's tools. |
| agent → proxy | the proxy | `postgres://diverge@127.0.0.1:81/diverge?sslmode=disable` | A pgwire listener that relays to the caller's database. Raw, with one session per connection. |
| agent → proxy | the proxy | `ws://127.0.0.1:80/daemon` | **Tool containers only.** The proxy closes this socket on an agent at once, because agents hold no account. |

**The port.** If `PORT` is set to a decimal in `1..=65535` with no whitespace, that is the port. In every other case (unset, empty, non-numeric, `0`, out of range) the port is `8080`. The proxy reads the variable on every call, so the agent must bind the same one.

The proxy sets **no timeouts** on any call to the agent: a `/run` stream lasts as long as the loop does.

---

## 2. The five endpoints

Every request is `POST` with `Content-Type: application/json`.

| # | Endpoint | Request body | Success | Failure |
|---|---|---|---|---|
| 1 | `POST /register` | `{ "arguments": <any JSON>, "template": "<sha256 hex>", "image": { "digest", "references"? } }` | `2xx` + register response JSON | any non-`2xx`; second call → `409 {"kind":"registered"}` |
| 2 | `POST /secrets` | `{ "<name>": <value>, … }` | `2xx` once the values are applied (the body is not read) | non-`2xx` |
| 3 | `POST /secrets/collect` | `{ "names": ["<name>", …] }` | `2xx` + `{ "<name>": <value>, … }` with **every** name | non-`2xx` |
| 4 | `POST /run` | `Item`: chat or structured decision | `2xx` `text/event-stream` of `AgenticLoopChunk` | non-`2xx`; before register → `{"kind":"unregistered"}` |
| 5 | `POST /enqueue` | `{ "messages": [Message, …] }` | `204` immediately | `409 {"kind":"closed"}`, or another `4xx` (refused) |

### 2.1 `POST /register`, called once and first

The proxy calls this once, before anything else, when the container begins.

```json
{
  "arguments": { "...": "whatever the image's /.diverge/DIVERGE.md says it takes" },
  "template":  "3f9a…(64 lowercase hex chars: the agent template's id)",
  "image": {
    "digest": "sha256:4b825dc642cb6eb9a060e54bf8d69288fbee4904b825dc642cb6eb9a060e54bf",
    "references": [ { "registry": "ghcr.io", "name": "divergenetwork/some-agent" } ]
  }
}
```

- `arguments` is opaque JSON. The image defines it, and documents it in its own `/.diverge/DIVERGE.md`, which the proxy reads from the image. The program is never asked to describe its arguments.
- `image` is the image this container runs: its manifest `digest` (`<algorithm>:<hex>`) and `references`, the registries and paths it can be pulled from. `references` is omitted when empty. The **provider** supplies it, copying the `image` of the run request it deployed the container from; the daemon sends nothing extra.
- The arguments, the template and the image are **fixed for the container's life**. Every loop runs as the agent built from them.
- A non-`2xx` means the image refuses the arguments, in its own words. The proxy treats an unreachable server, or a `2xx` whose body doesn't parse, the same way.
- A second `/register` gets `409 {"kind":"registered"}`, whatever it carries.

**Response (`2xx`).** Every member is optional, and `{}` is a valid answer.

```json
{
  "dependencies": [ /* Template, … : tool containers the caller should deploy for this agent */ ],
  "secrets":      { "OPENAI_API_KEY": "fixed", "REFRESH_TOKEN": "rotating" },
  "permissions":  [ /* ignored for agent containers (tool containers only) */ ]
}
```

- `secrets` maps a name to its mode, and the order matters. `fixed` means the value is read from its key at every activation. `rotating` means the value is locked to the container while it is active and handed back at inactive (§2.3). Every name must be declared by the agent template.
- `dependencies` is an ordered list of tool templates:

```json
{
  "image":    { "digest": "sha256:…", "references": [ … ] },
  "memory":   536870912,
  "disk":     1073741824,
  "arguments": { … },
  "secrets":  ["NAME", …],
  "mode":     "lazy",
  "scope":    "instance",
  "fuse_file_mounts":      [ { "agent_path": ["work","notes.md"], "tool_path": ["data","notes.md"] } ],
  "fuse_directory_mounts": [ { "agent_path": ["work"],            "tool_path": ["mnt","work"] } ]
}
```

  - `mode` is `"lazy"` or `"eager"`. `scope` is `"instance"`, `"template"` or `"global"`.
  - FUSE mounts are allowed **only** when the mode is `lazy` and the scope is `instance`. Any other combination fails to decode.
  - A dependency's id is the SHA-256 of its canonical JSON: keys sorted, `image.references` dropped, `secrets` sorted. A list that declares the same dependency twice is refused.
  - Within a scope: one `instance` tool per agent, one `template` tool per agent template, and one `global` tool shared by everyone. Each tool's database is shared exactly as widely as the tool.

### 2.2 `POST /secrets`, feeding values in

```json
{ "OPENAI_API_KEY": "sk-…", "REFRESH_TOKEN": "…" }
```

- The body is a flat object holding every declared secret, in declaration order.
- **Accept `{}`.** The SDK comment (`secrets/mod.rs`) says a program with no secrets is never called. But the proxy's driver (`agent/driver.rs`) forwards every feed the server sends, and the server sends an empty feed to open the gate for such a program. So expect `POST /secrets` with `{}` before each loop, and answer it `2xx`.
- The proxy calls this after registration (before the container's id goes out) and **again before every loop** (each activation).
- **Answer only once the values are applied.** Nothing is delivered (no `/run`, no `/enqueue`) until this call returns `2xx`. The proxy does not read the body.
- A non-`2xx` refuses the values and fails the activation. Never echo secret values in an error.

### 2.3 `POST /secrets/collect`, handing rotating values back

```json
{ "names": ["REFRESH_TOKEN"] }
```

Respond `2xx` with **every** name requested:

```json
{ "REFRESH_TOKEN": "the-new-or-unchanged-value" }
```

- The proxy calls this after every loop ends (at each inactive), and only if the program declared at least one `rotating` secret.
- If a name is missing, the body doesn't parse, or the response is non-`2xx`, the run ends with an `Error` of kind `secrets`. That error names the secret, never a value. Extra names are ignored.
- The purpose: if the agent rotated a token during a loop, the new value is written back and the next loop is fed that value.

### 2.4 `POST /run`, one loop

The proxy calls this when no loop is running and messages (or a decision) are waiting. The body is an `Item`, tagged by `type`.

**Chat:**

```json
{
  "type": "chat",
  "messages": [
    { "key": "msg-a", "content": [ { "type": "text", "text": "hello" } ] },
    { "key": "msg-b", "content": [ { "type": "image", "data": "…", "mimeType": "image/png" } ] }
  ]
}
```

- `content` is a list of **MCP `ContentBlock`s**: text, image, audio, resource (embedded), or resource_link.
- `key` is the caller's handle on a message. It is not unique and not interpreted. Echo it back on that message's user chunks.
- Join **all** the messages, in order, into **one user turn**.

**Structured decision:**

```json
{
  "type": "structured_decisions",
  "key": "decision-1",
  "content": [ { "type": "text", "text": "Context for the decision…" } ],
  "questions": [
    { "type": "noul",   "id": "q1", "instructions": [ … ] },
    { "type": "choice", "id": "q2", "instructions": [ … ],
      "options": [ { "value": "yes" }, { "value": "no", "description": [ … ] } ] },
    { "type": "score",  "id": "q3", "instructions": [ … ], "levels": [ [ … ], [ … ], [ … ] ] }
  ]
}
```

Validity rules (from `structured_decisions/validate.rs`):

- There is at least one question, and every id is non-empty and unique.
- `instructions` are non-empty, meaning at least one block that is not empty text.
- A `choice` has at least two options with unique, non-empty values. An option's `description`, if present, is non-empty.
- A `score` has at least two levels, and no level is empty.

**Response.** It is either a non-`2xx` (no loop happened) or a `2xx` SSE stream:

- **Non-`2xx`** means there is no loop to report on: arguments the image won't take, a missing key, a history it can't open, a run already in progress, a block type it doesn't accept, or a structured decision the image doesn't support. The body should be JSON in the error shape (§4). Every message the run was going to carry gets this error as its fate.
- **`2xx`, `Content-Type: text/event-stream`.** Each SSE `data:` is one `AgenticLoopChunk` JSON (§3), and the end of the stream is the end of the loop.
  - **Return the `2xx` as soon as you've taken the messages, before doing the turn's work.** The proxy reports the messages as `delivered` on this `2xx`.
  - **The first chunks must be the user parts:** one chunk per content block of each message, in order, each carrying the message's `key`.
  - **The stream never carries an error event.** A failure after the stream starts, including one during the first turn, is sent as a `notification` chunk with `"is_fatal": true`, and then the stream ends.
  - For a structured decision: the first chunk is one `user_structured_decisions` chunk (the decision echoed back), then one `structured_decisions_response` chunk with the answers, then the stream ends.
  - **Don't set `_meta`** on chunks. The proxy relays them verbatim, and attestation is the daemon's job.
- Only one loop runs at a time. The proxy never starts a second loop while one is open. If one is somehow in progress, answer non-`2xx`.
- Each loop **resumes the conversation** the previous loop left, so the agent keeps its history across runs.

### 2.5 `POST /enqueue`, messages for the running loop

```json
{ "messages": [ { "key": "msg-c", "content": [ { "type": "text", "text": "also…" } ] } ] }
```

The proxy calls this only while a **chat** loop is running, with at most one `/enqueue` in flight at a time.

| Answer | Meaning | What the proxy does |
|---|---|---|
| `204` (immediately) | Accepted onto the program's own queue | Marks every message `delivered`. |
| `409 {"kind":"closed"}` | The loop's last look already found the queue empty and closed the run | Puts the batch back at the **front** of its queue and starts the next `/run` with it. |
| other `4xx` | Content refused (checked **before** accepting anything) | Every message's fate is that error. Nothing is resent. |
| `5xx` / unreachable | — | Treated like `409`: the batch is returned and starts the next run. |

The program's rules:

1. Keep a plain FIFO queue of accepted messages. Nothing is ever withdrawn from it.
2. **Never interrupt.** The turn in flight always runs to completion. At the next *seam* (for example beside the next tool results, or after the assistant finishes), take **everything** queued as **one** user turn, each message's user parts under its own key.
3. **The last look and the close must be atomic.** When the loop is about to end, check the queue. If it holds anything, keep going. If it's empty, close the run in the same step, so that any `/enqueue` after that gets `409 {"kind":"closed"}`. Otherwise a message could be lost when a loop ends.
4. The program is never sent an `/enqueue` for a structured-decision run. Nothing joins a decision run.

---

## 3. Stream chunks (`AgenticLoopChunk`)

Every chunk is a JSON object with a `type` discriminator. MCP content fields (`text`, `data`, `mimeType`, `resource`, `uri`, …) are flattened into it. Assistant-side chunks can carry `parent_tool_call_id` to nest output under a tool call (sub-agents).

| `type` | Fields | Emitted when |
|---|---|---|
| `user_text_content` | `key`, + MCP TextContent (`text`) | echoing a user text block |
| `user_image_content` | `key`, + MCP ImageContent (`data`, `mimeType`) | echoing a user image |
| `user_audio_content` | `key`, + MCP AudioContent | echoing user audio |
| `user_resource` | `key`, + MCP EmbeddedResource (`resource`) | echoing an embedded resource |
| `user_resource_link` | `key`, + MCP Resource (`uri`, `name`, …) | echoing a resource link |
| `user_structured_decisions` | `key`, `content`, `questions` | echoing a decision (first chunk of a decision run) |
| `assistant_reasoning` | `text`, `logprobs?`, `parent_tool_call_id?` | thinking deltas |
| `assistant_text_content` | `text`, `logprobs?`, `parent_tool_call_id?` | answer text deltas |
| `assistant_image_content` | MCP ImageContent, `parent_tool_call_id?` | assistant image |
| `assistant_audio_content` | MCP AudioContent, `parent_tool_call_id?` | assistant audio |
| `assistant_refusal` | `text`, `logprobs?`, `parent_tool_call_id?` | model refusal |
| `assistant_tool_call` | `id`, `name`, `arguments?` (string, streamed in pieces), `parent_tool_call_id?` | tool call deltas (same `id`, arguments concatenated) |
| `tool_response` | `id`, + MCP CallToolResult (`content`, `isError`, `structuredContent?`), `parent_tool_call_id?` | tool result for call `id` |
| `structured_decisions_response` | `decisions`: `{ "<question id>": Answer }` | the answers of a decision run |
| `usage` | `completion_tokens`, `prompt_tokens`, `total_tokens` | token accounting |
| `notification` | `is_fatal` (bool), `message` (any JSON) | status or warnings; `is_fatal: true` = the loop failed, stream ends next |

Text, reasoning, refusal and tool-call chunks are **deltas**. Consumers concatenate consecutive chunks of the same kind; tool calls are merged by `id`.

**Logprob:** `{ "token", "bytes": [u8]|null, "logprob": number, "top_logprobs": [ { "token", "bytes", "logprob" } ] }`.

**Decision answers.** Each answer holds probabilities only. Never send a winner or a score; the daemon computes those.

| Question | Answer |
|---|---|
| `noul` | `{ "noul": 0.25 }` |
| `choice` | `{ "probabilities": { "yes": 0.7, "no": 0.3 } }` (keyed by option `value`) |
| `score` | `{ "probabilities": [0.1, 0.6, 0.3] }` (one per level, in order) |

Numbers are JSON numbers, never strings. Unknown fields are rejected.

**Example stream:**

```
data: {"type":"user_text_content","key":"msg-a","text":"what's 2+2?"}

data: {"type":"assistant_reasoning","text":"Simple arithmetic."}

data: {"type":"assistant_tool_call","id":"call_1","name":"calc","arguments":"{\"expr\":"}

data: {"type":"assistant_tool_call","id":"call_1","name":"calc","arguments":"\"2+2\"}"}

data: {"type":"tool_response","id":"call_1","content":[{"type":"text","text":"4"}],"isError":false}

data: {"type":"assistant_text_content","text":"It's 4."}

data: {"type":"usage","completion_tokens":12,"prompt_tokens":80,"total_tokens":92}

```

---

## 4. Errors

Any non-`2xx` body should be JSON in the protocol's one error shape, an object with a `kind`:

```json
{ "kind": "closed" }
{ "kind": "registered" }
{ "kind": "unregistered" }
{ "kind": "<your kind>", "error": "human-readable reason" }
```

If the body is JSON, the proxy relays it **verbatim**. If it isn't, the proxy wraps it as `{"kind":"program","status":<code>,"error":"<body text>"}`. A server it cannot reach becomes `{"kind":"program","error":"<transport error>"}`.

---

## 5. Lifecycle and ordering

```
container starts
  └─ POST /register               ← once; answer with dependencies + secret declaration
  └─ POST /secrets                ← deploy feed (before the container id is out)
                                     (the gate stays shut after this one)
loop N:
  └─ POST /secrets                ← activation feed; the gate opens when you answer 2xx
  └─ POST /run {chat|decision}    ← 2xx immediately, then stream chunks
       ├─ POST /enqueue …         ← chat runs only, one at a time; 204 / 409 closed / 4xx
       └─ stream ends             ← loop over
  └─ POST /secrets/collect        ← only if any secret is `rotating`
  (gate shuts; proxy emits Inactive upstream; repeat for loop N+1)
```

How the proxy orders work for you:

- Every enqueue the caller makes carries an `index` (0, 1, 2, …). The proxy hands items to the agent **strictly in index order**, holding any that arrive early.
- **Chat batches** at the head of the queue are joined: everything waiting goes in one `/run` or one `/enqueue`.
- **A structured decision** waits until no run is open, then gets a `/run` **alone**. Nothing is handed over until that run ends. For example, `chat, chat, decision, decision, chat, chat` becomes: (both chats joined) → decision 1 → decision 2 → (both chats joined).
- After every loop the gate shuts. No `/run` or `/enqueue` is sent until the next `/secrets` feed returns `2xx`.
- `delivered` means *the program has it*: a `2xx` from `/run` or a `204` from `/enqueue`. No fate waits for a turn to finish.

---

## 6. What the agent can call (inside the container)

- **Tools:** an MCP client pointed at `http://127.0.0.1:80/mcp` (Streamable HTTP). It supports `tools/list`, `tools/call`, `resources/list` and `resources/read`, all relayed to the caller's servers and the agent's declared dependencies. A tool that fails comes back as a result with `isError: true`; only a broken MCP link comes back as an error. Tool names from dependencies are labelled by the dependency's id, and `_meta` keys say which tool serves each one (see `diverge-sdk-rs/src/shared/mcp`).
- **Database:** `postgres://diverge@127.0.0.1:81/diverge?sslmode=disable`. There is no password, and TLS is off because the connection stays on the loopback. Each connection is one session; the proxy retries nothing, so use a pool that reconnects.
- **Daemon:** not available to agents.
- **Image self-description:** `/.diverge/DIVERGE.md` at the image root documents the arguments, and whether the image serves a frontend.

---

## 7. Minimal conformance checklist

- [ ] Listen on `127.0.0.1:${PORT:-8080}`.
- [ ] `/register`: accept once, return `{dependencies?, secrets?}`, and return `409 {"kind":"registered"}` on any repeat.
- [ ] `/run` before register returns `{"kind":"unregistered"}`.
- [ ] `/secrets`: apply the values, *then* return `2xx`. Accept `{}`.
- [ ] `/secrets/collect`: return every requested name.
- [ ] `/run`: validate the content first (`4xx` if unacceptable), return `2xx` SSE immediately, and emit the user parts first, keyed.
- [ ] The stream holds only chunks: failures go out as `notification` with `is_fatal: true`, then the stream closes.
- [ ] Decision run: emit `user_structured_decisions`, then `structured_decisions_response` with probabilities only, then close.
- [ ] `/enqueue`: return `204` immediately, queue the messages, and fold them in at the next seam as one user turn. Make the last look and the close atomic, and answer `409 {"kind":"closed"}` once closed.
- [ ] Never interrupt a turn in flight, and never set `_meta` on chunks.
- [ ] Keep conversation history across loops in the same container.
- [ ] Use `127.0.0.1:80/mcp` for tools and `127.0.0.1:81` for Postgres.
