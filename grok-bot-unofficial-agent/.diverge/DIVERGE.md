---
{
  "type": "agent",
  "name": "grok-bot-unofficial-agent",
  "description": "Unofficial. Hands each message to your own Grok Bot, through its own tool. Text only; no reply comes back.",
  "schema": {
    "$schema": "https://json-schema.org/draft/2020-12/schema",
    "title": "Arguments",
    "description": "The arguments a Grok Bot agent is made with.\n\nThe agent is a thin wrapper: Grok Bot is reached from a tool\ncontainer of its own, the `grok-bot-unofficial-tool` image, which\nthe agent declares as its one dependency. The agent must be told\nwhich image that is, and names the webhook to it on every run:\nthe tool is shared by every agent that declares it, and holds no\nwebhook of its own. Every member\nis required, and any other member is refused, so a misspelled\nsetting is never silently ignored. The arguments are stored with\nthe agent's template and are not hidden: whoever can read the\ntemplate can read the key.",
    "type": "object",
    "properties": {
      "tool": {
        "description": "The `grok-bot-unofficial-tool` image the agent's tool is made\nfrom: its digest, and the references under which its bytes may\nbe fetched. For testing; a released agent image will name it\nitself.",
        "$ref": "#/$defs/Image"
      },
      "webhook_url": {
        "description": "The address a routine's webhook is called at, shown as\n\"POST to\" in the Grok Bot app.",
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
      "tool",
      "webhook_url",
      "webhook_key"
    ],
    "$defs": {
      "Image": {
        "description": "The image, by its manifest digest and the references under which\nits bytes may be fetched — what\n[`images::read`](crate::provider::endpoints::images::read::client::request::Frame)\nasks about, so a read scope that came back held names an image a\nrun can ask for, with nothing to translate between them. The same\nvalue is the whole of the shared\n[`oci::has`](crate::shared::containers::oci::has) request a\nprovider asks a caller on a run, and the image every\n[`oci::prove`](crate::shared::containers::oci::prove) ask on a\nscope naming it is over.\n\n# The digest is the image\n\nA digest cannot be repointed at different content, so an image\nmeans the same bytes every time, wherever they come from. That is\nthe whole of what the wire asserts: the provider runs the image\nthe digest names and no other. The references are not the image:\nthe same bytes sit under different names on different registries,\nand an id that hashes an image hashes it with none — see\n[`hashed`](Self::hashed).\n\n# Where the bytes come from\n\nA provider looks in the caller's own Store first, by digest alone —\nthe images the caller\n[built](crate::provider::endpoints::images::build) or\n[pushed](crate::provider::endpoints::images::push) on it, and no\nother caller's. Failing that it asks any referenced registry its\nown policy lists, with the credential it holds for it, and ignores\na reference naming one it does not; and failing that it takes the\nimage from the caller — asking on the run scope whether the caller\nholds it, naming this image, and, when it does, for its manifest\nand blobs, then for the proof that it holds it, see\n[`oci`](crate::shared::containers::oci). The order, and which\nanswered, is stated on\n[`Source`](crate::shared::containers::response::Source), which the\nrun's answer carries. A caller need not hold what it names; one\nthat does answers so when asked.",
        "type": "object",
        "properties": {
          "digest": {
            "description": "The manifest digest, `<algorithm>:<hex>`.\n\nWhat identifies the image: a runtime hashes what it pulls, so\na source serving other bytes under it fails before anything\nruns.",
            "type": "string"
          },
          "references": {
            "description": "Where the bytes may be fetched, each a registry and a path\nthere: see [`Reference`]. Absent when there are none, and then\nthe provider has only what it holds and what the caller holds.",
            "type": "array",
            "items": {
              "$ref": "#/$defs/Reference"
            }
          }
        },
        "required": [
          "digest"
        ]
      },
      "Reference": {
        "description": "One place the bytes an image's digest names may be fetched from: a\nregistry, and the repository path there. A reference locates and\ndoes not identify — the digest does — so an image may carry none,\none or several, and two images with one digest are one image\nwhatever each lists. A provider asks a referenced registry only\nwhere its own policy lists that registry, with the credential it\nholds for it, and ignores a reference naming one it does not.",
        "type": "object",
        "properties": {
          "registry": {
            "description": "The registry's host, as a reference names it before its first\n`/`: `docker.io`, `ghcr.io`, `registry.example.com:5000`. No\nscheme and no path.",
            "type": "string"
          },
          "name": {
            "description": "The repository path at that registry — `library/nginx`,\n`myorg/myimage`. No host and no tag: the image is\n`<registry>/<name>@<digest>` there.\n\nIt lands in a reference by concatenation wherever a provider\npulls from, so a provider refuses one that is not a repository\npath and normalizes nothing.",
            "type": "string"
          }
        },
        "required": [
          "registry",
          "name"
        ]
      }
    }
  },
  "memory": 536870912,
  "disk": 1073741824,
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
# Grok Bot agent (unofficial)

An agent that hands its messages to **your own Grok Bot**. This image
is not made, endorsed or supported by the makers of Grok Bot. You need
your own Grok Bot account, and its terms apply to you.

The agent is a thin wrapper. Grok Bot is reached from a second
container, the **`grok-bot-unofficial-tool`** tool, which this agent
declares as its one dependency and reaches over the tool's API.

- **A run** posts its messages to the tool with the agent's webhook,
  which says which Bot the run is for. The tool is shared by every
  agent that declares it. It calls the webhook with the messages' text, and the events it answers with are
  the agent's run.
- **No reply out.** The webhook returns no answer. The run says in one
  line that the message was delivered, stays open for ten seconds, and
  ends; the Bot's answer appears in the Grok Bot app.
- **Text only.** A message carrying an image, audio or a file is
  refused, not trimmed.
- **A failure is known at once.** If the webhook cannot be called, the
  run is refused with why, and nothing is streamed.

## Arguments

```json
{
  "tool": { "digest": "sha256:…" },
  "webhook_url": "<the address>",
  "webhook_key": "<the key>"
}
```

- **`tool`** is the `grok-bot-unofficial-tool` image, with its
  `references` where its bytes are to be fetched from a registry. It is
  an argument for testing.
- **`webhook_url`** and **`webhook_key`** are one routine's webhook, as
  the Grok Bot app shows them. The agent names them to its tool on
  every run.

The arguments are stored with the template and are not hidden: whoever
can read the template can read the key. Use a routine whose webhook you
are willing to expose that way, and replace the key in the Grok Bot app
if it leaks.

## What it asks of the daemon

One grant, declared at its registration: `api` over the dependency
tools attached to it, which is its tool and nothing else.

```json
{"dependency_tools": {"actions": ["api"], "within": "attached"}}
```

## Setting it up

1. Have both images where the provider can find them: this one and
   `grok-bot-unofficial-tool`.
2. In the Grok Bot app, ask your Bot to add a webhook trigger to a
   routine. The routine's instruction decides what the Bot does with
   what it is sent.
3. Open the routine's **Webhook** section and copy the address
   ("POST to") and the key.
4. Make an agent template of this image with the arguments above, and
   an agent of the template.
5. Message the agent. Its tool is deployed with it.
