# Grok Bot: what it is and what it does

A survey of Grok Bot's features as its own site describes them, read on 2026-11-24.

**Sources read**

- `https://x.ai/bot` (home, pricing, FAQ)
- `https://x.ai/news/grok-bot-1-0` (launch post, 2026-11-11)
- `https://x.ai/bot/marketplace` and `https://x.ai/bot/use-cases`
- Guides under `https://x.ai/bot/guides/`: `getting-started`, `grok-bot-101`, `templates-for-grok-bot`, `how-i-run-multiple-teams-of-grok-bots`, `grok-bot-for-engineering`, `grok-bot-for-work`, `grok-bot-for-support`, `grok-bot-for-mobile-app-development`, `designing-grok-bot-with-grok-bot`

**Not read**

- `https://x.ai/changelog/bot` and `https://x.ai/legal/faq-enterprise` both returned 403.
- Ten role guides (marketing, Linear, founder-led sales, performance marketing, post-sales, Clay, recruiting, SDRs, legal ops, GTM, PMs) were not fetched.
- `docs.x.ai` was not fetched; no Grok Bot page there is linked from the pages above.

> **Correction, 2026-11-24.** This report was written from the marketing pages before the product docs at `https://docs.x.ai/grok-bot/` were found. The docs contradict it on these points: desktop clients exist for Windows and Linux; an Android app exists; Bots have voice chat and voice memos; a Bot can have its own email address; a routine can be started by an HTTP webhook; Bots can be reached in X Chat and by tagging `@bot` on X; all of a user's Bots share one cloud computer. See `GROK_BOT_INTERACTION.md`, which is written from the docs. Sections 2 and 14 below are the ones affected.

Everything below is what those pages state. Where a claim comes from a guide author's own setup rather than a product statement, it is marked "(author's setup)".

---

## 1. What Grok Bot is

Grok Bot is a product for running long-lived AI workers called **Bots**. The site's short definitions:

- "AI teammates you can give real work to. Bots sign in to your tools, work with your entire team, and come back with finished work."
- "Grok Bot is an agent with a computer."
- "A personal assistant that can take any action on a computer."

A Bot is not a chat window. Each one is a standing worker with a job, its own cloud computer, memory, and a schedule. Version 1.0 shipped on 2026-11-11, after a beta.

### Anatomy of a Bot

| Part | What it is |
|---|---|
| **Identity** | Three settings: `Name`, `Title`, `Description`. The description is free text and acts as the Bot's job prompt. Set in chat or in Settings. |
| **Rules** | Natural-language rules in Settings > General > Agent. "The rules are a prompt." |
| **Computer** | A cloud machine with a desktop, filesystem, terminal, browser and apps. |
| **Connections** | Accounts the Bot is signed in to (Gmail, Slack, GitHub, …). |
| **Routines** | Standing work on a schedule or an event trigger. |
| **Skills** | Recorded or written playbooks for a task. |
| **Memory** | What the Bot has learned from conversations and teaching. |
| **Plugins / MCP servers** | Extra tools. "Supports the same MCP servers, plugins, and skills as Cursor." |
| **Handoffs** | The ability to message and trigger other Bots directly. |

### Vendor note

The pages are served from x.ai and footed "© 2026 SpaceX Super Intelligence LLC", but:

- the download link points to `api2.cursor.sh`, and "Contact sales" to `cursor.com/contact-sales?product=grok-bot`;
- the FAQ says Grok Bot "uses the same Cursor SSO, auth, and privacy mode";
- coding work is delegated to Cursor cloud agents (§7).

The pages do not explain the relationship beyond this.

---

## 2. How people talk to a Bot

The 101 guide names three ways in:

1. **Chat.** A person messages the Bot.
2. **Routines and triggers.** The Bot runs on its own schedule or reacts to an event in another app.
3. **Other Bots.** Bots message and trigger each other.

### Surfaces

| Surface | Detail |
|---|---|
| Grok Bot app, desktop | macOS (Apple silicon) is the named download. "Other platforms and devices" exists as a link; no Windows or Linux client is named in the text read. |
| Grok Bot app, iOS | "The Grok Bot Mobile App". |
| Slack | Bots work "right in Slack like any teammate", in channels and threads. |
| Microsoft Teams | Same role as Slack. |
| Video calls | Google Meet, Zoom, Teams (§9). |
| Channels inside the app | A sidebar "Grok Bot channel" can hold several Bots and the user together (author's setup: one channel per project). |

Not found on any page read: WhatsApp, iMessage, Telegram, Discord, SMS, email as a chat surface, or a web app.

### Conversation behaviour

- A Bot carries a project "from start to end" and "checks in when your approval is needed."
- A Bot can be blocked and say so: it marks the task blocked and pings the user in the channel (author's setup).
- Follow-ups can be sent to running work "by queueing a message or interrupting the run" (stated for Cursor cloud agents the Bot drives).
- Notifications arrive when work finishes; results can be reviewed on mobile.

---

## 3. The Bot's computer

Every Bot has its own computer, in the cloud.

- **Contents:** "a desktop, a filesystem, a terminal, and apps", with a browser.
- **Always on:** runs "24/7, even when your laptop is closed". The same computer is reachable from phone and desktop.
- **What it does there:** "use apps, browse the web, write and run code." It uses websites "just like you would, even the hard-to-navigate ones."
- **Isolation:** "The work still happens in an isolated environment."
- **Encryption:** "encrypted in transit and at rest".
- **Local services:** a Bot can run a dev server and a "playground on localhost" on its machine, and return "a screenshot plus a live URL".

### Signing in

- "Sign a Bot in once, and it uses your apps and websites."
- Multiple accounts per service can be connected.
- **Connectors:** "tell it what you want to connect and click authorize", then a one-click link in the browser.
- **Desktop handback:** when a flow needs a human, the Bot hands the desktop over "the same way a remote computer would", for a CAPTCHA or a 2FA login.
- **Secure form:** the Bot can send "a secure form for a password or an API key."
- **Shared sign-ins:** "if you log into a site with one bot, every other bot can reach that site too."

### Own hardware

Bots can start Cursor cloud agents "on your own worker machines, like a spare Mac mini" (a "Cursor Cloud private worker"). That is how iOS Simulator work and screenshots are done.

---

## 4. Tools and integrations

### Built-in abilities

| Ability | What the pages say |
|---|---|
| Browser use | Navigates sites, handles sign-ins. Improved in 1.0 ("Better browser use"). |
| Computer use | "Drives the UI on a computer, captures failures." |
| Terminal and code | Writes and runs code on its computer. |
| Web search | A marketplace Bot "runs every pass on the latest Grok model with live web search." |
| Documents | Writes documents. |
| Presentations | Builds presentations; delivers "an editable link". |
| Pages | "Interactive Pages that you can edit together or ask them to keep updated." |
| Screen and video capture | Takes screenshots, records "a video of the bad state", returns "screenshots or a clip". |
| Images | Reads screenshots to check its own work; one marketplace Bot generates profile pictures from a photo; another writes image alt text. |

### Named integrations

Gmail, Google Calendar, Google Drive, GSuite, GitHub, Slack, Microsoft Teams, Notion, Linear, Jira, Figma, Salesforce, Gong, Ramp, Clay, Stripe, Sentry, PostHog, Google Ads, Meta Ads, AppLovin, Adjust, Google Cloud, Apple and Google Play (app stores), LinkedIn, Facebook Marketplace, Craigslist, X (bookmarks), Cursor cloud agents.

Where a service has no usable API, the Bot uses the website through its browser. One author taught a Bot to upload ad creatives through Meta's web UI because the API was unavailable.

### Extension points

- **MCP servers, plugins, skills:** "the same MCP servers, plugins, and skills as Cursor." Example: the Figma MCP, used to read exact positions and typography from a file.
- **First-party vs non-standard plugins:** the templates guide distinguishes them; only first-party ones travel with a template (§10).
- **Slash-invoked skills:** guides show skills called by name, such as `/react-native-best-practices`.

---

## 5. Routines and triggers

Routines are "standing work on a clock. Not a reminder you set, and not something you have to open the app to trigger." One guide calls them "Routine (aka Automation)".

- **Schedules.** Examples from the guides: every 30 minutes, hourly, daily at 5 a.m., nightly at 3 a.m., "twice daily on weekdays at 8am and 5pm", "Mondays at 9:00 AM", with a time zone. One reproduced prompt shows a cron expression, `5 9,16 * * 1-5`.
- **Event triggers.** A Bot can "listen for events from other apps. It can watch a Slack thread or a GitHub PR."
- **Who creates them.** The user, in a few words of chat, or the Bot: "A bot can set its own schedule." One author reports a Bot adding a routine to its own remit unasked.
- **Enable switch.** Routines can be created disabled and wait for the user to enable them (author's setup).
- **Temporary routines.** A short-lived high-frequency check, such as every five minutes on a P0 (author's setup), with the warning that it "can burn tokens much faster than you think."
- **Chains.** "A weekday routine can route a request through different bots."

---

## 6. Memory, skills and teaching

### Memory

- "Bots keep context from conversations, learn how your team works."
- Bots "remember decisions" and "learn individual preferences."
- The app shows a "Saved to memory" marker when something is stored.
- Each Bot has its own memory ("a different memory system and limited context", author's description).
- Team Bots: "Manage what a Team Bot knows" in settings.
- Shared learning: "When someone teaches a Bot a better way to complete a task, that improvement can benefit everyone who uses it."

### Self-improvement

Bots "suggest ways to improve themselves" and "suggest changes to how they work based on what they learn from the team." The app shows these as suggestions from the Bot.

### Teaching a task

- **By recording:** "Show your Bot a workflow once. It remembers the steps and does them on its own after that." The user presses record and does the task on the Bot's computer; the Bot "keeps the click-path". The control is labelled "Teach a task".
- **By prompt:** a workflow can also be taught by writing it out.
- The result is a **skill**: "Recorded playbooks."

---

## 7. Coding

Grok Bot does not write code itself in the flows the guides describe. "It is creating the same prompts I would, then sending them to a specialized coding harness that lives inside Cursor."

- Bots "create Cursor cloud agents, read transcripts, review proofs attached to PRs, and send follow-ups."
- They monitor a run's transcript and artifacts, and "keep working until the goal is met."
- They open PRs, watch PRs on a cadence, react to review comments, failing CI and merge conflicts, and start code-review runs.
- Auto-merge is possible "if the review is highly confident and the blast radius is low" (author's setup).
- One author reports a fleet managing "more than 200" simultaneous agents.
- Stated limit: a Bot cannot fix something "when Grok Bot doesn't have the security permissions to fix it itself."

---

## 8. Many Bots

- **Parallel work.** "They work in parallel, hand off to each other, and keep going 24/7."
- **Bot-to-bot messaging.** "Bots can message each other and trigger each other." "One specialist can ask another for help." Handoffs go "directly, without routing through you."
- **Visibility.** "Your team sees every step."
- **Patterns the guides show** (all authors' setups):
  - a front-door Bot that routes questions to specialists and collects their reports;
  - a manager Bot that creates a project, opens a channel and staffs it with other Bots;
  - an operations Bot that holds a daily 1:1 with every other Bot and onboards new ones;
  - shared channels of up to six Bots plus the user (the six is the author's own cap).
- **Creating Bots.** A Bot can propose and create new Bots, after the user agrees (author's setup).

---

## 9. Teams and meetings

### Team Bots

- A Bot can be shared "with specific people or groups". Anyone it is shared with "can ask it questions, hand it work, or loop it into a thread."
- Team Bots work in Slack and Microsoft Teams.
- "Bots learn from everyone they work with."
- "Bots respect each person's permissions" and "check whether information is appropriate to share in a public channel."
- **Settings and analytics:** "Manage what a Team Bot knows and see how your team uses it."

### Meetings

- "Add a Bot to your Google Meet or Zoom call. It listens in, jumps into the chat when it can help, and follows through after." Teams calls are also named.
- The pages describe listening and using the meeting chat. They do not say the Bot speaks.

---

## 10. Templates and the marketplace

### Templates

"A template is a collection of skills, memories, and plugins." It is "a recipe, not a clone": a blueprint from which another user launches their own copy.

| Travels with a template | Does not |
|---|---|
| The Bot's instructions | Secrets |
| Relevant memories | Personal or internal memories (left out by default) |
| Skills | Custom code and scripts |
| First-party plugins and skills | Non-standard plugins, including MCP servers |

- **Publish:** Settings > "Share as Template", choose "Team" or a public link, "Publish", "Copy Link". Nothing is live until published.
- **Install:** open the link, "Add to Grok Bot", review the context and integrations, "Add Bot". The copy is a distinct new Bot; its plugins must be reinstalled and its connections re-made.
- Whether routines are copied is not stated clearly.

### Marketplace and use cases

- The marketplace lists ready-made Bots with an "Add" button, in categories: Engineering, Sales, Marketing, Design, Personal, Recruiting & People, Product, Operations, and "From Grok Bot Team".
- The use-cases page lists 56 Bots across nine categories.
- An "Import Bot" "brings your setup from other agents into Grok Bot."
- Example jobs: Sales Outbound, Talent Scout, Paid Media, Expense Manager, Product Performance, Bug Reproduction, Account Health, Chief of Staff, Inbox Manager, Calendar EA, Presentation Designer, Prototype Builder, Playtest Operator, Sales Call Coach.

---

## 11. Inputs and outputs

No page gives a formal list. What the pages show:

| Direction | Seen in the pages |
|---|---|
| **In** | Text chat; pasted transcripts; uploaded recordings (audio/video of a call); uploaded images and photos; links; screen recordings of a task being taught; live meeting audio; events from connected apps. |
| **Out** | Text replies; documents; presentations (editable link); interactive Pages; screenshots; video clips; live URLs to things it built; pull requests; files uploaded to Drive; emails and calendar invites sent on the user's behalf; rows written to Notion and similar tools; infographics. |

---

## 12. Permissions, approvals and safety

- **Three controls:** "Bots are kept in check by permissions, a reviewer, and allow/block lists."
- **Review agent (Auto Review):** "A separate review agent checks proposed actions and can allow, block, or escalate to you." "Sensitive actions can go through Auto Review before they run."
- **Asking first:** Bots "ask before taking sensitive actions." Many marketplace Bots promise "nothing sends without your yes" and "never spends, signs, or sends without you."
- **Smarter approvals (1.0):** Bots "better understand when you've already authorized an action and when they need to ask."
- **Per-person permissions** in team use (§9).
- **Rules are a prompt.** The stated caveat: "You are trusting the model to follow what you wrote."
- **Privacy:** Cursor SSO and auth, "privacy mode", "training opt-out".
- **Enterprise admin:** "DLP, certs, proxies, and network controls at boot." Admins turn Meetings on in settings.

---

## 13. Plans, usage and availability

| Plan | Price | Includes |
|---|---|---|
| Premium | $30/month | The Bot's own computer, signs into your tools, routines on a schedule, "desktop, iOS, Slack, Teams and more", weekly usage included |
| Super | $100/month | 3.5x Premium usage, priority model access at peak times |
| Ultra | $200/month | 12x Premium usage, earliest access to products and models |
| Teams / Enterprise | Contact sales | Team Bots, admin controls |

- Usage: "weekly usage included, with additional usage billed based on token cost."
- **Team Bots:** Team and Enterprise plans.
- **Meetings:** Team, Enterprise and Ultra plans.
- No numeric usage caps are published.
- **Models:** no model is named for Grok Bot itself. One marketplace Bot mentions "the latest Grok model".

---

## 14. What the pages do not describe

These are absent from every page read. Each is a gap in the public description, not a statement that the feature is missing.

- **No programmatic interface.** No API, SDK, CLI, webhook, or headless mode for driving a Bot is described. The only ways in are the app, Slack/Teams, meetings, routines, and other Bots.
- **No configuration file format.** A Bot is configured through chat and Settings. The only structured settings shown are `Name`, `Title`, `Description`.
- **No self-hosting.** The computer is in the vendor's cloud. The one exception is running Cursor cloud agents on a user's own worker machine.
- **No export format** for a Bot other than a template link.
- **No model choice or parameters** (model, temperature, reasoning effort).
- **No streaming or event format** for a Bot's output.
- **No formal input/output list**, file-size limits, or supported file types.
- **No voice.** Voice is a separate x.ai product in the site navigation. Bots are not said to speak, in meetings or elsewhere.
- **No specification of memory:** what is stored, for how long, per user or per team, or how to read or edit it beyond "Manage what a Team Bot knows".
- **No specification of the review agent's rules** or of allow/block list syntax.
- **No rate, concurrency or runtime limits.**
- **No Windows or Linux desktop client** named.
- **The changelog** (per-version detail) could not be read.
