# Grok Bot: every way to interact with a Bot

Each way a person or a system can reach a Grok Bot, with what works there and what does not. Read on 2026-11-24.

**Sources**

- The product documentation at `https://docs.x.ai/grok-bot/` (28 pages read, listed at the end). This is the main source and is cited as "docs".
- `https://x.ai/bot`, the launch post `https://x.ai/news/grok-bot-1-0`, all 19 guides under `https://x.ai/bot/guides/`, the marketplace and the use-cases page.

**How to read the "not supported" lists**

- **No** means the docs say it does not work there.
- **Not stated** means the docs are silent. It is not evidence either way.

**Correction to `GROK_BOT_FEATURES.md`.** That report was written from the marketing pages only, before the docs were found. The docs contradict it on five points: desktop clients exist for Windows and Linux; an Android app exists; Bots have voice chat and voice memos; a Bot can have its own email address; and a routine can be started by an HTTP webhook. This report is the accurate one on those points.

---

## 1. Overview

A Bot is reached in four kinds of way:

| Kind | Modes |
|---|---|
| **A person messages it** | Desktop app, phone app, voice chat, group chat, X Chat, tagging `@bot` on X, Slack, Microsoft Teams, a meeting's chat |
| **Something else starts it** | Schedule, event trigger (Slack, GitHub, Linear, Sentry, PagerDuty), email to the Bot's address, webhook, another Bot |
| **A person steps into its work** | Approval cards, Secure Forms, viewing and taking over its computer, Teach a task |
| **An admin governs it** | Dashboard, Organization API, inference hooks |

There is **no end-user API, SDK or CLI** for sending a Bot a message and reading its reply. The only programmatic entry is the **webhook trigger** on a routine (§3.3), and it returns no result.

### Capability matrix

| Mode | Text in | Files in | Voice | Approvals answered there | See/take over computer | Bot can message other Bots | Where the result lands |
|---|---|---|---|---|---|---|---|
| Desktop app | Yes | Yes | Yes | Yes | Yes | Yes | The chat |
| Phone app (iOS, Android) | Yes | Yes | Yes | Yes | Yes | Yes | The chat |
| Group chat (in app) | Yes | Yes, to the group; Bot-to-group handoffs are text only | No | **No** | Not stated | Yes | The group |
| X Chat | Yes | Not stated | Not stated | Not stated | Not stated | Not stated | X Chat |
| Tag `@bot` on X | The post | Still photos only | No | No | No | Not stated | **The app**, not X |
| Slack, personal Bot, own DM | Yes | Not stated | Not stated | Yes | No | Yes | **The app chat**, not Slack |
| Slack, personal Bot, channel thread | Yes | Not stated | Not stated | Yes | No | **No** | The thread |
| Slack, Team Bot | Yes | Not stated | Not stated | Yes, by the asker only | No | Yes, to the asker's own Bots | The same thread |
| Microsoft Teams, Team Bot | Yes | Yes, files on posts | Not stated | Yes | No | Not stated | The chat; files by OneDrive in a DM |
| Meeting (Meet, Zoom, Teams) | Meeting chat | No | Listens only | Not stated | No | Not stated | Meeting chat, then a summary to the owner |
| Email to the Bot's address | Yes | Attachments | No | No | No | Not stated | The Bot's chat; replies wait as drafts |
| Webhook | JSON body | No | No | No | No | Not stated | The Bot's chat |
| Schedule / event trigger | The routine's instruction | No | No | No (30-minute expiry) | No | Yes | The Bot's chat |
| Another Bot | Yes | Not stated | No | No (30-minute expiry) | No | Yes | The Bots' chats |

---

## 2. A person messages the Bot

### 2.1 Desktop app

The full client. Everything else is a subset of it.

- **Platforms.** macOS (Apple silicon and Intel), Windows (x64 and Arm64), Linux (x64 and Arm64; `.deb`, `.rpm`, AppImage).
- **Sign-in.** In the browser, with a Cursor account and Cursor SSO.
- **Updates.** A build more than 14 days behind a newer release stops working until it is updated.

**Supported**

- **Chat.** One conversation per Bot in the sidebar. The chat shows each step as the Bot works.
- **Mentions.** `@` mentions a Bot, a group, a routine or an app.
- **Threads.** A reply in a thread applies to one result or one approval request.
- **Attachments.** Attach, drag and drop, or paste. Up to 20 per message. Images 25 MB each and 200 MB in total; documents, audio and other files 250 MB each; video 1 GB each. Accepted kinds: images, audio, video, PDF, plain text, Word, Excel, PowerPoint, CSV, JSON, YAML, source code, HTML, email files, Jupyter notebooks.
- **Dictation and voice chat** (§2.3).
- **Redirecting a run.** A new message while the Bot works redirects the current turn and takes priority over background work. The message "Stop now" ends work at once. It does not undo actions already completed. No stop button is described.
- **Approval cards.** "Allow once", "Always allow", "Deny".
- **Secure Forms and the Vault** (§4.2).
- **The Bot's computer.** A live preview in the right-hand panel; "Take control" to use it (§4.3).
- **Teach a task** (§4.4). Desktop only.
- **Routines.** Created by asking the Bot. Reviewed on the "Details" tab. "Test" runs one. The webhook URL and key are shown here and nowhere else.
- **Results.** Files, images, links and tool results arrive as cards that preview, save or open. Pages open from a card.
- **Settings.** Panes: General, Computer, Vault, Usage & billing, Updates. Bot name, title and description; permission rules under Settings > General > Agent; "Share as Template"; "Connect apps"; memory import.
- **Notifications.** A switch per Bot.
- **Local execution.** A Bot can run commands on the user's own computer: "Ask every time" (default), "Always allow", "Never allow".
- **Phone calls** placed by the Bot (§2.10).
- **Deep links** (§2.11).

**Not supported or limited**

- **Memory screen.** No. A personal Bot has no memory screen; the user asks it in chat what it remembers.
- **Linux.** Hardware security keys are not supported; hardware acceleration is off by default.
- **Biometrics and passkeys.** Fingerprint, Face ID, Windows Hello and passkeys saved on the user's device do not work on the Bot's computer.

### 2.2 Phone app (iOS and Android)

- **Platforms.** iPhone with iOS 18 or later; iPad with iPadOS 18 or later; Android 9 or later.
- **Plans.** Only monthly individual plans can be bought in the app.

**Supported**

- **Chat, group chats, threads, mentions**, including `@everyone` in a group.
- **Attachments.** Take or attach a photo; choose an image or file.
- **Share sheet.** From another app into a chosen Bot, with an optional note. iPhone: a photo, file, link or text. Android: text, photos, videos, audio or PDFs.
- **Dictation, voice chat, voice memos** (§2.3).
- **Siri and Shortcuts (iPhone).** List Bots, Send Message, Start Dictation, Start Voice Chat. The Action Button can start a voice chat.
- **Approval cards.** Tap a card for the full request and its buttons.
- **Notifications** for a result, a question or an approval request.
- **The Bot's computer.** Watch browser or desktop work; take over for a password, a two-factor code or a CAPTCHA; hand back with "I'm done".
- **Routines.** View, pause, delete, see run history, "Add routine" (which starts a chat message beginning "Set up a routine to").
- **Phone tools.** In the user's own chat with a Bot they own, the Bot can use the phone's Calendar, Contacts, Location and Alarms. Reminders and Home are iPhone only. The app must stay open while the Bot uses them.
- **Vault** (Settings > Vault).
- **Phone calls** placed by the Bot (§2.10).

**Not supported or limited**

- **Teach a task.** No.
- **Testing a routine.** No; desktop only.
- **Webhook URL and key.** No; shown on desktop only.
- **Updating or recovering the Bot's computer.** No.
- **Renaming Pages.** No; desktop only.
- **Editing saved cards in the Vault.** No, not yet.
- **Vault CSV import.** No; desktop only.
- **Android notification reading.** Removed in 1.18.0.
- **Attachment size limits on phone.** Not stated separately from desktop.

### 2.3 Voice

Three separate things:

- **Dictation.** Speech becomes text in the composer. Desktop (`Cmd/Ctrl+D`) and phone.
- **Voice chat.** A spoken conversation with one Bot. Started from the empty composer with "Start voice chat". Mute, hang up, show transcript. Voice, speed and language are settable. Desktop and phone.
- **Voice memos.** A Bot can reply with a voice memo that plays in the chat, with a transcript.

**Not supported or limited**

- **Group chats.** No. Voice chat works with one Bot at a time.
- **Two voice chats at once.** No.
- **Phone interruptions.** A phone call, FaceTime or another app taking the audio ends the voice chat. It does not resume.
- **Voice in Slack, Teams, X or meetings.** Not stated. In meetings the Bot listens and writes in the chat; it is not said to speak.

### 2.4 Group chats in the app

A conversation holding the user and 2 to 6 Bots. Bots message each other in it, and a Bot may answer in its own chat instead.

**Not supported or limited**

- **Approval requests.** No; they do not appear in a group.
- **Secret and login requests.** No.
- **Email and Slack drafts.** No.
- **Images in Bot-to-group handoffs.** No; those messages are text only. Images go directly to a Bot.
- **Voice chat.** No.
- **Per-Bot notification switch.** No.
- **Mixing.** A teammate's shared Bot cannot be in a group with Bots that run on the user's own computer.
- **Stopping Bot-to-Bot messaging.** No switch exists. The user tells the Bots not to.
- **Cost.** Every Bot reply in a group counts toward usage.

### 2.5 X Chat

A user chats with their Bots in X Chat, in the X app on iOS and Android and on `x.com`. Rolling out gradually.

- **Requirements.** A plan that includes Grok Bot (Premium, Super or Ultra); the Grok account connected to a Cursor account; a verified email.

**Not supported or limited**

- **Team accounts and company-managed email domains.** No. Personal accounts only.
- **Group chats.** No.
- **Legacy privacy mode.** Blocks the connection.
- **Attachments, approvals, voice, results delivery.** Not stated.

### 2.6 Tagging `@bot` on X

A public post or reply that tags `@bot` hands that post to the user's Bot. Example from the docs: "@bot summarize this thread and send the key points to my Slack."

- **Requirements.** A Grok account connected to the X account, Grok Bot access on it, and at least one Bot the user owns.
- **What the Bot receives.** The post, the post it replies to, and any quoted post, with still photos.
- **What comes back on X.** One short confirmation. The work and its results are in the Bot's conversation in the app.
- **Cost.** A tagged request starts work automatically and spends usage.

**Not supported or limited**

- **Results on X.** No. Only the confirmation is posted.
- **Video or GIF** in the post, the parent or a quoted post. No; `@bot` stays silent.
- **A post that also tags `@grok`.** No; silent.
- **Bots shared with the user.** No; they cannot take tagged requests.
- **Regions.** Not available to accounts based in Australia, the UK, the EU, Iceland, Liechtenstein or Norway.
- **Follow-up, stopping, approvals on X.** Not stated.

### 2.7 Slack

Slack behaves differently for a personal Bot and a Team Bot.

**Setup**

- A Team Bot is shared first, then connected with "Bring to your team's Slack" > "Connect". A Slack workspace admin may have to approve the app.
- Each teammate links their Slack account once per workspace.
- Team Bots require a Team or Enterprise plan and at least one other person on the team.

**What starts the Bot.** A mention in a channel or thread, or a DM.

| Bot | Where it is asked | Can message other Bots | Acts for | Reply lands |
|---|---|---|---|---|
| Personal Bot | The user's own Slack DM with it | Yes | The user | The app chat, not Slack |
| Personal Bot | A channel thread | No | Nobody (shared thread) | The thread; it says it cannot hand off |
| Team Bot | A channel thread or a DM | Yes, to the asker's own Bots | The teammate who mentioned it | The same thread |

**Supported**

- **Questions, handed-off work, being looped into a thread.**
- **Approvals in the thread.** "Allow once", "Deny", "View details". Only the person who asked can answer.
- **Usage.** A Team Bot spends the usage of the person who asked.
- **Slack as a routine trigger** (§3.2): on a mention, a word or phrase, a reaction, or any message; one channel or all of Slack.
- **Posting unprompted.** Through routines; the launch post shows a Bot creating a channel.
- **Judgement about audience.** Bots check whether information is appropriate for a public channel.

**Not supported or limited**

- **Slack guests, Slack Connect channels, other workspaces.** No; they cannot use a Team Bot.
- **"Always allow".** Not offered in the Slack card.
- **The Bot's computer, Teach a task, voice.** Not available from Slack.
- **Vault.** A user's Vault is not available on Team Bots.
- **File attachments in Slack.** Not stated.
- **Sending as the user.** The separate Slack *app connection* posts as the connected Slack user, not as a bot. Such messages are drafts needing approval by default.
- **Policy blocks.** Under an inference hook, the Bot replies that the organization's policy blocked the conversation.

### 2.8 Microsoft Teams

Team Bots only.

**Setup**

- An admin enables "Enable Microsoft Teams" with a Microsoft Teams or Global administrator account.
- The Bot's owner or a Manager chooses "Connect Microsoft Teams". Approval can take up to an hour.
- On first message, each user gets a "Link Account" card only they can see.

**What starts the Bot.** A mention in a channel of a team it joined, being added to a group chat, or a direct message.

**Supported**

- **Reading files** attached to posts.
- **Sending files** through the user's OneDrive.
- **Approvals** before risky actions, as in Slack. It spends the asker's usage.

**Not supported or limited**

- **Files in a channel or group chat.** No. Asked for a file there, the Bot asks the user to message it directly.
- **Personal Bots in Teams.** Not stated; setup requires a published Team Bot.
- **Voice, the Bot's computer, routines started from Teams, bot-to-bot handoffs.** Not stated.

### 2.9 Meetings (Google Meet, Zoom, Microsoft Teams calls)

**How a Bot joins.** The user pastes the meeting link into the Bot's chat and asks it to join; the user approves a card; the host admits the Bot. It appears as "Bot name (first name's Bot) - transcribing". A routine can also join meetings.

**Supported**

- **Listening and note-taking.**
- **Answering questions in the meeting chat.** On Zoom, a private chat message gets a private answer.
- **Follow-up.** A summary and action items go to the user afterwards.

**Not supported or limited**

- **Speaking.** Not stated. Only listening and the chat are described.
- **Plans.** Team, Enterprise, Ultra and Super only. (The launch post omits Super.)
- **Admin switch.** With "Meetings" off, Bots cannot join or read meetings.
- **Files, approvals inside the meeting, screen sharing, private answers on Meet or Teams.** Not stated.

### 2.10 Phone calls placed by the Bot

Outbound only: the Bot calls a number for the user. Available from the desktop and phone apps.

- **Flow.** The Bot shows whom it will call; the user chooses "Call" or "Cancel". During the call the user can end it, "Take over" to join, or answer a question the Bot puts to them. Afterwards the card shows the length, a playable recording, and on desktop a transcript and a recording download.

**Not supported or limited**

- **Outside the US.** No.
- **Calling a Bot by phone.** Not stated; no inbound number is described.

### 2.11 Links and shortcuts into the app

These open the app; none sends a message by itself.

- **Prompt link.** `grokbot://app/v1/prompt?id=<bot-id>&text=<message>`. Opens the Bot's chat with `text` in the composer. `text` is at most 1,000 characters, is added only if the composer is empty, and is not sent until the user sends it. Without `id` the primary Bot is used.
- **Plugin link.** `https://x.ai/bot/plugin/<plugin-id>`, or `grokbot://app/v1/plugin/add?id=<plugin-id>` for a team-only plugin.
- **Template link.** Opens a preview on `x.ai` with "Add to Grok Bot".
- **Share sheet and Siri Shortcuts.** See §2.2.

**Not supported.** A link that starts a run without the user pressing send.

### 2.12 Not offered

- **A web chat client.** The docs name the desktop app and the phone app only. Web pages exist for sign-in, billing, templates, plugins and shared Pages.
- **An end-user API, SDK or CLI.**
- **Other messengers** (WhatsApp, iMessage, Telegram, Discord, SMS). Not named anywhere.

---

## 3. Something else starts the Bot

All of these are **routines**: an instruction the Bot owns, plus a trigger. Common rules:

- **Created by asking the Bot** that will own it. No form or file format is described.
- **Limits.** 50 routines per Bot. Schedules at least five minutes apart. 200 runs an hour per routine and 300 an hour across all routines; beyond that a run fails with `Automation run rate limit exceeded`.
- **Runs in the cloud**, with the laptop closed. Each run spends usage.
- **History.** The 20 most recent runs per routine.
- **Approvals expire.** An approval requested by a routine, a trigger or another Bot expires after about 30 minutes. One requested in chat waits indefinitely.
- **Email from a routine** always waits as a draft.
- **Team Bot routines** are visible to "Only me" or "Everyone with access".
- **Policy blocks.** Under an inference hook, a routine stops and records the block.

### 3.1 Schedule

A time and a recurrence, in the time zone set under Settings > Bot > Timezone. A new routine waits for its next scheduled time; it does not run on creation.

### 3.2 Event triggers

- **Sources.** A Slack message; a GitHub, Linear, Sentry or PagerDuty event; an email to the Bot's own address; a webhook call.
- **Slack options.** When the Bot is mentioned, when a word or phrase is mentioned, when the user reacts to a message, or on any message. One channel or all of Slack.
- **Only new activity counts.** Events from before the trigger existed start nothing.
- **Separate connection.** Cursor account integrations start these routines. They are separate from the Slack or GitHub apps and may need their own connection.

**Not stated.** The full option list for GitHub, Linear, Sentry and PagerDuty, and what event data the Bot receives.

### 3.3 Webhook

The one programmatic way in.

- **Setup.** The user asks the Bot to add a webhook trigger to a routine. The routine's "Webhook" section, on desktop, shows the URL ("POST to"), the secret ("key") and the header.
- **Request.** `POST` to the URL with `Authorization: Bearer <key>` and, optionally, a JSON body.
- **What the Bot receives.** The body, together with the routine's instruction.
- **Response.** `200` means the call was accepted and a run started, not that the Bot finished. Any other response means no run started.

**Not supported or limited**

- **A result.** No. Nothing is returned or called back; the result appears in the Bot's chat.
- **Streaming, run status, cancellation by HTTP.** Not stated.
- **Body size limit, response body, key rotation.** Not stated.
- **Free-form conversation.** The body is input to a fixed routine instruction, not a chat turn.

### 3.4 Email to the Bot's address

- **Address.** A user can claim one address at `mail.grokbot.com`, in chat or under "Connect apps" > "Email".
- **Inbound.** Mail to the address can start a routine, including one that watches for mail from anyone.
- **Outbound.** Email the user asks for in chat is sent once Auto Review allows it. Up to 50 recipients; up to 10 attachments, 10 MB each and 25 MB in total.

**Not supported or limited**

- **An inbox view or new-mail notifications.** No.
- **A team address for a Team Bot.** No.
- **Teams.** "Agent Email" is off by default on a team; an admin turns it on.
- **Policy blocks.** Under an inference hook, email gets no reply.

### 3.5 Another Bot

A Bot sends an asynchronous message to another Bot. The receiver wakes, handles it, and can reply later.

- **Shared ground.** All of a user's Bots share one cloud computer, its sign-ins, and files under `/workspace`. Each Bot has its own screen and its own memory.
- **Cost.** Messages between Bots count toward usage.

**Not supported or limited**

- **From a personal Bot in a Slack channel thread.** No (§2.7).
- **An off switch.** No.
- **Loop or depth limits.** Not stated.

---

## 4. A person steps into the Bot's work

### 4.1 Approvals

- **Who decides.** Permissions, allow and block rules written in plain language, and Auto Review, a separate review agent that allows, blocks or escalates.
- **The card.** "Allow once", "Always allow", "Deny".
- **Drafts.** Outgoing email and Slack messages are drafts needing approval by default. "Disable drafts for Bot" turns that off per Bot, owner only.

| Where | Approval behaviour |
|---|---|
| Desktop, phone | Full card |
| Slack thread | "Allow once", "Deny", "View details"; asker only |
| Teams | Asks before risky actions, as in Slack |
| Group chat | Not shown |
| Routine, trigger, another Bot | Expires after about 30 minutes |
| X, meetings, email, webhook | Not stated |

### 4.2 Secure Forms and the Vault

- **Secure Form.** When a page needs something only the user should give, the Bot shows a form in the chat. The answers go straight into the page and are never shown to the Bot.
- **Vault.** Saved secrets and cards, under Settings > Vault on desktop and phone.

**Not supported or limited**

- **Team Bots.** No; the user's Vault is not available to them.
- **Group chats.** No; secret and login requests do not appear.
- **Slack, Teams, meetings.** Not stated.

### 4.3 Viewing and taking over the Bot's computer

- **What it is.** A persistent cloud computer with a browser, a command line and a filesystem. Each Bot has its own screen, watchable live.
- **Takeover.** For single sign-on or a passkey, a CAPTCHA, an approval on another device or a QR code, or a bank card verification. The user completes the blocked step and tells the Bot to continue.
- **Where.** Desktop ("Take control") and phone ("I'm done" to hand back).

**Not supported.** From Slack, Teams, X, meetings or email.

### 4.4 Teach a task

The user records a task on the Bot's computer and the Bot turns it into a skill.

- **Where.** Desktop only, from a one-to-one Bot conversation and its computer view.
- **Limits.** Up to ten minutes. No microphone audio. Being enabled gradually.

**Not supported.** Phone, group chats, and every other surface. A skill can still be taught anywhere by writing it out.

---

## 5. An admin governs it

None of these sends a Bot a message.

- **Dashboard switches** (Team and Enterprise): Meetings, Microsoft Teams, Agent Email, Slack Workspace Access (on by default), Execution on Local Computer, Enforce Auto-review (Enterprise only).
- **Organization API.** With a key of scope `admin:*`, an organization admin recreates, terminates or deletes members' computers, or deletes all their Grok Bot data. The endpoints are documented on `cursor.com`, not in these docs. An Admin API enables Grok Bot and manages capabilities.
- **Inference hooks** (Enterprise, beta). An HTTPS endpoint the admin supplies receives every model request (`prompt`) and, optionally, every tool call before it runs (`tool_call`), signed with Standard Webhooks headers. It answers `{"action":"allow"|"deny", …}`. It does not cover system prompts, tool definitions, Auto Review, memory calls or Cloud Agents.

---

## 6. Where results appear

| Output | Delivered |
|---|---|
| Text, files, links, tool results | Cards in the chat (app); the thread (Slack, Teams) |
| Images and video | In the conversation, on request only; kept in the Library. Video is 1 to 15 seconds, MP4 |
| Pages | A card with "Open"; shareable to invited people, the team, or anyone with the link (`pages.grok.bot`). Not available on Enterprise teams yet |
| Voice memos | Played in the chat |
| Files in Teams | The user's OneDrive, in a direct message |
| Tagged request on X | The app; X gets a confirmation only |
| Webhook, schedule, event, email run | The Bot's chat, plus a notification |
| Meeting | The meeting chat during; a summary to the owner after |

---

## 7. Plan and admin requirements

| Mode | Requirement |
|---|---|
| Grok Bot at all | Premium, Super or Ultra on the X Subscription, a legacy paid Cursor plan, or Team/Enterprise. Plus does not include it |
| Team Bots, and so Slack channels and Teams | Team or Enterprise, with at least one other team member |
| Microsoft Teams | An admin enables it |
| Meetings | Team, Enterprise, Ultra or Super; admin switch on teams |
| X Chat | Personal accounts on a plan with Grok Bot |
| Tagging `@bot` | Outside AU, UK, EU, Iceland, Liechtenstein, Norway |
| Phone calls | US only |
| Agent email on a team | An admin enables it |
| Inference hooks, computer operations | Enterprise |

---

## 8. Open points

- **No way to get a reply programmatically.** The webhook starts a run and returns `200`. Whether any supported path returns a run's output to a caller is not stated.
- **Webhook details.** Body size limit, response body, key rotation.
- **Trigger payloads.** What a Bot receives from a GitHub, Linear, Sentry or PagerDuty event.
- **Slack file handling** and whether a personal Bot can be used in Teams.
- **Speech in meetings.**
- **The Cursor relationship.** Accounts, SSO, the dashboard, the Organization API and the update feed are all Cursor's. The docs do not explain it.

**Docs pages read** (all under `https://docs.x.ai/grok-bot/` unless noted): overview, `get-started`, `mobile`, `chat-and-collaboration`, `skills-routines-and-automations`, `approvals-security-and-privacy`, `teams-and-enterprises`, `microsoft-teams`, `meetings`, `computer-and-apps`, `team-bots`, `create-a-team-bot`, `vault`, `settings-and-notifications`, `faq`, `tag-on-x`, `deep-links`, `files-and-results`, `bots`, `how-grok-bot-works`, `inference-hooks`, `computers`, `deployment`, `identity-and-access`, `troubleshooting`, and `https://docs.x.ai/x-subscription/grok-bot`.

**Not read:** `use-cases`, `security`, `security-faq`, `private-networks`, `proxies`, the Organization API reference on `cursor.com`, and the per-plugin pages under `x.ai/bot/plugin/`.
