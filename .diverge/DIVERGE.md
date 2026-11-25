---
{
  "type": "agent",
  "name": "diverge-grok-bot-unofficial",
  "description": "Unofficial. Hands each message to your own Grok Bot through a routine's webhook. Text only; no reply comes back.",
  "schema": {
    "$schema": "https://json-schema.org/draft/2020-12/schema",
    "title": "Arguments",
    "description": "The arguments a Grok Bot agent is made with.\n\nNone: which Bot the agent speaks to is decided by the webhook its\nsecrets name. `{}` is the one value; any member is refused, so a\nmisspelled setting is never silently ignored.\n\n# Secrets\n\n`GROK_BOT_WEBHOOK_URL` and `GROK_BOT_WEBHOOK_KEY`: the address and\nthe key of one routine's webhook, as the Grok Bot app shows them.",
    "type": "object",
    "additionalProperties": false
  },
  "memory": 2147483648,
  "disk": 4294967296,
  "secrets": [
    {
      "name": "GROK_BOT_WEBHOOK_URL",
      "description": "The address a routine's webhook is called at, shown as \"POST to\" in the Grok Bot app."
    },
    {
      "name": "GROK_BOT_WEBHOOK_KEY",
      "description": "The key of the same webhook, sent as its bearer token."
    }
  ],
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
- **No reply out.** The webhook returns no answer. The run ends with one
  line saying the message was delivered; the Bot's answer appears in the
  Grok Bot app.
- **Text only.** A message carrying an image, audio or a file is
  refused, not trimmed.
- **No frontend yet.**

## Setting it up

1. In the Grok Bot app, ask your Bot to add a webhook trigger to a
   routine. The routine's instruction decides what the Bot does with
   what it is sent.
2. Open the routine's **Webhook** section and copy the address
   ("POST to") and the key.
3. Bind them to the secrets `GROK_BOT_WEBHOOK_URL` and
   `GROK_BOT_WEBHOOK_KEY`.
4. Make an agent template of the image with the arguments `{}`.

## The app is not in the image

The Grok Bot desktop app is the vendor's software and this image does
not contain it. The program fetches the current release from the vendor
when the container starts, so a starting container needs the vendor's
download hosts and the Debian mirrors reachable. Nothing uses the app
yet; it is there for the frontend to come.

Secrets: `GROK_BOT_WEBHOOK_URL`, `GROK_BOT_WEBHOOK_KEY`. Arguments: `{}`.
