# X API: direct messages and X Chat

What the X API offers for sending and receiving private messages, from the official docs at `https://docs.x.com`. Read on 2026-11-24.

The X API has **two separate messaging stacks**, and a third API for receiving either in real time:

| Stack | Paths | Messages | Docs' own description |
|---|---|---|---|
| **Direct Messages** (legacy) | `/2/dm_events`, `/2/dm_conversations/…` | Plaintext | "the legacy, unencrypted DM system" |
| **X Chat** | `/2/chat/…`, `/2/users/{id}/public_keys` | End-to-end encrypted | "the new, encrypted messaging stack, or XChat" |
| **X Activity API** | `/2/activity/…`, `/2/webhooks` | Events for both (`dm.*`, `chat.*`) | Real-time delivery by stream or webhook |

**"Not stated"** below means the docs are silent. It is not evidence either way.

The docs do not say whether the two stacks share an inbox, or whether the legacy endpoints can see X Chat conversations.

---

## 1. Common ground

- **Base URL.** `https://api.x.com`.
- **Auth.** User context only, for both stacks. OAuth 2.0 Authorization Code with PKCE is the recommended flow. "App-Only authentication is not supported. All Direct Messages are private."
- **OAuth 1.0a.** "Legacy support" on the Direct Messages endpoints. Not stated for X Chat.
- **Scopes.** `dm.read` ("Read all your Direct Messages"), `dm.write` ("Send and manage your Direct Messages"), with `tweet.read` and `users.read`. X Chat media needs `media.write`.
- **Response envelope.** `{data, errors, includes, meta}`.
- **Errors.** Problem objects: `type` (a URI under `https://api.x.com/2/problems/`), `title`, `detail`, optional `status`. A rate-limit `429` carries `{"errors":[{"code":88,"message":"Rate limit exceeded"}]}`.
- **Rate-limit headers.** `x-rate-limit-limit`, `x-rate-limit-remaining`, `x-rate-limit-reset`.
- **Pricing.** "The X API uses pay-per-usage pricing with no subscriptions." Rate limits and billing are separate.

---

## 2. Direct Messages (legacy, plaintext)

### 2.1 Endpoints

| # | Method and path | Purpose |
|---|---|---|
| A | `GET /2/dm_events` | Recent events across all conversations |
| B | `GET /2/dm_conversations/with/{participant_id}/dm_events` | Events of the one-to-one conversation with a user |
| C | `GET /2/dm_conversations/{id}/dm_events` | Events of a conversation by id |
| D | `GET /2/dm_events/{event_id}` | One event |
| E | `POST /2/dm_conversations/with/{participant_id}/messages` | Send to a user (creates the one-to-one conversation if needed) |
| F | `POST /2/dm_conversations/{dm_conversation_id}/messages` | Send to an existing conversation |
| G | `POST /2/dm_conversations` | Create a group conversation with a first message |
| H | `DELETE /2/dm_events/{event_id}` | Delete an event the user owns |
| I | `GET /2/dm_conversations/media/{dm_id}/{media_id}/{resource_id}` | Download attached media as raw bytes |
| – | `POST /2/users/{id}/dm/block`, `…/dm/unblock` | Block or unblock a user's DMs |

### 2.2 Reading (A to D)

**Query parameters (A, B, C)**

- **`max_results`.** Integer, 1 to 100, default 100.
- **`pagination_token`.** From `meta.next_token`. Events are returned newest first.
- **`event_types`.** Any of `MessageCreate`, `ParticipantsJoin`, `ParticipantsLeave`. Default: all three.
- **`dm_event.fields`.** `attachments`, `created_at`, `dm_conversation_id`, `entities`, `event_type`, `id`, `text`.
- **`expansions`.** `attachments.media_keys`, `participant_ids`, `referenced_posts`, `sender_id`.
- **`user.fields`, `post.fields`, `media.fields`.** For the expanded objects.

**The DM event object**

| Field | Content |
|---|---|
| `id` | Event id |
| `event_type` | `MessageCreate`, `ParticipantsJoin` or `ParticipantsLeave` |
| `text` | Message text |
| `sender_id` | Sender's user id |
| `dm_conversation_id` | Conversation id |
| `created_at` | Timestamp |
| `attachments` | `media_keys[]`, `card_ids[]` |
| `entities` | `urls[]`, `mentions[]`, `hashtags[]`, `cashtags[]`, each with `start` and `end` |
| `referenced_posts` | `[{id}]` for a Post shared in the message |
| `participant_ids` | For join and leave events |

- **Defaults.** A `MessageCreate` returns `id`, `event_type`, `text` unless more fields are requested.
- **Meta.** `{next_token, previous_token, result_count}`.
- **History.** "Events from up to 30 days ago are available through these endpoints."

### 2.3 Sending (E, F, G)

**Body for E and F** (no other properties are accepted):

```json
{ "text": "Hello", "attachments": [ { "media_id": "1234567890" } ] }
```

- At least one of `text` and `attachments` is required. `text` has minimum length 1.
- **Response `201`.** `{"data": {"dm_conversation_id": "…", "dm_event_id": "…"}}`.

**Body for G:**

```json
{ "conversation_type": "Group", "participant_ids": ["…", "…"], "message": { "text": "Hi" } }
```

- **`conversation_type`.** `Group` only.
- **`participant_ids`.** 2 to 49 ids.

### 2.4 What a message can carry

- **Text.** Maximum length not stated; a `dm-text-too-long` problem exists.
- **Media.** One photo, video or GIF. "Only one attachment per message is supported." The sender uploads it first through the general media upload endpoint; it is usable for 24 hours after upload. Size limits not stated.
- **Posts.** A Post URL in the text shares the Post; it comes back under `referenced_posts`.
- **Links, mentions, hashtags.** Parsed into `entities`.

### 2.5 Conversations

- **One-to-one id.** `{smaller_user_id}-{larger_user_id}`, for example `1234567890-0987654321`. Always exactly two participants.
- **Group id.** A 15 to 19 digit number.
- **Group membership.** Changes surface as `ParticipantsJoin` and `ParticipantsLeave` events. No endpoint adds or removes members.

### 2.6 Not supported in Direct Messages

- **Reactions, edits, read receipts, typing indicators, voice messages, calls.** No endpoint; not stated.
- **Quick replies, buttons, welcome messages, custom profiles.** Not accepted; the request body takes `text` and `attachments` only.
- **More than one attachment.** No.
- **Naming a group or changing its members.** No.
- **Delete for everyone.** No. Delete removes the event for the authenticated user.
- **History older than 30 days.** No.
- **Encrypted conversations.** Problem types `encryption-required` and `unencrypted-send-not-allowed` exist: a plaintext send to an encrypted chat is refused.

### 2.7 Limits and price

| Endpoint | Per user | Per app |
|---|---|---|
| All four `GET` event endpoints | 15 / 15 min | – |
| Each `POST` send or create | 15 / 15 min and 1,440 / 24 h | 1,440 / 24 h |
| `DELETE /2/dm_events/:id` | 300 / 15 min and 1,500 / 24 h | 4,000 / 24 h |
| DM block, unblock | 10 / 15 min and 400 / 24 h | 25 / 15 min and 1,000 / 24 h |

- **Read.** $0.010 per DM event.
- **Create.** $0.015 per request.

At 15 reads per 15 minutes, polling can run at most once a minute.

---

## 3. X Chat (encrypted)

"The X Chat API lets you send and receive encrypted direct messages on X." "Message bodies are encrypted on the client; X routes ciphertext and cannot read plaintext content." Messages are also signed.

The API moves ciphertext only. All encryption, decryption, signing and verification is done by the caller with the **Chat XDK**, a library that makes no HTTP calls.

### 3.1 Endpoints

| # | Method and path | Purpose | Scopes |
|---|---|---|---|
| 1 | `GET /2/chat/conversations` | List conversations | `dm.read`, `users.read` |
| 2 | `GET /2/chat/conversations/{id}` | One conversation | `dm.read`, `users.read`, `tweet.read` |
| 3 | `GET /2/chat/conversations/{id}/events` | History (ciphertext events) | `dm.read`, `users.read`, `tweet.read` |
| 4 | `POST /2/chat/conversations/{id}/messages` | Send a message | `dm.write`, `users.read`, `tweet.read` |
| 5 | `POST /2/chat/conversations/{id}/keys` | Add or rotate the conversation key | `dm.write`, `users.read`, `tweet.read` |
| 6 | `POST /2/chat/conversations/group/initialize` | Reserve a group id | `dm.write` |
| 7 | `POST /2/chat/conversations/group` | Create a group | `dm.write`, `users.read`, `tweet.read` |
| 8 | `POST /2/chat/conversations/{id}/members` | Add group members | `dm.write`, `users.read`, `tweet.read` |
| 9 | `POST /2/chat/conversations/{id}/typing` | Send a typing indicator | `dm.write`, `users.read`, `tweet.read` |
| 10 | `POST /2/chat/conversations/{id}/read` | Mark read | `dm.write`, `users.read`, `tweet.read` |
| 11 | `POST /2/users/{id}/public_keys` | Register the user's public keys | `dm.write`, `users.read`, `tweet.read` |
| 12 | `GET /2/users/{id}/public_keys` | A user's public keys | `dm.read`, `users.read`, `tweet.read` |
| 13 | `GET /2/users/public_keys?ids=…` | Public keys of 1 to 100 users | `dm.read`, `users.read`, `tweet.read` |
| 14 | `POST /2/chat/media/upload/initialize` | Start a media upload | `media.write` |
| 15 | `POST /2/chat/media/upload/{id}/append` | Upload a segment (index 0 to 999) | `media.write` |
| 16 | `POST /2/chat/media/upload/{id}/finalize` | Finish the upload | `media.write` |
| 17 | `GET /2/chat/media/{id}/{media_hash_key}` | Download encrypted media bytes | `media.write` |

### 3.2 Conversation ids

- **One-to-one.** Two user ids, written `A:B` in events and `A-B` in paths. For a one-to-one conversation the path also accepts the recipient's bare user id.
- **Group.** `g` followed by digits.

### 3.3 Listing and reading

**`GET /2/chat/conversations`**

- **`max_results`.** 1 to 1000, default 200.
- **Filters.** `unread_only`, `type` (`direct` or `group`), `is_archived`, `is_muted`, `participant_ids` (1 to 10), `participant_query` (an `@username` or a word of a display name), `start_time`, `end_time`.
- **`chat_conversation.fields`.** Include `type`, `unread`, `last_read_event_id`, `newest_event_id`, `message_ttl_ms`, `group_name`, `group_description`, `group_avatar_url`, `pending_member_ids`, `is_archived`, `is_muted`, `created_at`, `updated_at`.
- **`expansions`.** `participant_ids`, `member_ids`, `admin_ids`.
- **Meta.** `{has_message_requests, has_more, next_token, result_count}`.

**`GET /2/chat/conversations/{id}/events`**

- **`max_results`.** 1 to 100, default 10. Newest first.
- **Event fields.** `id`, `conversation_id`, `sender_id`, `created_at`, `encoded_event` (the ciphertext, always present), `conversation_key_version`, `conversation_token`, `message_event_signature`, `previous_id`, `is_trusted`.
- **Meta.** `{conversation_key_events, has_more, next_token, result_count}`.
- **Unread.** Events with an id above `last_read_event_id`.
- **Paging matters.** Key-change events can sit on older pages, so a full decrypt pages through everything.

### 3.4 Sending

**`POST /2/chat/conversations/{id}/messages`**

```json
{
  "message_id": "<uuid from the Chat XDK>",
  "encoded_message_create_event": "<base64>",
  "encoded_message_event_signature": "<base64>"
}
```

- **Response `201`.** `{"data": {"encoded_message_event": "<base64>"}}`.
- **Retries** resend the same payload with the same `message_id`.
- **Plaintext fallback.** A `content` object (`{"message": {"text": "…"}}`, or `reaction_add` / `reaction_remove` with `event_id` and `emoji`) is accepted only when the recipient's `receives_your_chat` is `unencrypted`.
- **`400 encryption-required`.** "means the chat can be encrypted or is a group, so send it encrypted instead."
- **`403 recipient-not-messageable`.** The recipient has a closed inbox, is a system account, or has blocked the sender.

### 3.5 The flow

**Set up the account's keys, once**

1. `generate_keypairs()` in the Chat XDK.
2. `POST /2/users/{id}/public_keys`. The response gives `public_key_version` and `juicebox_config`.
3. Store the private keys one of two ways:
   - **Passcode backup.** `setup(passcode)`, later `unlock(passcode)`. Twenty wrong passcodes destroy the backup share.
   - **Key blob.** `export_keys()` gives an opaque secret blob, restored with `import_keys()`. Native bindings only. The docs: "Server and bot samples often use a key blob."
4. `set_identity(user_id, public_key_version)`.

**Start a one-to-one conversation**

1. Check the recipient's `receives_your_chat`: `encrypted`, `unencrypted` or `none`.
2. Fetch both parties' public keys (endpoint 13).
3. `verify_key_binding(…)` on each record.
4. `prepare_conversation_key_change(…)`, which returns the raw conversation key, its version, one wrapped copy per participant, and an action signature.
5. `POST /2/chat/conversations/{recipient_id}/keys`. The response carries the canonical `conversation_id`.

**Send.** `encrypt_message(conversation_id, text, …)`, then endpoint 4.

**Receive.** A `chat.received` event (§4) or a poll of endpoint 3; then `set_signing_keys(…)`, `decrypt_events(…)` for any key change, and `decrypt_event(encoded_event)`.

**State the caller keeps.** The key blob or passcode, the user id, the public key version, and optionally a cache of conversation keys.

### 3.6 What a decrypted message carries

- **Text.** `content.content_type` is `Text`; `content.text` is the text.
- **Identity.** `sender_id`, `conversation_id`, `message_id`, `verified` (the signature check).
- **Attachments.** `media_hash_key`, `filename`, `filesize_bytes`, `width`, `height`. Images and files. The media is encrypted separately with the XDK's stream functions and moved with endpoints 14 to 17.
- **Replies.** `encrypt_reply(…)` embeds a signed preview of the message replied to.
- **Reactions.** `encrypt_add_reaction(…)`, `encrypt_remove_reaction(…)`.
- **Expiry.** `ttl_msec` on a message or a group; `message_ttl_ms` on a conversation.

### 3.7 Groups

- **Create.** Endpoint 6 for the id, `prepare_group_create(…)`, then endpoint 7 with `group_members`, optional `group_admins`, `group_name`, `group_description`, `group_avatar_url`, `ttl_msec`.
- **Add members.** `prepare_group_members_change(…)`, then endpoint 8. Adding a user with a pending join request approves it.

### 3.8 Typing and read state

- **Send typing.** Endpoint 9, no body.
- **Mark read.** Endpoint 10, with an optional signed receipt naming the event read up to.
- **Own read state.** `unread`, `last_read_event_id`, `newest_event_id` on the conversation.

### 3.9 Chat XDK

| Language | Package | Notes |
|---|---|---|
| Python | `chatxdk` (import `chat_xdk`) | Python 3.10+ |
| TypeScript | `@xdevplatform/chat-xdk` | Node 18+; WASM included; no key-blob export or import |
| Go | `github.com/xdevplatform/chat-xdk/go/chatxdk` | Go 1.21+; prebuilt static libraries, Linux glibc and musl included; needs a C compiler |
| Rust | `chat-xdk-core` | Git dependency, not on crates.io; needs a patched `thrift` |
| C# | `XDevPlatform.ChatXdk` | .NET 8+ |
| Java | `com.x:chatxdk` | JDK 17+ |

Version numbers and licence are not stated in the docs.

### 3.10 Not supported or not stated in X Chat

- **Reading plaintext without the XDK.** No. The API returns ciphertext only.
- **Removing a member or leaving a group.** No endpoint is documented.
- **Deleting a message.** Not stated.
- **Editing.** Edit events exist; the send path is not stated.
- **Sending an encrypted reaction.** The XDK builds one; how it is posted is not stated.
- **Receiving typing indicators or other people's read receipts.** Not stated; no such `chat.*` event exists.
- **Voice messages and calls.** Not stated.
- **Maximum text length, media types and sizes, group size.** Not stated.
- **Rate limits.** Not stated; no `/2/chat/` row is in the rate-limit table.
- **Price of the REST calls.** Not stated; no chat row is in the price list.
- **History retention.** Not stated.
- **Using an identity created in the X app.** `unlock(passcode)` is described for an identity that "completed first-time setup before". Whether keys created in the X app unlock this way is not stated.
- **Bots and automated accounts.** No X Chat rule is stated. `recipient-not-messageable` names "system account" as a refusal reason.

---

## 4. Receiving in real time: X Activity API

The older Account Activity API "is being deprecated" in favour of this one.

### 4.1 Events

| Event | Meaning | Price |
|---|---|---|
| `dm.received` | A legacy DM arrived | $0.010 |
| `dm.sent` | A legacy DM was sent | Not billed |
| `dm.read` | A legacy read receipt | Not billed |
| `dm.indicate_typing` | A legacy typing indicator | Not billed |
| `chat.received` | An X Chat message arrived | $0.010 |
| `chat.sent` | An X Chat message was sent | Not billed |
| `chat.conversation_join` | A user joined an encrypted conversation | Not billed |

All seven are private events: the monitored user authorizes the app with OAuth 2.0 user context.

### 4.2 Setup

1. **Subscribe.** `POST /2/activity/subscriptions` with `event_type`, `filter.user_id`, and optionally `tag`, `webhook_id`, `expires_at`. Also `GET`, `PUT /:id`, `DELETE /:id`.
2. **Receive**, one of:
   - **Stream.** `GET /2/activity/stream`, a persistent HTTP connection, app Bearer token. `backfill_minutes` 0 to 5. Two connections per app.
   - **Webhook.** `POST /2/webhooks` with `{"url": …}`. The URL is public HTTPS with no port. The server answers a `GET` carrying `crc_token` with `{"response_token": "sha256=<base64 HMAC-SHA256 of the token with the client secret>"}`, and answers each `POST` with `200` within 10 seconds. Each `POST` is signed in `X-Twitter-Webhooks-Signature-OAuth2`.

- **Subscription limits.** 1,500 self-serve; 75,000 Enterprise; 150,000 Partner.
- **Rate limits.** Stream 450 / 15 min; subscriptions 500 / 15 min; webhooks 450 / 15 min.

### 4.3 Payloads

- **Envelope.** `event_uuid`, `event_type`, `created_at`, `filter`, `tag`, `payload`, `includes`.
- **`dm.received`, `dm.sent`.** `direct_message_events[]` with `id`, `created_timestamp`, and `message_create {sender_id, target.recipient_id, message_data.text}`. The text is plaintext.
- **`chat.received`, `chat.sent`.** `conversation_id`, `sender_id`, `encoded_event` (ciphertext), `conversation_key_version`, optional `conversation_key_change_event`, `message_event_signature`. A filter `conversation_type` takes `direct` or `group`.

### 4.4 Delivery

- **Duplicates happen.** The docs say to deduplicate by event id.
- **Latency, retries, ordering, stream heartbeat and reconnection.** Not stated.

---

## 5. Message granularity

**Every mechanism delivers whole messages.** A DM event and a chat event are each one complete message; `encoded_event` is one complete ciphertext. No endpoint, stream or webhook delivers partial or incremental message text.

A program reading a correspondent's reply through this API sees each message when it is sent, in full, and nothing before.

---

## 6. Rules on automated messaging

From the developer guidelines:

- **"Send DMs | Only after user DMs you first. Easy opt-out required."**
- **Prohibited:** bulk DMs, and automatic DMs to new followers ("Unsolicited, even to followers").
- **Allowed:** "App responds to DMs with helpful info" ("User-initiated").
- **Automated accounts** carry the "Automated" profile label, name their operator in the bio, and use only the official API.
- **Penalty.** "Violations can result in app suspension, API access revocation, or permanent account bans."

Who may message whom is not set out. The problem types name the gates without defining them: `dm-recipient-not-accepting`, `dm-sender-dms-closed`, `dm-recipient-blocks-sender`, `dm-sender-not-verified`, `dm-sender-phone-required`, `dm-safety-limited`, `message-request-not-supported`, `dm-request-in-progress`, `xchat-setup-required`, `recipient-xchat-setup-required`.

---

## 7. Side by side

| | Direct Messages | X Chat |
|---|---|---|
| Content on the wire | Plaintext | Ciphertext; Chat XDK needed |
| Send text | Yes | Yes |
| Media | One photo, video or GIF | Images and files, encrypted |
| Replies | No | Yes |
| Reactions | No | Built by the XDK; send path not stated |
| Typing indicator | Receive only (`dm.indicate_typing`) | Send only |
| Read receipts | Receive only (`dm.read`) | Send only; own read state readable |
| Groups | Create only, 2 to 49 others | Create, add members |
| List conversations | No (only events) | Yes, with filters and unread state |
| History | 30 days | Not stated |
| Real-time receive | `dm.received` | `chat.received` |
| Partial message text | No | No |
| Rate limits | 15 / 15 min; 1,440 sends / 24 h | Not stated |
| Price | $0.010 read, $0.015 send | Event $0.010; REST not stated |

---

## 8. Docs inconsistencies noted

- The DM guides say delete is "not yet available in v2"; the reference has `DELETE /2/dm_events/{event_id}`.
- The DM guide says one attachment; the schema's `attachments` array has no maximum.
- The DM guide says `referenced_tweets`; the schema says `referenced_posts`.
- Webhook re-validation is "every 30 minutes" on one page and "every hour" on another.
- `POST /2/chat/conversations/{id}/keys`: the guide says `action_signatures` is required; the schema does not.
- `POST /2/chat/conversations/{id}/members`: the response is typed as a Post object.
- The media guide says finalize yields `media_hash_key`; the schema returns it from initialize.

## 9. Pages

**Read:** the Direct Messages lookup and manage introductions and integration guides and all nine DM reference pages; every page under `/xchat/` except the WASM guide; all Chat reference pages under `/x-api/chat/` and the two public-key pages under `/x-api/users/`; the X Activity API introduction, quickstart, stream and event-payload pages; the webhooks introduction and quickstart; the deprecated Account Activity introduction; rate limits; pricing; developer guidelines.

**Failed:** `help.x.com/en/using-x/about-chat` and `…/encrypted-direct-messages` (403).

**Not read:** `docs.x.com/developer-terms/policy`, the DM quickstarts and migration guides, the DM block reference pages, the media upload reference, `docs.x.com/xchat/building-ui-apps-with-wasm`, and the `xdevplatform/chat-xdk` repository.
