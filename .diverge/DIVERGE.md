---
{
  "type": "agent",
  "name": "diverge-grok-bot-unofficial",
  "description": "Unofficial. Hands each message to your own Grok Bot through a routine's webhook. Text only; no reply comes back.",
  "schema": {
    "$schema": "https://json-schema.org/draft/2020-12/schema",
    "title": "Arguments",
    "description": "The arguments a Grok Bot agent is made with: the webhook of one\nroutine, which decides which Bot the agent speaks to.\n\nBoth members are required, and any other member is refused, so a\nmisspelled setting is never silently ignored. The arguments are\nstored with the agent's template and are not hidden: whoever can\nread the template can read the key.",
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
  "chat": {
    "prompt_modalities": {
      "image": false,
      "audio": false,
      "file": false
    }
  },
  "structured_decisions": false
}
---
# Grok Bot (unofficial)

An agent container that hands its messages to **your own Grok Bot**.
This image is not made, endorsed or supported by the makers of Grok
Bot. You need your own Grok Bot account, and its terms apply to you.

## What it does today

- **A message in.** Each run joins its messages' text and calls one
  routine's webhook with `{"text": "…"}`. The Bot receives that text
  together with the routine's own instruction.
- **No reply out.** The webhook returns no answer. The run says in one
  line that the message was delivered, stays open for ten seconds, and
  ends; the Bot's answer appears in the Grok Bot app.
- **Text only.** A message carrying an image, audio or a file is
  refused, not trimmed.
- **No frontend yet.**

## Setting it up

1. In the Grok Bot app, ask your Bot to add a webhook trigger to a
   routine. The routine's instruction decides what the Bot does with
   what it is sent.
2. Open the routine's **Webhook** section and copy the address
   ("POST to") and the key.
3. Make an agent template of the image with them as its arguments:

   ```json
   { "webhook_url": "<the address>", "webhook_key": "<the key>" }
   ```

The arguments are stored with the template and are not hidden: whoever
can read the template can read the key. Use a routine whose webhook you
are willing to expose that way, and replace the key in the Grok Bot app
if it leaks.

## The app is not in the image

The Grok Bot desktop app is the vendor's software and this image does
not contain it. The program fetches the current release from the vendor
when the container starts, so a starting container needs the vendor's
download hosts and the Debian mirrors reachable. Nothing uses the app
yet; it is there for the frontend to come.

Arguments: `webhook_url`, `webhook_key`.
