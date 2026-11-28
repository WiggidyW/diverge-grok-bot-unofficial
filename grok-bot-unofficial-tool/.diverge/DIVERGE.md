---
{
  "type": "dependency_tool",
  "name": "grok-bot-unofficial-tool",
  "description": "Unofficial. Where the Grok Bot agent reaches your own Grok Bot: one routine's webhook, behind its API.",
  "schema": {
    "$schema": "https://json-schema.org/draft/2020-12/schema",
    "title": "Arguments",
    "description": "The arguments a Grok Bot tool is made with.\n\nOne tool is one Grok Bot account: the app signs in once, so each\naccount has a container of its own, and every agent made with the\nsame `account` shares it. Which Bot of the account a turn reaches\nis said by the turn itself, which names the webhook. Any other\nmember is refused, so a misspelled setting is never silently\nignored.\n\n# How it runs\n\nAs a dependency tool of the `grok-bot-unofficial-agent` agent: an\neager tool whose scope is `template`, so one container serves every\nagent that declares it with the same account.",
    "type": "object",
    "properties": {
      "account": {
        "description": "A name for the Grok Bot account, chosen by whoever makes the\nagents: any text that is not empty. It is a label and not a\ncredential, and nothing checks it against the vendor. It tells\none account's tool from another's, and nothing else is done\nwith it yet.",
        "type": "string",
        "minLength": 1
      }
    },
    "additionalProperties": false,
    "required": [
      "account"
    ]
  },
  "memory": 4294967296,
  "disk": 8589934592,
  "mode": "eager",
  "scope": "template",
  "frontend": true,
  "api": true
}
---
# Grok Bot tool (unofficial)

The tool of a **`grok-bot-unofficial-agent`** agent: where that agent
reaches **your own Grok Bot**. This image is not made, endorsed or
supported by the makers of Grok Bot. You need your own Grok Bot
account, and its terms apply to you.

**One tool is one Grok Bot account.** The app signs in once, so each
account has a container of its own, and every agent made with the same
`account` shares it. Nothing is made by hand: the agent declares this
image as its dependency with its account, and the daemon deploys it.
The tool holds no webhook of its own; each turn names the one it is
for, which says which Bot of the account is reached.

Arguments: `{"account": "…"}`, a name you choose for the account. It is
a label and not a credential: nothing checks it against the vendor, and
nothing is done with it yet beyond telling one account's tool from
another's.

## Its two doors

| door | who may come through | what it is |
|---|---|---|
| `/frontend` | a person: a request the daemon stamps as a client | the tool's desktop, with the Grok Bot app and a browser on it |
| `POST /api/run` | any agent that declares the tool, and nobody else | one turn: the agent's messages in, the chunks of its run out |

The tool serves an MCP server, as every tool does, with no tool on it.
It declares no permission: it asks the daemon nothing.

## The frontend

The frontend is a remote desktop inside the tool's container. It shows
two windows, and a taskbar to switch between them:

- **The Grok Bot app**, the vendor's own, running here.
- **A browser**, Chromium, also running here.

Both are in the container on purpose. Signing in to the app opens a web
page, and that page ends by handing a link back to the app, which only
works when the browser and the app are on the same machine. Any link
the app opens goes to this browser, and not to the browser on your own
computer.

To sign in: open the frontend, wait for the Grok Bot window, choose to
sign in, and finish in the browser window that opens beside it.

- **Everyone who opens the frontend sees the same desktop**, and whoever
  can open it can use the signed-in account.
- **A window you closed** comes back from the menu on a right click of
  the desktop's background.
- **The sign-in is kept in the container** and is lost if the container
  is replaced.
- **Not there yet:** the clipboard, sound, and sending files through the
  page.

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
download hosts and the Debian mirrors reachable. That takes a few
minutes, and the frontend says so while it does. The rest of the
desktop (the display, the browser, the noVNC web client) is Debian's
own software and is in the image.

## Trying it by hand

With `GROK_BOT_TOOL_UNSTAMPED=1` in its environment the program listens
on every address and takes a frontend request with no stamp as a
person's, so the image can be tried with a container engine alone:
`podman run --rm -e GROK_BOT_TOOL_UNSTAMPED=1 -p 8080:8080 <image>`,
then open `http://127.0.0.1:8080/frontend/`. Under Diverge the variable
is never set.
