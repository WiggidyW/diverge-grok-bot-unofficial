---
{
  "type": "dependency_tool",
  "name": "grok-bot-unofficial-tool",
  "description": "Unofficial. Where the Grok Bot agent reaches your own Grok Bot: one routine's webhook, behind its API.",
  "schema": {
    "$schema": "https://json-schema.org/draft/2020-12/schema",
    "title": "Arguments",
    "description": "The arguments a Grok Bot tool is made with.\n\nNone: the tool is every agent's that declares it, and which Bot a\nturn reaches is said by the turn itself, which names the webhook.\n`{}` is the one value; any member is refused, so a misspelled\nsetting is never silently ignored.\n\n# How it runs\n\nAs a dependency tool of the `grok-bot-unofficial-agent` agent: an\neager tool whose scope is `template`, so one container serves every\nagent that declares it.",
    "type": "object",
    "additionalProperties": false
  },
  "memory": 2147483648,
  "disk": 4294967296,
  "mode": "eager",
  "scope": "template",
  "api": true
}
---
# Grok Bot tool (unofficial)

The tool of a **`grok-bot-unofficial-agent`** agent: where that agent
reaches **your own Grok Bot**. This image is not made, endorsed or
supported by the makers of Grok Bot. You need your own Grok Bot
account, and its terms apply to you.

It is a shared dependency tool: one container serves every agent that
declares it. Nothing is made by hand: the agent declares this image as
its dependency, and the daemon deploys it. The tool holds no webhook of
its own; each turn names the one it is for.

Arguments: `{}`.

## Its door

| door | who may come through | what it is |
|---|---|---|
| `POST /api/run` | any agent that declares the tool, and nobody else | one turn: the agent's messages in, the chunks of its run out |

The tool serves an MCP server, as every tool does, with no tool on it.
It declares no permission: it asks the daemon nothing. It has no
frontend yet.

## A turn

The agent posts JSON:

```json
{
  "webhook": { "url": "<the address>", "key": "<the key>" },
  "messages": [{ "key": "…", "content": [] }]
}
```

`webhook` is one routine's webhook, as the Grok Bot app shows it, and
says which Bot the turn reaches. The tool joins the messages' text,
calls that webhook with `{"text": "…"}`, and answers only once the
webhook has. It keeps neither the address nor the key.

| answer | what it states |
|---|---|
| `200`, server-sent events | The webhook accepted the message. Each event's data is one chunk of the agent's run: the messages' parts, then one line stating that the message was delivered. The stream stays open for ten seconds and ends. |
| `400`, `{"kind":"webhook",…}` | The turn names no webhook: an empty address or an empty key. |
| `400`, `{"kind":"content",…}` | No message, a message with no content, or content that is not text. |
| `403`, `{"kind":"forbidden",…}` | The caller is not an agent. |
| `409`, `{"kind":"unregistered",…}` | The tool is not registered yet. |
| `502`, `{"kind":"webhook",…}` | The webhook was not reached, or did not answer `200`. No run started. |

The webhook returns no reply, so a turn carries none: the Bot's answer
appears in the Grok Bot app.

## The app is not in the image

The Grok Bot desktop app is the vendor's software and this image does
not contain it. The program fetches the current release from the vendor
when the container starts, so a starting container needs the vendor's
download hosts and the Debian mirrors reachable. Nothing uses the app
yet; it is there for the frontend to come.
