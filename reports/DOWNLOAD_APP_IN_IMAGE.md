# Downloading the Grok Bot desktop app inside an image

Written 2026-11-25. This report covers an image that holds none of the
Grok Bot desktop app and downloads it from the vendor when a container
starts. It then covers one variation: the machine running the container
operates a web cache, so the download is answered from the cache and not
from the vendor.

The case where the image itself holds the app is in
[SHIP_APP_IN_IMAGE.md](SHIP_APP_IN_IMAGE.md). The terms as a whole are in
[GROK_BOT_IMAGE_TOS.md](GROK_BOT_IMAGE_TOS.md).

This is not legal advice and I am not a lawyer. Three kinds of statement
appear below and are kept apart:

- **From the pages.** What the vendor's terms and documentation say,
  with short exact quotes.
- **Observed.** What the vendor's download servers returned on
  2026-11-25.
- **General law.** How copyright law ordinarily treats the act. This is
  my general knowledge, not read for this report and not checked against
  a lawyer or a court decision.

## 1. Conclusion

- **Downloading at container start is the vendor's own distribution.**
  The vendor hands a copy to whoever runs the container. We distribute
  nothing of theirs. No page forbids it, and the deployment guide
  documents fetching the installer by script. I found nothing in the
  terms that makes the download itself a breach, with one clause that
  could be read against it (section 4).
- **A cache on the host changes who hands over the copy.** The first
  download comes from the vendor. Later ones come from the cache
  operator's stored copy. Whether that is acceptable depends on who the
  cache serves:
  - a cache that serves only one organisation's own members is what the
    deployment guide tells IT teams to do;
  - a cache that serves different, unrelated users is the operator
    redistributing the vendor's software. General law has a specific
    protection for caches, but it has conditions, and I could not confirm
    that a cache of this kind meets them.
- **Neither case touches the larger question.** A program operating the
  app after it is installed is still within the Acceptable Use Policy's
  clause on automated access. That is covered in the terms report.

## 2. The act

The image holds our program and a start script. At container start the
script:

1. reads the vendor's release feed;
2. downloads the installer the feed names, from the vendor's server;
3. installs it inside the container.

The copy that results is made by the vendor's server sending it and the
container receiving it. The person running the container is the one who
obtains it. The author of the image supplies instructions, not a copy.

## 3. What the vendor publishes

**From the pages (deployment guide, `docs.x.ai/grok-bot/deployment`)**

- "For scripted packaging, read the current release from the release feed
  and download the file it names."
- The feed is
  `https://api2.cursor.sh/updates/api/download/stable/{platform}/sand`,
  with `linux-x64` and `linux-arm64` among the platforms.
- The guide is addressed to an organisation's IT team and speaks of
  installing "on member devices".
- The download page (`cursor.com/download/bot`) offers the same files to
  anyone. It has no licence text and no "by downloading you agree"
  wording.

**Observed**

- The feed answered an unauthenticated request with the current release:
  version `1.3.0`, and URLs for the AppImage, the `.deb` and the `.rpm`
  on `downloads.cursor.com`.
- The `.deb` URL answered an unauthenticated request with status `200`
  and a length of 117,982,212 bytes. The file was not downloaded.
- No sign-in, token, cookie or agreement step stood in front of either
  request.

So the vendor offers the installer to the public, without an account, and
documents fetching it with a script.

## 4. What the terms say

**How a person becomes bound.** The Cursor Terms of Service (2026-09-03)
state: "By using the Service, you agree to these Terms including our
Acceptable Use Policy." The Service is defined to include "the website
www.cursor.com, and all related software made available by Anysphere".
Downloading is not named as the act that binds. Signing in to the app
with a Cursor account is use of the Service on any reading, so whoever
signs in is bound whether or not the download bound them.

**Clauses about downloading or installing.** There are none. No clause
addresses downloading, installing or updating the software.

**Clauses that could be read against a scripted download.**

- *Automated access.* The Acceptable Use Policy lists, under "Not
  complying with laws or regulations": "Accessing the Service through
  automated or non-human means, whether through a bot, script, or
  otherwise".
  - Against: a script fetching from the vendor's servers is, literally,
    a script accessing something the vendor runs.
  - For: the vendor's own guide tells readers to do exactly this
    ("scripted packaging"), package managers do it for every `.deb` and
    `.rpm` update, and the app's own updater does it. A reading that
    forbids the documented method cannot be the intended one.
  - My reading is that this clause is not aimed at fetching the
    installer. That is a reading, not something a page states.
- *Rate limits.* The policy prohibits "circumventing any rate limits or
  restrictions or protective measures". One download per container start
  circumvents nothing. Many containers starting at once from one address
  could look like abuse to the server; that is a practical risk more than
  a legal one.
- *Reproduction.* §1.5(ii) of the Terms forbids anyone to "reproduce …
  the Service". Receiving the copy the vendor sends in order to install
  it is the use the vendor offers it for. If that counted as forbidden
  reproduction, nobody could install the app.

**Who the guide covers.** The guide is written for an organisation
deploying to its own members. An individual running a container for
themselves is doing the same act the public download page offers to
anyone. An image author whose users each run it for themselves is not
deploying to "members", but each user is still obtaining their own copy
from the vendor.

**What the image author is responsible for.** §1.5(xi) forbids one to
"knowingly permit any third party to do" the restricted acts. The
download is not one of them on the reading above. The author's exposure
comes from what the program does with the app afterwards, not from the
download.

## 5. General law on the download

- **The vendor makes the distribution.** A copyright owner who offers a
  file for public download and sends it on request has authorised that
  copy. The recipient holds a lawfully made copy to use on the vendor's
  terms.
- **Publishing instructions is not distributing the work.** A script that
  says where to fetch a file contains none of the file. Liability for
  another person's download would need that download to be unlawful in
  the first place, and here it is the vendor's own offer.
- **The licence position is unchanged.** The recipient still has only the
  right to use, and may not pass the copy on.

## 6. Practical facts that bear on the choice

- **The running image differs from the registered one.** What runs is the
  registered image plus whatever the vendor served that day.
- **The download can fail or change.** The vendor may alter the feed,
  remove old installers ("How long older installers stay available for
  download isn't published"), or refuse requests. The container then
  fails to start.
- **Integrity.** The feed carries no checksum field that the guide
  documents. The `.deb` and `.rpm` add a "signed" package repository, so
  later updates through the package manager are verified; a first
  download by plain HTTPS relies on the connection alone.
- **Size.** About 118 MB per container start, which is the reason anyone
  would want a cache.

## 7. A web cache on the host

### 7.1 What the cache does

The host stores the installer the first time a container downloads it
and answers later requests from its stored copy. Two things follow.

- **The cache makes and keeps a copy.** It is no longer a copy in
  transit.
- **The cache hands copies to later requesters.** Those requesters
  receive the file from the cache operator, not from the vendor.

### 7.2 A technical fact that matters legally

**Observed.** The download URLs are HTTPS only. The `.deb` response
carried `Last-Modified` and `ETag` headers and no `Cache-Control` or
`Expires` header; the vendor's own content network reported it as not
cached (`cf-cache-status: DYNAMIC`). The feed response carried no caching
headers at all.

- **The vendor neither invites nor forbids caching.** It sent no
  `no-store` or `private` instruction. Under the ordinary rules of HTTP,
  a response with a `Last-Modified` date and no instruction may be stored
  by a cache for a period the cache chooses.
- **An ordinary cache cannot see HTTPS traffic.** To cache this download
  the host must either break open the encrypted connection, with its own
  certificate trusted inside the container, or the container must be
  pointed at the cache as a mirror on purpose. Either way the cache is a
  deliberate arrangement, not a passive part of the network. That weakens
  any claim that the operator is a mere conduit.
- **The feed must not go stale.** If the cache also stores the feed, it
  can go on naming a build the vendor has stopped supporting. The app
  then shows "Update required", because the vendor's limits "are set
  remotely" and apply to "every install".

### 7.3 What the terms say

Nothing. No clause in the Terms of Service, the Grok Bot Terms or the
Acceptable Use Policy mentions caching, mirroring or proxying. The
clauses that could apply are the general ones:

- §1.5(ii), not to "reproduce … the Service";
- the Acceptable Use Policy item listing "copying" and "distributing".
  Its wording is ambiguous: the list ends with "or otherwise seek to
  obtain the source code of our Service", and it is unclear whether
  "copying" and "distributing" apply to the Service as a whole or only to
  its source code.

A contract binds those who agreed to it. A cache operator who has no
Cursor account and never uses the Service may not be a party to these
terms at all. Copyright law applies to the operator regardless.

### 7.4 The cases

**The cache serves one organisation's own members.** The guide says:
"Download that release's installer, keep it in your own software
repository or MDM package store, and deploy from your copy". A cache or
internal mirror that only that organisation's members draw from is a
software repository in all but name. This is the documented method, and
it is as well supported as the internal case in the shipping report.

**The cache serves one person's own containers.** The same person
downloads once and reuses their own copy on their own machine. This is
the weakest possible form of copying: nobody else receives anything.
General law in the United States has an allowance for copies a program's
owner makes in order to use it, and whatever its exact reach, I know of
no theory on which reusing one's own lawful download is treated as
distribution.

**The cache serves different, unrelated users.** This is the case of a
Diverge provider that runs containers for many customers and caches
downloads for all of them. The operator stores the vendor's software and
hands it to people the vendor has not served.

- *What helps.* Every recipient could have fetched the identical file
  from the vendor for free, without an account. The copy is unmodified.
  The vendor loses nothing but a download count. The vendor sent no
  instruction against caching.
- *The legal protection for caches.* United States law limits the
  liability of service providers for "system caching", and European
  Union law has a parallel rule. Both come with conditions. From general
  knowledge, the conditions are along these lines: the material is stored
  through an automatic process and passed on unmodified; the cache obeys
  the origin's rules on refreshing; it does not interfere with the
  origin's technology for counting use; it respects any access conditions
  the origin imposes; and it removes material on notice.
  - A cache that respects the vendor's headers, keeps the file
    unmodified, and serves only what a user's own request asked for fits
    the shape of that rule.
  - Two points are uncertain. The rule was written for caches that sit in
    the path of ordinary traffic, and this one needs a deliberate
    arrangement to read HTTPS. And the protection limits remedies against
    a qualifying provider; it does not turn the copy into a licensed one.
  - I have not read the statute or any case for this report. Whether a
    particular provider qualifies is a question for a lawyer.
- *What hurts.* The operator is keeping a copy of software it has no
  licence to redistribute and handing it out. If the operator has a
  Cursor account, §1.5(ii) applies to it directly. A cache that keeps
  serving an installer after the vendor has withdrawn it is serving
  something the vendor no longer offers.

**A mirror, not a cache.** If the host fetches installers in advance, or
keeps them indefinitely regardless of what the vendor serves, it is a
mirror. That is the shipping case under another name: a stored copy
handed out by someone other than the vendor. The reasoning in
[SHIP_APP_IN_IMAGE.md](SHIP_APP_IN_IMAGE.md) applies, and for an outside
operator the answer is the same as for a private image given to others.

### 7.5 Summary of the cache cases

| Who the cache serves | Terms | General law | Position |
|---|---|---|---|
| One person's own containers | Not addressed; nothing passes to anyone | Reuse of one's own lawful copy | Sound |
| One organisation's members | Matches the deployment guide | Copies stay with licensed users | Sound; the guide is the support |
| Unrelated users, cache follows the vendor's headers and keeps nothing the vendor has withdrawn | Not addressed; operator may not be a party | Redistribution, possibly within the caching protection | Unclear; the best argument is the caching rule |
| Unrelated users, fetched in advance or kept indefinitely | Reproduction on its face if the operator is bound | Redistribution with no licence | Same as shipping the app to others |

### 7.6 Who carries the risk

- **The image author** published a script that asks the vendor for the
  file. The author neither runs the cache nor chooses it, and may not
  know it exists.
- **The person running the container** asked for the vendor's file and
  received identical bytes. Their own position is the same as for a
  direct download.
- **The cache operator** made the stored copy and handed it on. The
  exposure, such as it is, sits there.

## 8. What would reduce the risk

- **Fetch the feed fresh every time**, and let only the installer be
  cached. The build is then always one the vendor currently names.
- **Have the cache honour the vendor's headers** and revalidate with the
  `ETag`. This keeps it a cache and not a mirror, and it is the condition
  the caching protection turns on.
- **Verify what was received.** Prefer installing through the vendor's
  signed package repository, so a cached file that was altered is
  rejected.
- **Keep the cache per customer** where it is not one organisation's own.
  A cache that never serves one customer's download to another removes
  the redistribution question.
- **Ask the vendor.** A written answer settles the shared-cache case, and
  the automated-access question with it.

## 9. What is not known

- **Whether the vendor regards a scripted download by an outsider as
  permitted.** The guide documents the method for IT teams; no page says
  who else may use it.
- **Whether downloading alone binds a person to the terms.** The terms
  bind "by using the Service" and do not name downloading.
- **The x.ai legal pages**, which refuse automated reads (403).
- **The statute and case law on caching.** Section 7.4 is from general
  knowledge only.
- **Other countries' law**, and the law of Texas, which the Terms choose
  as governing (§16.1).
- **The package's own licence files**, which were not read.
- **The vendor's practice.** No statement about caches, mirrors or
  proxies of the installer was found, and no report of action against
  one.

## 10. Sources

- `https://cursor.com/terms-of-service` (2026-09-03)
- `https://cursor.com/terms/grok-bot` (2026-10-31)
- `https://cursor.com/acceptable-use-policy` (2026-08-11)
- `https://cursor.com/download/bot`
- `https://docs.x.ai/grok-bot/deployment`
- Release feed for `linux-x64` and the response headers of the `.deb`
  URL it named, requested 2026-11-25 without signing in.

The terms pages were read through a tool that returns extracted text;
quoted words were checked against that text, not the rendered page.
