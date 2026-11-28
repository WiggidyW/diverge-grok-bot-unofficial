# Letting the Bot connect itself: Grok Bot and MCP

**Date:** 2026-11-27.
**Question:** Can a Grok Bot set its own webhook details into Diverge, through MCP or anything like it, so that nobody copies a URL and a key by hand?

**Sources:** eight pages of the Grok Bot docs under `docs.x.ai/grok-bot/` and Cursor's MCP page at `cursor.com/docs/context/mcp`, all read today. Quotations are from those pages. Nothing here was tried against a real Bot.

## 1. Conclusion

- **Yes, Grok Bot speaks MCP, and the Bot can add a server itself.** The docs say so in terms (§2).
- **So the idea works in principle.** We serve an MCP server with a tool such as `connect(webhook_url, webhook_key)`. The Bot adds the server, creates its routine, and calls the tool with its own details.
- **The one real obstacle is reachability.** The Bot reaches a remote MCP server from the vendor's servers, over public HTTPS. A Diverge tool has no public address (§3).
- **MCP would do more than setup.** The same server can carry the Bot's reply back to us, which is the thing the webhook cannot do. That is a supported route to real chunks, and it does not depend on reading the app's traffic (§5).
- **Recommended next step:** try it with a temporary public tunnel from the tool's container, before asking Diverge for anything (§6).

## 2. What Grok Bot supports

### MCP servers

- **Stated support:** "Grok Bot supports custom tooling, so it can interact with any MCP or API."
- **Two kinds of server**, in the desktop app's "Add a custom connector" form:
  - **Remote HTTPS:** "takes a **Server URL**, with optional **HTTP headers** and an OAuth **Client ID**".
  - **Command:** "takes a **Command** and its **Arguments**."
- **The Bot can add one itself:** "A Bot can also add an MCP server that isn't in the catalog, by URL or by a command that runs on its computer."
- **Where a command server runs:** on the Bot's cloud computer, not on the person's machine and not inside the desktop app. "The Grok Bot cloud computer is separate from the Mac or Windows computer in front of you."
- **Where a remote server is called from:** "Bots reach remote MCP servers from Cursor's servers".
- **Not stated:** which remote transport (streamable HTTP or SSE). Cursor's own MCP page lists both, and Grok Bot is said to support "the same MCP servers, plugins, and skills as Cursor", so both are likely. That is an inference.

### Signing in to a server

- **Headers:** a remote server takes fixed HTTP headers, which is enough for a bearer token.
- **OAuth:** supported, with tokens "held on Cursor's connector backend, and the Bot invokes tools without receiving them."
- **Not stated:** how a header's value is stored, or whether the Bot can see it.

### Approvals

- **Auto Review** "checks each action your Bot is about to take, for example before it runs a command, uses a connected app, or uses its computer."
- **A private network needs a yes first:** "Adding a server on a private network, or one that uses a private certificate authority, asks for your approval first."
- **Teams can restrict it:** an admin can block a connector, and an Enterprise team can keep an allowlist of MCP servers. A personal account has neither.

### Plugins and skills

- **A skill** is "a reusable set of instructions for how to do a task."
- **A plugin** is a marketplace item that bundles connectors and skills. The docs do not describe how one is made or published.
- **A custom connector has no link:** "A custom connector you added yourself has no link." So a one-click install link is only for marketplace plugins.

### Other ways out of the Bot

The docs name what a network policy covers: "Shell commands and the browser on the computer, WebFetch, and connected apps (MCP servers)". So besides MCP, the Bot can call any public HTTP address from its cloud computer.

## 3. The obstacle: our tool has no public address

A Diverge tool's program listens on its container's loopback. Requests reach it only through the daemon, from an account, an agent or another tool. I found no door in Diverge that gives a tool a public URL.

Both kinds of MCP server need to reach us from outside:

| Kind | Where it runs | What it needs from us |
|---|---|---|
| Remote HTTPS | The vendor's servers call our URL | A public HTTPS address |
| Command | The Bot's cloud computer | Something public for the command to dial out to |

Neither works with the tool as it is today.

## 4. Ways to become reachable

| Option | What it is | Diverge change | Notes |
|---|---|---|---|
| **A. A tunnel from the tool** | The tool starts an outbound tunnel client that gives it a public HTTPS URL | None | Fastest to try. Adds third-party software and a third-party service in the path. |
| **B. A public door in Diverge** | Diverge offers a tool a public URL, relayed to its `/mcp` or `/api` | A new feature | The right long-term answer. General: any tool whose service must call it back would use it. |
| **C. A relay we host** | A small public service the tool dials out to and the Bot calls | None in Diverge | Works, but it is a service to run and to trust. |

The server would be public whichever option is taken, so it must refuse strangers. A long random token in a header, made by the tool and different for each account, is enough for a first version.

## 5. What the server would offer

### Setup: the Bot connects itself

1. The tool's frontend shows one line to give the Bot, holding the server's URL and its token.
2. The person pastes it to the Bot: "Add this MCP server and connect yourself."
3. The Bot adds the server. This may raise one approval card.
4. The Bot creates a routine with a webhook trigger and calls our `connect` tool with the routine's URL and key, and a name for itself.
5. The tool stores the pairing. The agent then names its Bot by that name, and the key is no longer in the agent's template.

One thing is not confirmed: that a Bot can read its own routine's webhook key. The Bot's own words suggest it can ("I can usually connect the app myself"), and the docs do not say.

### The reply: real chunks, the supported way

The routine's instruction can end with: "send your answer with the `reply` tool."

- **`reply(run, text)`** carries the Bot's answer to our tool, which emits it as the run's chunk.
- **`progress(run, text)`** could carry what the Bot is doing while it works.
- The webhook's body would carry the `run` id, so a reply is matched to its run.

This changes the picture for the whole project:

| | Through MCP | Through the app's traffic |
|---|---|---|
| Is it a route the vendor offers? | Yes: "it can interact with any MCP or API" | No: it reads the app's own connection |
| How fine are the chunks? | One per tool call: whole messages, not word by word | Whatever the app receives, likely word by word |
| Does it need the app running and signed in? | No | Yes |
| Does it depend on undocumented formats? | No | Yes, and they can change without notice |

The two are not exclusive. MCP could give dependable whole replies now, and the traffic log could still show us whether finer streaming is worth pursuing.

A caution: the reply depends on the Bot following its routine's instruction. A model can forget to call a tool. The run needs a timeout and a plain statement when no reply came.

## 6. Recommendation

1. **Try option A first, as an experiment.** Add a real MCP server to the tool (it already serves an empty one at `/mcp`), put a temporary tunnel in front of it, and ask a Bot to add it and call `connect`. That answers the open questions in an afternoon.
2. **If it works, add `reply`** and see what a run looks like when the Bot answers through it.
3. **Then decide on option B.** If this is how the agent should work, a public door for tools is worth asking Diverge for, and the tunnel goes away.

## 7. What is not known

- Whether a Bot can read its own routine's webhook URL and key.
- Whether a Bot reliably calls a reply tool when its routine says to, and how long after the webhook that happens.
- Which remote transport Grok Bot uses, and whether it accepts our server's version of the protocol.
- How a header's token is stored on the vendor's side.
- Whether adding a custom connector by chat works on a personal plan the same as the form does. The form is desktop only.
- What Auto Review does with our tools: whether each call raises a card until "Always allow" is chosen.

## 8. Pages read

- `docs.x.ai/grok-bot/`: `overview`, `computer-and-apps`, `skills-routines-and-automations`, `vault`, `deep-links`, `approvals-security-and-privacy`, `teams-and-enterprises`, `security`.
- `cursor.com/docs/context/mcp`.
- **Not read:** Cursor's plugins page (it returned `404`), the marketplace pages under `x.ai/bot/plugin/`, and `grok.com/help/grok-bot/connect-plugins`.
