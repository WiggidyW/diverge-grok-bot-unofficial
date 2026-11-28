# Plugins, skills and the other ways in and out of a Grok Bot

**Date:** 2026-11-27.
**Question:** Besides MCP, what does Grok Bot offer that our Diverge tool could use: to set a Bot up without hand-copying, to send it a message, or to get its answer back?

MCP itself is in `MCP.md`. This report covers everything else.

**Sources:** the Grok Bot docs under `docs.x.ai/grok-bot/`, the pages `x.ai/bot`, `x.ai/bot/marketplace` and `x.ai/bot/plugin/674`, and Cursor's plugin pages (`cursor.com/docs/plugins`, `cursor.com/docs/reference/plugins`), all read today. Quotations are from those pages. Nothing here was tried against a real Bot.

## 1. Conclusion

- **A plugin is the one-click way to give a Bot our tools.** It is a package that can hold an MCP server, skills and more, and it has an install link. A custom MCP server has no link; a plugin does.
- **A skill is how we would teach the Bot our routine.** The instruction "take the webhook's text as a message, answer it, send the answer with the reply tool" belongs in a skill, shipped inside the plugin.
- **A plugin does not solve reachability.** Its MCP server still has to be at a public address (`MCP.md` §3).
- **Publishing publicly has a cost.** The marketplace is reviewed by hand and requires the plugin to be open source. A team-only plugin avoids both, but needs a Teams plan.
- **Nothing outside MCP returns a Bot's answer to a program.** There is no outgoing webhook and no reply API. Hooks that fire on a Bot's output exist in Cursor's plugin format, but no Grok Bot page says they run on a Bot.
- **Recommended:** prove the idea with a custom MCP server first. Package it as a plugin only once it works.

## 2. Plugins

### What a plugin is

Grok Bot's own docs never define the format. They use "app", "plugin" and "MCP server" loosely for the same thing. The format is Cursor's, and Grok Bot's plugin pages say so: a plugin is "Published by Slack on the Cursor Marketplace."

Cursor's definition: "Plugins package rules, skills, agents, commands, MCP servers, and hooks into distributable bundles."

The layout, from Cursor's reference page:

```
my-plugin/
├── .cursor-plugin/plugin.json     the manifest
├── skills/<name>/SKILL.md
├── rules/  agents/  commands/
├── hooks/hooks.json
├── mcp.json                       the MCP servers it brings
└── README.md
```

- **The manifest** requires only `name`. Optional fields include `description`, `version`, `author`, `repository`, `license`, `logo`, and the lists of what it bundles.
- **`variables`** declares values the installer supplies, such as a token. Their values are set in the dashboard and are not in the repository.
- **Not confirmed for Grok Bot:** that every part runs on a Bot. The docs show plugins bringing connectors and skills to a Bot. They say nothing of rules, agents, commands or hooks on a Bot.

### How a plugin is installed

| Way | What happens |
|---|---|
| A web link, `https://x.ai/bot/plugin/<id>` | Opens the app with **Add** ready; the page says "By clicking 'Add to Grok Bot', you accept the terms." |
| An app link, `grokbot://app/v1/plugin/add?id=<id>` | The same, for a plugin only a team can see |
| From the app | **Connect apps**, search, add, then sign in to the provider when asked |
| By the Bot | The person says "Add the Linear app"; the Bot installs it and shows a **Connect** card |

- **Added is not signed in:** "**Added** means the app is installed, not that you are signed in."
- **Sign-in has a clock:** "Finish the sign-in within about 10 minutes."

### How a plugin is published

- **Public marketplace:** plugins are "distributed as Git repositories and submitted through the Cursor team." "Every plugin is manually reviewed before it's listed." Cursor's FAQ adds: "All plugins must be open source."
- **Team marketplace:** a Teams or Enterprise plan has its own, managed on the dashboard's **Plugins & MCPs** page. An admin sets each plugin to off by default, on by default, or required. A Teams plan has "up to 1 team marketplace."
- **A personal account** has neither route to a private plugin. It can use public plugins and custom connectors.

### What this means for us

- **A public plugin** would let anyone add our tools to their Bot from a link. Our repository is already public and MIT, so the open-source rule is met. It would be reviewed by the vendor, which also means the vendor would see exactly what we are doing, and could say no.
- **The plugin's MCP address is fixed in the package**, or supplied as a variable. Each Diverge user has their own tool, so the address differs per user unless there is one shared public door in front of every user's tool. That is another reason reachability (`MCP.md` §4, option B) is the real question.

## 3. Skills

- **What one is:** "A **skill** is a reusable set of instructions for how to do a task."
- **Format:** not stated by Grok Bot. In Cursor's plugin format a skill is `skills/<name>/SKILL.md`, with a `name` and a `description` at its top. Grok Bot skills are probably the same; that is an inference.
- **Where they live:** private skills are "one library shared by all your Bots."
- **How a Bot gets one:** the person asks it to save a process as a skill; or uses "Teach a task" to record a browser task; or a plugin brings it.
- **Sharing:** a skill can be published to a team from **Connect apps**. "Skills that came with the app or with a Bot's template can't be published."

**For us:** a skill is the right home for the Bot's side of the arrangement. Without one, the routine's instruction has to be typed by each person, and small differences in wording change what the Bot does with our messages.

## 4. Routines

A routine is a saved instruction with a trigger. This is what our webhook starts today.

- **Triggers:** a schedule, an event from a connected app, an email to the Bot's address, a webhook call, or another Bot.
- **The webhook:** its section shows **POST to**, **key** and **header**. A `200` "means Grok Bot accepted the call and started a run, not that the Bot has finished."
- **Where results go:** to the Bot's chat, or to Slack with the Slack app.
- **Created by asking the Bot.** No page describes creating one by a link or a file.
- **A template can carry routines** (§5), which is one way to hand somebody a ready-made one.

## 5. Templates

A template is a shareable copy of a Bot's setup.

- **Making one:** open the Bot's details, choose **Share**, then **Create a copy**, then **Publish** and **Copy link**.
- **What it carries:** a public link "exposes the Bot's shared configuration: its identity, description, skills, and routines."
- **What it does not:** the Bot's computer, its logins and its conversation history.
- **Using one:** the recipient opens the link and chooses **Add to Grok Bot**, which makes a new Bot on their own account.

**For us:** a template is the closest thing to "one click and the Bot is set up for Diverge". We publish a template of a Bot that already has our skill and a webhook routine. A person adds it, and the new Bot has both.

Two things are not known, and they decide whether this works:

- Whether a routine's webhook trigger survives the copy, and whether the copy gets its own new URL and key. It must, or every copy would share one webhook.
- Whether a custom MCP connector travels with a template. An earlier reading of the templates guide said only first-party plugins do.

## 6. Hooks

Cursor's plugin format lets a plugin run a script on events. The list includes `afterAgentResponse`, `stop`, `beforeMCPExecution` and `afterMCPExecution`.

- **Why it matters:** a hook on `afterAgentResponse` is exactly "run our code each time the Bot says something". It would be a supported way to get the Bot's words out as they are produced.
- **What is not known:** whether any hook runs on a Grok Bot. No Grok Bot page mentions plugin hooks. Cursor documents them for its editor and its agents.
- **Worth one test:** a plugin with a single hook that calls a public address, added to a Bot. If it fires, this is better than a reply tool, because it does not depend on the Bot remembering to call anything.

**Inference hooks** are a different thing: an Enterprise feature, in beta, where the vendor calls an admin's server before each model request with "the conversation so far". It is a control for administrators, it needs an Enterprise plan, and the docs say "Model responses' text isn't checked on its own." It is not a route for us.

## 7. Other ways in and out

| Route | Direction | Usable by a program? |
|---|---|---|
| Webhook on a routine | In | Yes. What we use today. No reply. |
| The Bot's email address (`mail.grokbot.com`) | In and out | In: yes, an email can start a routine. Out: a routine's email "waits as a draft until you press **Send**", so not unattended. |
| Slack, Microsoft Teams | In and out | Only as chat surfaces, through those services' own apps |
| The Bot's cloud computer (shell, browser, WebFetch) | Out | Yes: the Bot can call any public HTTP address when told to. Needs the same public address MCP does. |
| Prompt link, `grokbot://app/v1/prompt?…` | In | No: it fills the composer and a person must press send |
| Action Recording with OpenTelemetry export | Out | Enterprise only, and it carries "ids, outcomes, and durations, never tool arguments, file paths, or message content" |
| Admin API, audit logs | Neither | Settings and records, not messages |

**Not found on any page:** an outgoing webhook, a "send results to this URL" setting, or an API that returns a run's output.

The cloud computer row deserves a note. Even with no MCP server at all, a routine could say "when you have answered, POST the answer to this address", and the Bot would do it from its computer. It is the plainest form of the reply idea. An MCP tool is the tidier version of the same thing, with a defined shape and an approval the person can set to always allow.

## 8. How the pieces would fit

A complete, supported arrangement would be:

1. **A public door** to each user's Diverge tool.
2. **An MCP server** behind it, with `connect` and `reply` tools.
3. **A skill** that tells the Bot how to treat our messages and to answer through `reply`.
4. **A template**, or later a plugin, that gives a new Bot the skill and the routine in one step.

Items 2 to 4 are ours to build. Item 1 is a Diverge feature or a tunnel.

## 9. What is not known

- Whether plugin hooks run on a Grok Bot.
- Whether a template's routine keeps a webhook trigger, with a fresh key for each copy.
- Whether a Bot can read its own routine's webhook URL and key.
- The skill file format on Grok Bot.
- What the marketplace review looks for, and how long it takes.
- Whether a plugin's MCP address can be a per-installer variable.

## 10. Pages read

- `docs.x.ai/grok-bot/`: `deep-links`, `skills-routines-and-automations`, `teams-and-enterprises`, `bots`, `computer-and-apps`, `inference-hooks`.
- `x.ai/bot`, `x.ai/bot/marketplace`, `x.ai/bot/plugin/674`.
- `cursor.com/docs/plugins`, `cursor.com/docs/reference/plugins`, `github.com/cursor/plugins`.
- **Not read:** `grok.com/help/grok-bot/connect-plugins` returned a sign-in page with no article. `cursor.com/docs/plugins/building` and `cursor.com/marketplace` were not fetched.
