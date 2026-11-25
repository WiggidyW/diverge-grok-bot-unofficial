# The spirit of the terms

Written 2026-11-25. This report makes one argument, at length:

> The people who wrote these terms want Grok Bot to earn money. A
> container that runs the official app, under a paying user's own
> account, earns them money. If that container also complies with the
> letter of the terms, it upholds the letter and the spirit together.

The other reports in this folder ask what the words forbid. This one asks
what the words are for.

It is written as advocacy. It states the case for our use as strongly as
the pages allow. Section 9 says where the case is weakest, because an
argument that hides its weak points is not worth making. This is not
legal advice and I am not a lawyer.

A note on names. The terms are published on cursor.com and name
Anysphere, Inc. as the contracting party. The same pages link "SpaceXSI
Subscription Terms" at x.ai, and the product is offered at x.ai/bot. This
report says "the vendor" for whoever stands behind Grok Bot. The argument
does not depend on which company that is.

## 1. The premise: what the vendor wants

The vendor sells Grok Bot. Its terms say so at every turn.

- **It is a subscription.** The Terms of Service describe "automatically
  recurring" charges and reserve the right to "suspend or terminate
  access to the Service" for unpaid amounts.
- **The acceptable use rules protect the revenue.** The policy prohibits
  "Manipulating, circumventing, or interfering with Anysphere's
  subscription, billing, or usage-metering systems".
- **They protect the price.** It prohibits "Exploiting account
  provisioning, team management, or user assignment workflows to avoid or
  reduce fees".
- **They protect the product from being resold.** It prohibits
  "reselling", and the Terms forbid anyone to "rent, lease, lend, or sell
  the Service".
- **They protect the product from being copied.** The Terms forbid using
  the Service "to develop or train a model that is competitive with the
  Service".

No clause in these documents protects a principle for its own sake. Each
restriction guards something of commercial value: the fee, the meter, the
code, the models, the brand, or the vendor's freedom from liability. A
company's terms are an instrument of its business. To ask what the terms
intend is to ask what the business needs.

What the business needs is paying customers who use the product a great
deal, depend on it, and tell others.

## 2. What our container does for the vendor

A Diverge agent image built on Grok Bot does the following, and nothing
else.

- **It requires a paying customer.** The container is useless without a
  Grok Bot account. Each person who runs it brings their own account, on
  the vendor's own plans, at the vendor's own price.
- **It runs the vendor's own app.** The official desktop app, obtained
  from the vendor, makes every request. Nothing impersonates it.
- **It runs the vendor's own Bot on the vendor's own computer.** All the
  work happens where the vendor does its metering and enforces its
  limits.
- **It gives the product a new place to be used.** Diverge users who
  would not otherwise have opened Grok Bot now have a reason to subscribe
  to it.
- **It makes the product harder to leave.** A Bot wired into someone's
  daily tools is a Bot they keep paying for.

Every use of our container is a use of Grok Bot, by a Grok Bot customer,
through Grok Bot's app, billed by Grok Bot's vendor. We add a customer's
reasons to subscribe and take nothing from the price.

## 3. What our container does not do

The restrictions in the terms describe the conduct the vendor fears. Set
our container beside each of them.

| What the terms guard against | The words | Our container |
|---|---|---|
| Using a subscription without paying for the use | "circumventing … subscription, billing, or usage-metering systems" | Meters exactly as the app meters; touches no billing system |
| Many people on one seat | "to avoid or reduce fees" | One account per user, each their own |
| Reselling access | "rent, lease, lend, or sell the Service" | Sells nothing of the vendor's; every user contracts with the vendor directly |
| Taking the raw model out of the product | "model extraction or theft attacks" | The vendor's Bot does the work, inside the vendor's harness |
| Building a rival | "competitive with the Service" | Builds a route into the Service, not a replacement for it |
| Taking the code | "reverse engineer … decompile" | Runs the app as shipped and alters nothing |
| Draining the content | "harvest, scrape, or extract data" | Reads only the user's own conversation, for that user |
| Breaking the Service | "probe, scan or attempt to penetrate" | Makes no request the app does not make |
| Passing as the vendor | services that "appear to be from" the vendor | Named and described as unofficial |

On every line the feared conduct costs the vendor money or control, and
our container does the reverse.

## 4. What the vendor has said its concern is

One statement from the vendor's side shows the concern in its own words.
A Cursor staff member, asked about an unofficial client, wrote that it

> "authenticates using your access token and calls our private,
> non-public client endpoints"

and that

> "Using your subscription outside official clients can trigger abuse
> enforcement, up to and including an account ban."

Read what is objected to. It is the use of a subscription **outside
official clients**. The mischief is a flat-priced subscription being
drawn on by software the vendor did not build, cannot meter properly, and
cannot hold to its limits.

The same reply names what is acceptable. The supported routes "always run
the Cursor agent harness, they don't expose the raw model."

That is the test the vendor itself applies: does the vendor's own harness
do the work, or has the raw model been taken out from under it? In our
container the official client runs, the vendor's harness runs, and the
raw model is never exposed. We are on the permitted side of the line the
vendor drew.

## 5. The clause that seems to stand in the way

The Acceptable Use Policy lists, among prohibited acts:

> "Accessing the Service through automated or non-human means, whether
> through a bot, script, or otherwise"

Read alone, these words describe our container. Read for their purpose,
they do not.

**The clause cannot mean what it literally says.** If it did, the vendor
would be in breach of its own policy by offering its product.

- Grok Bot is a bot. The Grok Bot Terms authorise actions that "Grok Bot
  initiates autonomously".
- Its routines run on schedules with no person present. The Terms make
  the customer responsible for "monitoring recurring or unattended
  activities".
- It offers a webhook so that programs can start work.
- It lets one Bot message another.
- Its deployment guide tells readers how to fetch the installer "For
  scripted packaging".
- The app's own updater contacts the Service by automated means every
  day.

A clause that forbade all automated or non-human access would forbid the
product. No reasonable reader takes it that way, and no drafter meant it
that way.

**The clause's company shows its purpose.** It sits under the heading
"Not complying with laws or regulations". Its neighbours in that list
include "creating fake accounts" and "Misleading others or not being
transparent regarding your use of AI". The list is a catalogue of abuse:
sham sign-ups, deception, flooding. The clause belongs to that family. It
is aimed at traffic that pretends to be a customer and is not.

**Our container is not that traffic.** It is a real customer's real
account, operating the real app, openly. Nothing is faked and nobody is
misled.

**So the clause, read for what it is for, does not reach us.** It reaches
the script that opens a thousand accounts, the scraper, the flood. It
does not reach a paying customer who has arranged for the product to be
operated on their behalf.

## 6. The clause on human review

The Grok Bot Terms say the "Customer will use appropriate human review
before permitting Agentic Actions".

**The purpose is written on its face.** The same section says approval
controls "are provided as aids only", and that the "CUSTOMER IS SOLELY
RESPONSIBLE FOR ALL AGENTIC ACTIONS TAKEN BY GROK BOT". The clause exists
so that when a Bot does something costly, the cost is the customer's and
not the vendor's.

**Our container leaves that allocation exactly where it is.** The account
holder remains responsible for everything done under the account. The
vendor's exposure is unchanged. The word "appropriate" leaves the amount
of review to the customer, who bears the consequence of choosing badly.

The clause protects the vendor from liability. We do nothing to shift
liability onto the vendor, so its purpose is met.

## 7. Why purpose matters in reading a contract

These points are general law, from my own knowledge and not checked
against any authority.

- **A contract is read to give effect to what the parties intended.**
  Courts prefer the reading that serves the agreement's evident purpose
  over one that defeats it.
- **A term is read with the terms around it.** Words in a list take their
  colour from the list. A clause filed under illegal and abusive conduct
  is read as addressing illegal and abusive conduct.
- **A reading that makes the contract absurd is rejected.** A reading
  under which the vendor's own product breaches the vendor's own policy
  is such a reading.
- **Doubt is resolved against the party who wrote the words.** These
  terms were written by the vendor alone and offered without negotiation.
  Where a clause can fairly be read two ways, the customer is entitled to
  the reading that favours the customer.
- **Each party must act in good faith.** A power to end an account is not
  ordinarily to be used against a customer who has paid in full and done
  the vendor no harm.

Every one of these principles points the same way. The narrow, purposive
reading of the automated-access clause is the more reasonable reading. It
is the reading that serves both parties, and it is the one the law of
contract prefers.

## 8. The argument, stated whole

1. The terms exist to protect the vendor's revenue, its code, its models
   and its freedom from liability.
2. Our container adds to that revenue. It requires each user to be the
   vendor's paying customer, on the vendor's price, through the vendor's
   app, on the vendor's computer, under the vendor's meter.
3. Our container takes none of what the terms protect. It copies no
   code, extracts no model, resells no access, avoids no fee, deceives
   nobody, and shifts no liability.
4. The vendor's own stated line is between its harness and the raw model.
   Our container stays on the harness side.
5. The one clause whose words seem to reach us cannot be read literally
   without condemning the product itself, and read by its purpose and its
   place it is aimed at sham traffic, which ours is not.
6. Therefore a container that keeps to the letter of the terms in every
   other respect also keeps to their spirit. It does what the vendor
   wants done: it sells Grok Bot.

A customer who pays the price, uses the product as built, harms nothing
the vendor values, and brings the vendor new customers is not the person
these terms were written to stop. That customer is the person the terms
were written to keep.

## 9. Where the argument is weakest

An honest statement of the case includes these.

- **The premise in the argument is conditional, and the condition is not
  yet met.** The argument says "if it is compliant with the literal law".
  The other reports found that the words "non-human means" cover our
  container on their face, and that the Grok Bot Terms define a User as
  "an individual". The argument above is a reason to read those words
  narrowly. It is not a finding that they do not apply.
- **The vendor decides, not us.** The remedy in the terms is action
  against the account. A vendor that reads its clause broadly can act on
  that reading without persuading anyone. An argument about spirit
  matters most in front of a court, and few people go to court over a
  closed account.
- **More use is not always more money.** On a flat-priced plan, a
  customer who runs a Bot all day through an agent costs the vendor more
  than one who types a few messages. The vendor's rate limits bound this,
  but the staff remark about "abuse enforcement" shows that heavy
  automated use of a subscription is exactly what the vendor watches for.
  I have not checked how Grok Bot's plans are priced or metered.
- **The vendor wants more than revenue.** It also wants control of how
  its product is reached, the ability to change it without breaking
  others, and safety. A human in front of the app serves the safety aim.
  Our container removes that human.
- **The vendor has already shown a strict hand.** In November 2026 it
  closed a route that outside clients used, as a deliberate security
  change.
- **Reading the app's traffic weakens point 4.** If the container reads
  replies off the wire, it depends on the private protocol, which is
  closer to what staff objected to than operating the app is.
- **Our interest is not the vendor's interest merely because we say so.**
  The surest way to know what the vendor wants is to ask it.

## 10. What follows

- **The argument is a strong reason to ask, and a good letter to send.**
  Everything in sections 2 to 6 is what one would say to the vendor when
  requesting written consent. The policy contemplates "official written
  consent" for other restrictions. A vendor that wants paying customers
  has every reason to say yes.
- **It is a reason to build the container so that the argument stays
  true.** One paying account per user. The official app, unmodified. No
  token taken out. Nothing resold. Nothing hidden. Named as unofficial.
  The vendor's limits respected.
- **It is not a substitute for the vendor's answer.** If the vendor says
  no, the spirit of the terms is whatever the vendor says it is.

## 11. Sources

- `https://cursor.com/terms-of-service` (2026-09-03).
- `https://cursor.com/terms/grok-bot` (2026-10-31).
- `https://cursor.com/acceptable-use-policy` (2026-08-11).
- `https://forum.cursor.com/t/167778` (staff reply of 2026-08-10, about
  an unofficial Cursor client, not about Grok Bot).
- `https://docs.x.ai/grok-bot/` and its deployment page.

Quoted words were checked against text returned by a page-reading tool,
not against the rendered pages. The companion reports are
[GROK_BOT_IMAGE_TOS.md](GROK_BOT_IMAGE_TOS.md),
[SUPERINTELLIGENCE_USING_APP.md](SUPERINTELLIGENCE_USING_APP.md),
[READING_APP.md](READING_APP.md) and
[PROXY_LOGGING.md](PROXY_LOGGING.md).
