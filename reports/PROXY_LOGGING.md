# A logging proxy inside the container

Written 2026-11-25. This report covers one arrangement: the Grok Bot
desktop app runs in a container, and a proxy in the same container
decrypts and reads the app's own network traffic before the app receives
it. The proxy forwards everything unchanged. It sends nothing of its own.

The other ways of reading the app are compared in
[READING_APP.md](READING_APP.md). The question of a program, not a
person, using the app is in
[SUPERINTELLIGENCE_USING_APP.md](SUPERINTELLIGENCE_USING_APP.md).

This is not legal advice and I am not a lawyer. Two kinds of statement
appear below and are kept apart:

- **From the pages.** What the vendor's terms and documentation say, with
  short exact quotes. The pages are listed in section 10.
- **General law.** How the law ordinarily treats the act. This is my
  general knowledge, not read for this report and not checked against a
  lawyer or a court decision.

## 1. Conclusion

- **No clause forbids a proxy, and none permits one for this purpose.**
  The terms never mention proxies, interception or inspection.
- **The vendor knows its app runs behind inspecting proxies.** Its
  documentation has a page for company gateways that decrypt traffic. The
  page tells them to exempt the vendor's domains, and says what a gateway
  must do if it inspects anyway. That is the strongest support this
  arrangement has.
- **The support stops short of what we would do.** The page is about a
  company's security gateway watching its staff's traffic. It says
  nothing about a program reading the replies in order to act on them.
- **Three clauses could be turned against it:**
  - to "harvest, scrape, or extract data from the Service";
  - to "decode … the … underlying structure of the Service";
  - "bypassing our systems or protective measures", if the app resists
    the proxy and the resistance is overcome.
- **One part of the app appears to resist.** The documentation says that
  when a gateway re-signs the connection, "the computer setup handshake
  fails". Reading that link would mean defeating something. Reading the
  chat traffic may not.
- **Under general law, reading one's own traffic on one's own machine is
  lawful.** The risk here is to the account under the contract, not from
  the law at large.

## 2. The arrangement

1. The proxy runs in the container beside the app.
2. The container trusts a certificate authority that the proxy holds.
3. The app's connections are routed to the proxy.
4. The proxy completes the secure connection to the vendor's server
   itself, reads each reply as it arrives, and passes it to the app over
   a second secure connection.

What this involves, stated plainly:

- **Nothing in the app is altered.** No file is patched and no code is
  injected.
- **The app's encryption is not broken.** The proxy is a party to each
  connection because the machine's owner has told the machine to trust
  it. This is how company security gateways work.
- **The proxy sees everything on those connections.** The user's
  messages, the Bot's replies, and the sign-in token in the request
  headers.
- **The official app makes every request.** The proxy originates none.

## 3. What the terms say

**Not addressed anywhere.** Proxies, interception, traffic inspection,
logging by the user, certificate authorities, monitoring of one's own
app. No clause in the Cursor Terms of Service, the Grok Bot Terms or the
Acceptable Use Policy mentions any of them.

**Clauses that could apply.**

| Clause | Words | How it could apply |
|---|---|---|
| Terms §1.5(viii) | "harvest, scrape, or extract data from the Service" | A program that takes replies out of the stream is extracting data, on the plain words |
| Acceptable Use Policy | "Scraping, harvesting or reselling any Input or Suggestion" | The same, aimed at the content itself |
| Terms §1.5(i) | "decode, or otherwise attempt to derive or gain access to the … underlying structure of the Service" | Making sense of the replies needs the private protocol's format |
| Terms §1.5(vi) | "probe, scan or attempt to penetrate the Service" | A passive reader of one's own traffic does none of these; included because a vendor might cite it |
| Acceptable Use Policy | "bypassing our systems or protective measures" | Applies only if the app refuses the proxy and that refusal is defeated |
| Acceptable Use Policy | "Accessing the Service through automated or non-human means" | Applies to the program that acts on what is read, not to the proxy as such |

The §1.5 list applies "Except and solely to the extent such a restriction
is impermissible under applicable law".

**Clauses that favour it.**

- **The content is the user's.** §5.3: "You retain all of your right,
  title, and interest that you have in Inputs", and the vendor assigns
  its rights in the outputs to the user.
- **Keeping copies is advised.** §10: "You should retain copies of any
  Content as needed".
- **Nothing is confidential by contract.** The term "Confidential
  Information" appears on none of the pages. The protocol is not made a
  secret.
- **The account holder answers for the account.** §3: you "accept
  responsibility for all activities that occur under your account". The
  terms regulate what the holder does to the Service. They are silent on
  what the holder observes of their own session.

## 4. What the documentation says about proxies

The page `docs.x.ai/grok-bot/proxies` is "for the IT team that runs your
gateway".

- **The vendor expects gateways that decrypt.** The page addresses the
  case where a gateway "re-signs the connection with its own
  certificate".
- **Its advice is to exempt the app.** "Bypass TLS (SSL) inspection" for
  every domain the app uses, on the ground that "Cursor's connections are
  already encrypted end to end."
- **It gives a reason that is technical, not legal.** Under re-signing,
  "the computer setup handshake fails even though the hostname is
  allowed".
- **It allows for inspection all the same.** Where policy requires
  inspecting everything, the gateway "has to pass streaming through
  untouched". The page points to requirements on a Cursor page, which was
  not read; as summarised, they are HTTP/2 or the vendor's HTTP/1.1
  fallback, Server-Sent Events passed through without buffering, and
  long-lived connections without forced timeouts.
- **It says to "Turn off response buffering".** A gateway that holds
  replies "leaves the app waiting on output that never arrives". Output
  "should arrive line by line, not all at once".
- **The domains it lists:** `*.cursor.sh` (chat, sign-in, approvals),
  `*.cursor-cdn.com`, `*.cursorapi.com`, `*.cursorvm.com` and
  `*.*.cursorvm.com` (the hosted computer), `cursor.com` and
  `downloads.cursor.com`.

**What the page does not say.**

- Whether the customer may log or keep what an inspecting gateway sees.
  It neither permits nor restricts it.
- How to make the desktop app trust a custom certificate authority, or
  how to point the app at a proxy.
- Whether the app pins its certificates.

**What follows from it.**

- A proxy that decrypts the app's traffic is a situation the vendor has
  planned for and documented, not one it treats as an attack.
- The vendor's preference is that nobody inspects. A preference stated in
  a setup guide is not a contract term.
- The page confirms that replies stream, which is the reason for wanting
  a proxy in the first place.
- The Grok Bot Terms say Documentation "does not create a service level,
  warranty, or commitment". The page cannot be relied on as permission.

## 5. Where the analogy holds and where it breaks

| | A company's inspecting gateway | A logging proxy in our container |
|---|---|---|
| Who runs it | The customer's IT team | The image, for whoever runs the container |
| Why | Security, data-loss prevention | To read the Bot's replies as they arrive |
| What it does to traffic | Decrypts, inspects, forwards | The same |
| What is done with the content | Scanned and logged for review | Parsed and passed to a program |
| Depends on the private protocol's format | No | Yes |
| Addressed by the vendor | Yes, in the proxies page | No |

The mechanism is identical. The purpose is not. A gateway reads traffic
to police it. Ours reads traffic to use the product by another route, and
has to understand the vendor's private message format to do it. The
clauses on extraction and on underlying structure bear on the second and
not on the first.

## 6. Protective measures

Whether anything is "bypassed" decides how exposed this is.

- **Trusting a certificate authority is not a bypass.** It is an ordinary
  administrative setting that the vendor's own page presupposes.
- **Certificate pinning would be a protective measure.** No page says
  whether the app pins.
- **The computer link appears to be protected.** The handshake to the
  hosted computer "fails" under re-signing. That is consistent with
  pinning or a client certificate on `*.cursorvm.com`. Reading that link
  would require defeating the protection, which is "bypassing … protective
  measures" in the policy's words and may be circumvention under general
  law.
- **The sensible limit follows from that.** Inspect only the connections
  that work with nothing defeated, and pass the rest through untouched.
  The page's own domain list shows which is which: chat and approvals are
  on `*.cursor.sh`, the computer link on `*.cursorvm.com`.

If the app refuses the proxy on the chat connections as well, the
arrangement cannot be made to work without patching the app, and it then
falls with the hooking methods in [READING_APP.md](READING_APP.md).

## 7. General law

- **Intercepting one's own communications.** Laws against interception
  protect communications from outsiders. A party to a communication may
  ordinarily record it, and here the account holder is a party and has
  set the proxy up. The vendor's server is the other party; it is not
  deceived about who it is talking to, since the request is the user's
  own.
- **Whose machine.** The proxy is inside the container, under the control
  of whoever runs the container. If a Diverge provider's host ran the
  proxy instead, the provider would be intercepting its customers'
  traffic, which is a different and much harder case. This report does
  not cover it.
- **Other people's content.** The traffic can carry things other people
  wrote: messages in a group conversation, a Team Bot's thread, meeting
  notes. Keeping logs of those raises privacy duties that reading one's
  own replies does not. Some places require every party's consent to
  record a conversation.
- **Copyright.** The content is the user's. The message format is not
  expression the user is copying in any meaningful way by reading a
  reply.
- **Circumvention.** Only relevant if a protection is defeated, as in
  section 6.
- **Contract.** Everything above concerns the law at large. None of it
  prevents the vendor from ending an account under its terms.

## 8. Risks that are not about legality

- **The log holds the keys.** Every request carries the sign-in token. A
  proxy that records full requests writes that token to disk. Anyone who
  reads the log can act as the account. Record replies only, and strip
  the authorisation header from anything kept.
- **The proxy's private key is a weakness.** Whoever holds it can
  impersonate any site to that container. Generate it inside the
  container at start and never ship it in the image.
- **Buffering breaks the app.** The vendor says so directly. The proxy
  must stream.
- **The format can change without notice.** The vendor changed its
  backend twice in November 2026 in ways that broke outside clients. A
  reader that depends on the wire format breaks the same way, though the
  app itself keeps working.

## 9. What makes this more defensible

- **Read only.** Forward every byte unchanged. Never originate, replay or
  alter a request.
- **Defeat nothing.** Trust a certificate authority; do not patch the app
  or strip a pin. Leave the hosted-computer link alone.
- **Keep the user's own content only**, in the container, for the user
  who runs it. Do not publish what is learned about the protocol.
- **Do not keep credentials.**
- **Say what the image does.** A user who runs it should know their
  traffic is decrypted inside it.
- **Prefer a cleaner method if one works.** Reading through the operating
  system's accessibility interface exposes none of the software and
  depends on no private format. The proxy's only advantage is that it
  sees replies as they stream.

None of these removes the automated-access question. A program that acts
on what the proxy reads is still a non-human using the app.

## 10. What is not known

- **Whether the app pins certificates** on its chat connections. Not
  tested.
- **What exactly fails on the computer link,** and why.
- **The vendor's detailed inspection requirements.** They are on a Cursor
  page that was not read.
- **Whether the vendor regards reading replies this way as extraction.**
  No statement or reported action was found.
- **The statutes and cases** behind section 7. None was read.
- **Privacy law** in any particular place, for logs that hold other
  people's messages.
- **The x.ai legal pages**, which refuse automated reads (403).

## 11. Sources

- `https://cursor.com/terms-of-service` (2026-09-03): §1.5, §3, §5.3,
  §10.
- `https://cursor.com/terms/grok-bot` (2026-10-31).
- `https://cursor.com/acceptable-use-policy` (2026-08-11).
- `https://docs.x.ai/grok-bot/proxies` and `/security`.
- `https://docs.x.ai/grok-bot/network` returned 404.

The pages were read through a tool that returns extracted text; quoted
words were checked against that text, not the rendered page.
