# Shipping the Grok Bot desktop app inside an image

Written 2026-11-25. This report covers one case: a container image whose
layers hold the Grok Bot desktop app itself, built by us and given to
others through a registry. The case where the image holds none of the
app and installs it later is out of scope.

This is not legal advice and I am not a lawyer. Two kinds of statement
appear below and are kept apart:

- **From the pages.** What the vendor's terms and documentation say,
  with short exact quotes. Sources are in
  [GROK_BOT_IMAGE_TOS.md](GROK_BOT_IMAGE_TOS.md).
- **General law.** How United States copyright and trademark law
  ordinarily treats the act. This is my general knowledge, not something
  read for this report, and it has not been checked against a lawyer, a
  court decision on this app, or the law of any other country.

## 1. Conclusion

Shipping the app inside an image that other people pull is copying and
distributing the vendor's software without a licence to do so. On the
text of the terms it breaches the contract, and under general copyright
law it is infringement unless the vendor permits it. No page read grants
that permission. A private image used only inside one organisation, by
members who already have access, is close to what the vendor's own
deployment guide tells IT teams to do and stands on much firmer ground.

## 2. The act

An image is a set of layers. If one layer holds the app, then:

- building the image makes a copy of the app;
- pushing it puts a copy on the registry;
- every pull gives a further copy to whoever pulls;
- on Diverge, running a container hands the image to a provider, which
  is one more copy on one more party's machine.

The person who builds and pushes is the one doing the copying and the
distributing. That holds whether or not money changes hands and whether
or not the recipients have their own Grok Bot accounts.

## 3. Contract: what the terms say

Grok Bot signs in with a Cursor account, and the Cursor Terms of Service
(last updated 2026-09-03) govern it, with the Grok Bot Terms and the
Acceptable Use Policy on top. The contracting entity is Anysphere, Inc.

- **The Service includes the software.** It is defined to include "all
  related software made available by Anysphere".
- **The only grant is to use.** §1.1 gives "a limited right to access and
  use the Service". Nothing grants a right to copy or distribute.
- **Nothing is implied.** §5.1: "There are no implied licenses in these
  Terms".
- **Reproduction is forbidden.** §1.5(ii): you may not "reproduce,
  modify, translate, or create derivative works of the Service".
- **So is passing it on commercially.** §1.5(iii): you may not "rent,
  lease, lend, or sell the Service".
- **So is letting others do these things.** §1.5(xi): you may not
  "knowingly permit any third party to do any of the foregoing".
- **Notices stay.** §1.5(iv): you may not "remove any proprietary notices
  from the Service".
- **The policy repeats it.** The Acceptable Use Policy lists "copying",
  "reselling" and "distributing" among prohibited acts, in the context of
  the Service's source code and systems.
- **The Grok Bot Terms add no grant.** The words "license" and "software"
  do not appear in them. Use is for the "Customer's internal business
  purposes".
- **The download page adds no grant.** It has no licence text.

The §1.5 list applies except where a restriction is "impermissible under
applicable law". No page says which restrictions that carve-out reaches.

**Remedy stated in the terms:** termination of access for a violation
(§9), and under the policy "action against your account, up to suspension
or termination". The account at risk is the one that agreed to the terms,
which is the publisher's.

## 4. Copyright: general law

- **Software is a copyrighted work.** The owner holds the exclusive
  rights to reproduce it and to distribute copies to the public.
- **Free to download does not mean free to redistribute.** Offering an
  installer at no charge grants the right to obtain and use it on the
  vendor's terms, not the right to hand copies to others.
- **A public image is distribution to the public.** Anyone can pull it.
  Without a licence this is infringement, whatever the contract says.
- **The usual defences fit poorly.**
  - *First sale* lets the owner of a lawfully made copy pass that copy
    on. Software is ordinarily licensed, not sold, and the doctrine
    covers handing over a copy, not making new ones for each recipient.
  - *The backup and essential-step allowance* for computer programs
    covers copies an owner makes to run or archive the program. It does
    not cover distributing copies to other people.
  - *Fair use* weighs against copying a whole commercial work verbatim
    for the same purpose it was made for.
- **Remedies are separate from the contract.** They include a court order
  to stop and money damages. Whether the higher statutory amounts are
  available depends on facts not known here, such as registration.
- **The practical first step is a takedown.** A registry that receives a
  copyright notice normally removes the image. The image, and anything
  built on it, then stops being pullable without warning.

## 5. Where the documentation helps, and how far

The deployment guide is addressed to an organisation's IT team. It says:

- "Pin the version you tested. Download that release's installer, keep it
  in your own software repository or MDM package store, and deploy from
  your copy".
- "Wrap the `.dmg` or `.exe` in your management tool's package format if
  it requires one."
- Deploy to "every device whose owner has access".

What this supports:

- An organisation keeping its own copy of the installer and deploying it
  to its own members' devices is doing what the vendor tells it to do. A
  private image, pulled only by those members, is a close analogue.

What it does not support:

- **Third parties.** No sentence permits giving a copy to anyone outside
  the organisation.
- **Containers.** The guide names MDM tools and configuration management.
  Docker and containers appear nowhere.
- **A licence.** The Grok Bot Terms say Documentation "does not create a
  service level, warranty, or commitment". The guide is evidence of what
  the vendor tolerates for internal deployment. It is not a grant, and it
  sits beside a contract that forbids reproduction.

On Diverge the internal case has one more gap: the image goes to a
provider's machine. If the provider is the organisation's own, nothing
leaves. If it is someone else's, a copy has gone to a third party.

## 6. The cases

| Case | Contract | Copyright (general law) | Position |
|---|---|---|---|
| Public image, anyone can pull | Breach of §1.5(ii); likely (xi) | Reproduction and public distribution with no licence | Not defensible without permission |
| Private image given to other people or companies | Same breach; fewer recipients | Still distribution to others | Smaller exposure, same act |
| Private image inside one organisation, own members, own machines | Reproduction on its face, but matches the vendor's deployment guide | Copies stay with licensed users | Defensible; the guide is the support |
| Same, but run on a third party's Diverge provider | As above, plus a copy on an outside machine | A copy passes to a non-member | Unclear; not addressed anywhere |
| Any of the above with the vendor's written permission | Permitted to the extent of the permission | Licensed | Settled |

"Private" limits who can find and report the image. It does not change
what the act is.

## 7. Other things inside the package

- **Open-source components.** The app is reported to be built on
  Electron, which bundles Chromium and other components under their own
  licences. General law and those licences require that anyone
  redistributing the binaries keep the licence texts and notices with
  them. Shipping the package unmodified normally carries the notices
  along; stripping files to slim the image can remove them. The package
  was not opened for this report, so which components and licences it
  holds is not known.
- **Modification.** Patching the app inside the image, for example to
  change its start-up flags in its own files, adds "modify" under
  §1.5(ii) and may add circumvention under the Acceptable Use Policy.
  Passing flags on the command line changes no file.
- **Trademarks.** General law lets one name another's product to say
  truthfully what something contains or works with. It does not let one
  present one's own product as theirs. The Acceptable Use Policy
  prohibits providing services that "appear to be from" Anysphere when
  they are not. The Cursor brand page covers the Cursor name and logos
  only and states no rules for Grok Bot. An image named so that it could
  be taken for the official one, or that uses the app's icon as its own,
  is the exposure; a name that says "unofficial" and a plain statement of
  what is inside is the ordinary safe form.
- **Export.** The policy prohibits breaking United States export laws for
  software. A public registry serves every country. Whether this app
  carries any export restriction is not known.

## 8. A practical problem apart from the law

A shipped copy expires. The vendor disables a build when it falls below a
remote minimum version or passes a maximum age, and those limits "are set
remotely and aren't published". An image with the app baked in therefore
stops working at a date nobody outside the vendor knows, and has to be
rebuilt and redistributed each time.

## 9. How this could be made lawful

- **Written permission from the vendor** to redistribute the Linux
  package in an image, stating to whom and under what name. The
  Acceptable Use Policy contemplates "official written consent" for other
  restrictions, so there is a channel for asking.
- **Keeping it internal.** One organisation, its own members with access,
  its own registry, its own provider. This rests on the deployment guide
  and not on a grant.

## 10. What is not known

- **The x.ai legal pages** refuse automated reads (403). Whether they
  hold a licence or a restriction for the app is not known.
- **The package's own licence files** were not read.
- **Other countries' law** is not covered. Some give users rights that a
  contract cannot remove, which is what the "impermissible under
  applicable law" carve-out points at.
- **The vendor's actual practice.** No statement by the vendor about any
  image containing the app was found, and no report of a takedown or an
  account action over one. One public project
  (`kev-inc/grok-bot-cloud`) builds such an image; that shows it has been
  done, not that it is allowed.
- **Whether a court would enforce these terms as written** against a
  given person depends on how that person accepted them, which was not
  examined.
