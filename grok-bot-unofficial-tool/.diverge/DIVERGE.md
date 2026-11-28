---
{
  "type": "dependency_tool",
  "name": "grok-bot-unofficial-tool",
  "description": "Unofficial. Where the Grok Bot agent reaches your own Grok Bot: one routine's webhook, behind its API.",
  "schema": {
    "$schema": "https://json-schema.org/draft/2020-12/schema",
    "title": "Arguments",
    "description": "The arguments a Grok Bot tool is made with: the webhook of one\nroutine, which decides which Bot the tool speaks to.\n\nThe `grok-bot-unofficial-agent` image declares this tool as its\ndependency and hands its own `webhook_url` and `webhook_key` on as\nthese. Both members are required, and any other member is refused,\nso a misspelled setting is never silently ignored. The arguments\nare stored with the agent's template and are not hidden: whoever\ncan read the template can read the key.",
    "type": "object",
    "properties": {
      "webhook_url": {
        "description": "The address the routine's webhook is called at, shown as\n\"POST to\" in the Grok Bot app.",
        "type": "string",
        "minLength": 1
      },
      "webhook_key": {
        "description": "The key of the same webhook, sent as its bearer token.",
        "type": "string",
        "minLength": 1
      }
    },
    "additionalProperties": false,
    "required": [
      "webhook_url",
      "webhook_key"
    ]
  },
  "memory": 2147483648,
  "disk": 4294967296,
  "mode": "eager",
  "scope": "instance",
  "api": true
}
---
# Grok Bot tool (unofficial)

The tool of a **`grok-bot-unofficial-agent`** agent: where that agent
reaches **your own Grok Bot**. This image is not made, endorsed or
supported by the makers of Grok Bot. You need your own Grok Bot
account, and its terms apply to you.

It is the agent's own dependency tool, one for each agent. Nothing is
made by hand: the agent declares this image as its dependency and hands
it the webhook, and the daemon deploys it with the agent.

Arguments: `{"webhook_url": "…", "webhook_key": "…"}`, one routine's
webhook, as the Grok Bot app shows it. They are stored with the agent's
template and are not hidden.

## Its door

| door | who may come through | what it is |
|---|---|---|
| `POST /api/run` | the agent the tool is deployed for, and nobody else | one turn: the agent's messages in, the chunks of its run out |

The tool serves an MCP server, as every tool does, with no tool on it.
It declares no permission: it asks the daemon nothing. It has no
frontend yet.

## A turn

The agent posts JSON, `{"messages":[{"key":…,"content":[…]}]}`. The
tool joins the messages' text, calls the webhook with `{"text": "…"}`,
and answers only once the webhook has.

| answer | what it states |
|---|---|
| `200`, server-sent events | The webhook accepted the message. Each event's data is one chunk of the agent's run: the messages' parts, then one line stating that the message was delivered. The stream stays open for ten seconds and ends. |
| `400`, `{"kind":"content",…}` | No message, a message with no content, or content that is not text. |
| `403`, `{"kind":"forbidden",…}` | The caller is not the tool's agent. |
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
