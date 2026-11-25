# Grok Bot terms and distributing the app in an image

Written 2026-11-25. This report covers what the published terms and
documentation say about putting the Grok Bot desktop app inside a
container image and giving that image to other people, and about driving
the app with scripts.

This is a reading of public pages, not legal advice. Quotes are short and
exact; everything else is paraphrase. The pages read and not read are
listed in section 9.

## 1. Short answer

- **No page grants a right to redistribute the app.** No licence for the
  desktop app exists on any page read: not on the download page, not in
  the Grok Bot Terms, not in the Cursor Terms of Service.
- **The Cursor Terms of Service forbid reproducing the Service**, and the
  Service is defined to include the software. An image that contains the
  app binaries, pushed to a registry for others to pull, is a copy we
  distribute. That is the weakest position of the options below.
- **An image that downloads the app from the vendor on the user's own
  machine is a different act.** We distribute our own code only; each
  user obtains the app from the vendor under their own account. No page
  forbids this, and the deployment docs describe scripted download from
  the release feed. No page expressly permits it for this purpose either.
- **Driving the app with scripts is named in the Acceptable Use Policy.**
  It lists "Accessing the Service through automated or non-human means,
  whether through a bot, script, or otherwise" among prohibited acts.
  This applies regardless of how the image is built, and it is the larger
  risk of the two.
- **The consequence stated for a breach is action against the account**,
  "up to suspension or termination".

## 2. Which terms apply

Grok Bot is signed in to with a Cursor account, and its terms are on
cursor.com. The contracting entity named there is Anysphere, Inc.

| Document | URL | Last updated |
|---|---|---|
| Cursor Terms of Service | `https://cursor.com/terms-of-service` | 2026-09-03 |
| Grok Bot Terms | `https://cursor.com/terms/grok-bot` | 2026-10-31 |
| Acceptable Use Policy | `https://cursor.com/acceptable-use-policy` | 2026-08-11 |
| Deployment docs | `https://docs.x.ai/grok-bot/deployment` | no date shown |

- The Terms of Service state: "If you enable or access Grok Bot, your use
  of Grok Bot is also subject to the Grok Bot Terms".
- The Grok Bot Terms sit on top of an "Agreement" that includes the
  acceptable use policy. They name no entity; "Provider" is "the entity
  providing Grok Bot to Customer under the Agreement".
- The Acceptable Use Policy "forms part of" the agreement with
  "Anysphere, Inc. or its Affiliates", applies "regardless of
  subscription tier or how the Service is accessed", and controls on
  questions of acceptable use where documents conflict.
- The download page (`https://cursor.com/download/bot`) carries no
  licence text and no "by downloading you agree" wording. Its footer
  links the three documents above.
- The x.ai legal pages (`https://x.ai/legal/…`) refuse automated reads
  (403). Whether they add anything for Grok Bot is not known.

## 3. What the terms say about copying and distributing

**Cursor Terms of Service**

- §1.1 is the only grant: "a limited right to access and use the
  Service". It grants no right to copy or distribute.
- The Service is defined to include "all related software made available
  by Anysphere".
- §5.1: "There are no implied licenses in these Terms"; all rights not
  granted are reserved.
- §1.5 (Use Restrictions) opens "you may not" and lists, among others:
  - (i) "reverse engineer, disassemble, decompile, decode";
  - (ii) "reproduce, modify, translate, or create derivative works of the
    Service";
  - (iii) "rent, lease, lend, or sell the Service";
  - (iv) "remove any proprietary notices from the Service";
  - (viii) "harvest, scrape, or extract data from the Service";
  - (xi) "knowingly permit any third party to do any of the foregoing".
  - The list applies except where such a restriction is impermissible
    under applicable law.
- §9: access may be terminated "due to a violation of these Terms".
  §1.5 survives termination.

**Grok Bot Terms**

- The words "license" and "software" do not appear.
- §1.1: a Customer "may permit its Users to access and use Grok Bot for
  Customer's internal business purposes".
- §1.2: use follows the Provider's Documentation.
- Nothing addresses copying, installers, containers or redistribution.

**Acceptable Use Policy**

- Under "Detrimentally impacting the Service", the list of prohibited
  acts includes "copying", "reselling" and "distributing" in the context
  of the Service's source code and systems, and "Modifying".
- It also prohibits providing services that "appear to be from" Anysphere
  when they are not. That bears on naming and branding an image.

## 4. What the documentation says about installers

The deployment page is addressed to an organisation's IT team: "Your IT
team installs the Grok Bot desktop app on member devices."

- **Scripted download is documented.** "For scripted packaging, read the
  current release from the release feed and download the file it names."
  The feed is
  `https://api2.cursor.sh/updates/api/download/stable/{platform}/sand`
  with `linux-x64` and `linux-arm64` among the platforms. Its fields
  include `version`, `commitSha`, `downloadUrl` (AppImage on Linux),
  `debUrl` and `rpmUrl`.
- **Keeping a copy is documented, for the organisation's own devices.**
  "Pin the version you tested. Download that release's installer, keep it
  in your own software repository or MDM package store, and deploy from
  your copy".
- **Wrapping is documented, for management tools.** "Wrap the `.dmg` or
  `.exe` in your management tool's package format if it requires one."
- **The scope is members with access.** The page says to deploy to "every
  device whose owner has access".
- **Nothing covers third parties.** No sentence permits giving the
  installer to people outside the organisation, and no sentence forbids
  it.
- **Containers are not mentioned.** Docker, VDI and headless use of the
  desktop app do not appear on any page. The security page says
  "bring-your-own-image deployment" is not supported, but that sentence
  is about the hosted cloud computer, not the desktop app.

The documentation is not a licence. The Grok Bot Terms say Documentation
"does not create a service level, warranty, or commitment".

## 5. What the terms say about driving the app

- Acceptable Use Policy, prohibited acts: "Accessing the Service through
  automated or non-human means, whether through a bot, script, or
  otherwise". A related item lists "using bots to access" beside reverse
  engineering.
- It also prohibits "circumventing any rate limits or restrictions or
  protective measures and safety mitigations".
- "Don't circumvent safeguards unless you have our official written
  consent."
- A Cursor forum reply (thread 167778, 2026-08-10, about an unofficial
  Cursor client, not Grok Bot) states the company's reading: calling
  "private, non-public client endpoints" with an access token breaches
  §1.5, and "Using your subscription outside official clients can trigger
  abuse enforcement, up to and including an account ban." The same reply
  names the supported programmatic paths as the Cursor CLI, the Agent SDK
  or headless mode, and the Cloud Agents API. None of those is a Grok Bot
  interface.
- No page addresses an official app that a program operates. Earlier I
  said driving the real app was "a different category" from what staff
  objected to. The policy's wording on automated access is broad enough
  to cover it, so that distinction does not hold up on the text.

Grok Bot is itself an automation product: routines, triggers and the
webhook are documented features. The automated-access clause reads as
aimed at reaching the Service by means other than those it offers. That
is my reading, not something a page states.

## 6. Technical facts that shape the choice

- **Old builds stop working.** A build loses support when it falls below
  a remote minimum version or passes a maximum age; the app then shows
  "Update required". "This applies to every install". The limits "are set
  remotely and aren't published".
- **Old installers may disappear.** "How long older installers stay
  available for download isn't published."
- **A baked-in copy therefore expires.** An image holding a fixed build
  stops working at a date nobody outside the vendor knows. An image that
  fetches the app at start avoids this.
- **Linux packages do not self-update.** The `.deb` and `.rpm` update
  through the package manager from a signed repository the package adds;
  the AppImage updates itself.

## 7. What other projects do

| Project | Approach | Stated position |
|---|---|---|
| `kev-inc/grok-bot-cloud` | Downloads the `.deb` from the release feed during `docker build` and copies it into the final image, which it publishes to GHCR | No licence, no README, no disclaimer; one commit |
| `ferretgeek/prokbot-plus` | States that the repository and releases contain no official program; users build from the official installer on their own machine | Unaffiliated; warns that modifying the official client may breach its terms |
| `tyonger-stack/GrokNode` | Keeps a pinned official macOS installer in Git LFS for research | Unaffiliated; "grants no licence to the upstream app" |

No statement by the vendor about any of these was found. That one project
publishes an image with the binaries inside shows it has been done, not
that it is permitted.

## 8. Options, from least to most exposed

1. **Ask for permission.** A written answer from Cursor is the only thing
   that settles both questions. The Acceptable Use Policy itself
   contemplates "official written consent".
2. **Ship no vendor binaries; fetch at container start.** The image holds
   our program and a start script that reads the release feed and
   installs the app from the vendor. We distribute nothing of theirs, and
   the app is always a supported build. The image that runs then differs
   from the image that was registered by whatever was downloaded.
3. **Ship no vendor binaries; fetch at build time on the user's
   machine.** We publish a Dockerfile, not an image. Each user builds
   their own. This fits Diverge badly, since a registered image is pulled
   from a registry, which makes the user a distributor of their own
   private copy.
4. **Publish an image containing the app.** This is reproduction and
   distribution of the Service's software with no licence to do it,
   against §1.5(ii) of the Terms of Service on its face. A private
   registry narrows who receives it; it does not change the act.

Options 2 to 4 all leave the automated-access clause in play, because
that clause is about how the Service is used, not how the app arrived.
The exposure there falls on whichever account signs in.

Two further points apply to any option:

- **Naming.** An image or agent called "Grok Bot" risks the clause on
  services that "appear to be from" the vendor. A name that says what it
  is and that it is unofficial avoids this.
- **Whose account.** The terms make each account holder responsible for
  activity under the account. Users bring their own account; the
  enforcement risk is theirs, and they should be told so.

## 9. Sources

**Read (all HTTP 200)**

- `https://cursor.com/terms-of-service`
- `https://cursor.com/terms/grok-bot`
- `https://cursor.com/acceptable-use-policy`
- `https://cursor.com/download/bot`
- `https://docs.x.ai/grok-bot/deployment`, `/security`, `/security-faq`,
  `/get-started`, `/faq`, `/approvals-security-and-privacy`,
  `/teams-and-enterprises`
- `https://forum.cursor.com/t/167778`
- `https://github.com/kev-inc/grok-bot-cloud` and its Dockerfile
- `https://github.com/ferretgeek/prokbot-plus` (README)
- `https://github.com/tyonger-stack/GrokNode` (README, partly truncated)

**Failed**

- `https://x.ai/legal`, `/legal/terms-of-service`,
  `/legal/terms-of-service-enterprise`, `/legal/acceptable-use-policy`:
  403.

**Not read**

- `https://x.ai/legal/bot-sharing-terms` (the third-party bot terms the
  FAQ links).
- The previous Grok Bot Terms (2026-10-12).
- Any licence or notice file inside the `.deb`, `.rpm` or AppImage. The
  package was not downloaded or opened. Open-source notices for Electron
  and Chromium normally ship inside such packages and would matter to
  anyone redistributing them.
- `https://cursor.com/brand` (trademark and branding rules).
- The `kev-inc/grok-bot-cloud` workflow and start script; the Dockerfiles
  of `ferretgeek/prokbot-plus`; `NOTICE.md` and `PROVENANCE.md` of
  `tyonger-stack/GrokNode`.

**Limits of this report**

- The terms pages were read through a tool that returns extracted text.
  Section numbers and quoted words were checked against that text, not
  against the rendered page.
- No clause was tested against anyone's actual enforcement. No report of
  an account banned for running or automating the Grok Bot app was found.
