# Reading the Grok Bot app

Written 2026-11-25. This report covers reading what the Grok Bot desktop
app shows or holds on the user's own computer. It runs from an
accessibility screen reader at one end to hooking the process and reading
its memory at the other. It does not cover sending anything into the app:
typing, clicking, or starting work. That is in
[SUPERINTELLIGENCE_USING_APP.md](SUPERINTELLIGENCE_USING_APP.md) and
[GROK_BOT_IMAGE_TOS.md](GROK_BOT_IMAGE_TOS.md).

This is not legal advice and I am not a lawyer. Two kinds of statement
appear below and are kept apart:

- **From the pages.** What the vendor's terms and documentation say, with
  short exact quotes. The pages are listed in section 10.
- **General law.** How the law ordinarily treats the act. This is my
  general knowledge, not read for this report and not checked against a
  lawyer or a court decision.

## 1. Conclusion

- **No page names any reading method.** Screen readers, screenshots,
  debugging ports, traffic interception, local files and process memory
  appear nowhere in the terms or the documentation.
- **What the user reads is mostly the user's own.** The terms leave the
  user's inputs with the user and assign the outputs to the user. The
  conversation on screen is the user's content.
- **What the app is made of belongs to the vendor.** The terms forbid
  attempts to "gain access to the source code, object code or underlying
  structure of the Service".
- **So the line is drawn by what is read, more than by how.**
  - Reading what the app presents, through an interface the operating
    system offers for that purpose, is the most defensible.
  - Reading the app's own data as it arrives or sits on disk is
    undecided by the text.
  - Reading the app's internals, or changing it in order to read, is
    what the reverse-engineering and modification clauses are written
    against.
- **One clause could reach all of it.** The terms forbid anyone to
  "harvest, scrape, or extract data from the Service". Read broadly, that
  covers any systematic reading by a program, at any layer.
- **Who reads matters as well as how.** A person reading with an
  assistive tool is using the app. A program reading in place of a person
  meets the clause on "automated or non-human means".

## 2. The two things that can be read

| | The content | The software |
|---|---|---|
| What it is | The conversation, the Bot's replies, step progress, approval requests | Code, protocol, internal state, file formats |
| Whose it is, from the pages | The user's | The vendor's |
| Clause | §5.3: "You retain all of your right, title, and interest that you have in Inputs"; the vendor assigns its rights in Suggestions to the user, "if any" | §5.1: the vendor owns the Service "and all intellectual property rights associated with" it |

- **The terms advise keeping copies.** §10: "You should retain copies of
  any Content as needed". No page provides a way to export a
  conversation.
- **A gap.** The ownership clauses are in the Cursor Terms of Service and
  speak of "Suggestions". The Grok Bot Terms have no ownership clause and
  do not define the Bot's output. That §5.3 covers Bot replies is the
  natural reading, not a stated one.
- **Nothing is marked confidential.** The term "Confidential Information"
  appears on none of the pages. Neither the interface nor non-public
  information about the Service is made secret by contract.

Every method below reads the content. They differ in how much of the
software they must expose, or alter, to do it.

## 3. The clauses that bear on reading

**Cursor Terms of Service, §1.5.** "Except and solely to the extent such
a restriction is impermissible under applicable law, you may not":

- (i) "reverse engineer, disassemble, decompile, decode, or otherwise
  attempt to derive or gain access to the source code, object code or
  underlying structure of the Service";
- (ii) "reproduce, modify, translate, or create derivative works of the
  Service";
- (vi) "probe, scan or attempt to penetrate the Service";
- (viii) "harvest, scrape, or extract data from the Service".

**Acceptable Use Policy.**

- "reverse engineer, decompile, disassemble or otherwise seek to obtain
  the source code of our Service, including our systems, models, or
  algorithms (except to the extent this restriction is prohibited by
  applicable law)";
- "Scraping, harvesting or reselling any Input or Suggestion";
- "bypassing our systems or protective measures";
- "Accessing the Service through automated or non-human means, whether
  through a bot, script, or otherwise".

**Not in any page.**

- A clause on monitoring or intercepting one's own app.
- Any mention of accessibility or assistive technology.
- A security research policy or safe harbour.

**One staff statement.** A Cursor staff reply (forum thread 167778,
2026-08-10) said that calling "private, non-public client endpoints"
breaches the Use Restrictions. That was about a client that sends
requests. It shows that the company treats its private protocol as
within §1.5.

## 4. General law that applies throughout

- **Seeing is not copying.** Copyright does not restrict looking at what
  a lawfully used program displays. It restricts copying the work.
- **The user's own content is the user's to copy.** That follows from the
  ownership clauses above.
- **Copies made while running a program.** United States law has an
  allowance for copies made as an essential step in using a program. The
  European Union's software rules go further and give a lawful user the
  right to observe, study and test a running program to learn the ideas
  behind it, a right a contract cannot remove. The §1.5 carve-out, "to
  the extent such a restriction is impermissible under applicable law",
  points at rules of this kind.
- **Circumvention.** United States law separately prohibits circumventing
  a technical measure that controls access to a copyrighted work, with a
  narrow exception for interoperability. Whether anything in this app is
  such a measure is not known.
- **Computer misuse.** Laws against unauthorised access concern other
  people's computers. Reading a program on one's own machine is not that.
  Traffic interception is different when the machine or the connection is
  not one's own.
- **Accessibility.** Disability law generally obliges providers to make
  services usable with assistive technology. I know of no rule that turns
  that into a right for any program to read an app for any purpose.
- **Contract is separate from all of the above.** A use that copyright
  permits can still breach the terms, and the remedy the terms name is
  loss of the account.

## 5. The methods, from least to most intrusive

### 5.1 A person reading the screen

Permitted. It is the product.

### 5.2 A screen reader or the accessibility interface

Every desktop operating system offers an interface through which
assistive tools read the text and controls an application presents. An
application built on Chromium publishes its content there.

- **Software exposed:** none. The tool receives what the app chooses to
  publish to the operating system.
- **Software altered:** none.
- **Reverse engineering, §1.5(i):** not engaged. Nothing is derived about
  source code, object code or structure.
- **Extraction, §1.5(viii):** a person listening is not extracting. A
  program collecting the same text continuously into a store is closer to
  the word.
- **From the pages:** no accessibility statement exists, so the vendor
  has neither promised nor restricted this.
- **Position:** for a person, as sound as reading the screen. For a
  program reading through the same interface, the method is clean and the
  question becomes who is reading (section 7).

### 5.3 Screenshots and text recognition

- **Software exposed or altered:** none. The picture is of the screen.
- **Copyright:** a screenshot copies what is displayed. The content is
  the user's; the surrounding interface is the vendor's, and a private
  copy of it made in order to read one's own content is a small matter.
- **Position:** the same as 5.2. It reads less reliably and reveals
  nothing more.

### 5.4 Copying out by hand, the clipboard, saved files

- Selecting and copying text is a feature of the app.
- §10 advises keeping copies of Content.
- **Position:** sound.

### 5.5 Files the app keeps on disk

The app is reported to keep its settings and state under the user's
profile. The documentation does not say what it stores locally or where.

- **Logs and plain files:** reading a file on one's own disk exposes no
  code. If a file holds the user's own conversation, it is the user's
  content in another form.
- **Private formats:** working out an undocumented database or cache
  layout begins to "decode" the "underlying structure".
- **Credentials:** the sign-in token is the user's own, but taking it
  from the app's store is the first step toward using it elsewhere, which
  staff have said breaches §1.5. Reading it gains nothing for reading the
  conversation.
- **Position:** plain files are low risk; private formats are a
  grey area; the credential store is best left alone.

### 5.6 The debugging port: reading the page

Chromium applications can be started with a switch that opens a debugging
port. A tool connected to it can read the rendered page.

- **Software altered:** none. The switch is given on the command line and
  changes no file.
- **Software exposed:** all of the app's interface code and state become
  readable. Reading only the rendered text uses a small part of that.
- **Reverse engineering:** reading the document the app has rendered is
  reading its output. Reading the app's internal data structures, as one
  community project does, is access to "underlying structure".
- **Protective measures:** whether opening the port bypasses anything
  depends on whether the vendor has tried to disable it. That is not
  known. If the app refuses the switch and one forces it, the policy's
  clause on "bypassing our systems or protective measures" applies.
- **Not a supported interface.** No page mentions developer tools or
  debugging.
- **Position:** undecided by the text. Reading rendered text is the
  defensible end; reading internals is not.

### 5.7 The debugging port: watching the app's own traffic

The same port can report the requests the app makes and the replies it
receives, already decrypted, with nothing placed in the network path.

- **What is read:** the user's own content, in the form the vendor's
  server sent it, including any partial text as it arrives.
- **What that requires:** an understanding of the private protocol.
  Building that understanding is to "decode" and to "derive" the
  "underlying structure of the Service". Community projects have already
  published it; relying on their work does not change what the program
  then depends on.
- **Extraction:** a program that collects replies from the stream is
  within the plain words "extract data from the Service".
- **No request is sent.** The staff objection was to calling private
  endpoints. Here the official app makes every call and nothing else
  does. That is a real difference, and no page speaks to it.
- **Position:** undecided, and weaker than 5.6 for rendered text, because
  it depends on the protocol's structure and not on what the app shows.

### 5.8 Intercepting traffic on the network

A proxy with its own certificate, trusted by the machine, can decrypt the
app's connections.

- **From the pages:** the documentation has a page on corporate proxies
  that inspect encrypted traffic, which was not read. Its existence
  suggests the app is expected to run behind such proxies in companies.
  Whether the app pins its certificates is not stated.
- **If the app pins certificates,** defeating that is "bypassing …
  protective measures" and possibly circumvention under general law.
- **Otherwise** this is the same reading as 5.7 by a heavier route.
- **Position:** as 5.7 at best; clearly worse if a protection must be
  defeated.

### 5.9 Reading process memory

- **Software altered:** none.
- **Software exposed:** everything the process holds, including the
  vendor's code as loaded and the user's token.
- **Reverse engineering:** locating the conversation in memory requires
  learning the program's internal layout. This is access to "object code
  or underlying structure" in the plainest sense.
- **General law:** inspecting a running program on one's own machine is
  not unlawful in itself, and European Union law protects observing and
  studying a program one may lawfully use. The contract forbids it to the
  extent the law lets a contract do so.
- **Position:** within §1.5(i) on its text, saved only by the carve-out
  where local law applies.

### 5.10 Hooking and injecting code

Loading code into the process, patching functions, or altering the app's
files so that it reports what it is doing.

- **Software altered:** yes. This is to "modify" the Service under
  §1.5(ii), and "Modifying" under the policy.
- **Reverse engineering:** required in order to know what to hook.
- **Position:** the clearest breach of the methods here. Altering
  shipped files is also what would bring the redistribution and
  integrity questions in.

### 5.11 Unpacking or decompiling the app

Reading the app's packaged code to learn how it works.

- **This is reverse engineering by name.** §1.5(i) lists "decompile" and
  "disassemble".
- **General law** allows it in limited cases, such as what is necessary
  for interoperability in the European Union, under conditions.
- **Position:** prohibited by the text, outside those cases.

## 6. Summary

| Method | Alters the app | Exposes internals | Text against it | Position |
|---|---|---|---|---|
| Person reads the screen | No | No | None | Permitted |
| Screen reader, accessibility interface | No | No | None for a person | Sound |
| Screenshots, text recognition | No | No | None for a person | Sound |
| Copy, clipboard, saved files | No | No | None; §10 advises it | Sound |
| Plain files and logs on disk | No | Little | None | Low risk |
| Private file formats on disk | No | Some | "decode", "underlying structure" | Grey |
| Debugging port, rendered text | No | Available, unused | Possibly "extract data" | Undecided, defensible |
| Debugging port, internal state | No | Yes | "underlying structure" | Against the text |
| Debugging port, the app's traffic | No | The protocol | "decode", "extract data" | Undecided, weaker |
| Network interception | No | The protocol | As above; "protective measures" if pinned | As above, or worse |
| Process memory | No | Everything | §1.5(i) | Against the text, subject to the carve-out |
| Hooks, injection, patched files | Yes | Everything | §1.5(i) and (ii) | Clearest breach |
| Unpacking, decompiling | No | The code | §1.5(i) by name | Prohibited, subject to the carve-out |

## 7. Who is reading

The same method reads differently depending on who is at the other end.

- **A person with an assistive tool** is the user, using the app. Nothing
  in the terms is engaged.
- **A person's own tool keeping a record** of their conversations is
  copying the user's own content, which §10 advises. The only question is
  the word "extract".
- **A program reading in place of any person** is the case the Acceptable
  Use Policy names: "Accessing the Service through automated or non-human
  means". Reading alone sends nothing to the Service, and one can argue
  that a reader is not "accessing" anything. A program that reads in
  order to decide what to send next is part of a loop that is, taken
  whole, a non-human using the app.

A clean reading method does not make the whole arrangement clean. It
removes the reverse-engineering and modification questions and leaves the
automated-access question standing.

## 8. What makes a reading method safer

- **Read what the app presents, not what it contains.** The accessibility
  interface and the rendered page show output. Memory, internal state and
  the wire format are structure.
- **Change nothing.** No patched files, no injected code. A command-line
  switch is not a modification; a forced one may be a bypass.
- **Defeat nothing.** If the app resists a method, stop there.
- **Leave the token alone.** Reading the conversation never requires it.
- **Keep what is read to the user's own content**, on the user's own
  machine. Publishing what one learned about the protocol is a different
  act from reading one's own replies.
- **Use the operating system's interfaces** in preference to the app's.
  They exist for this, they are stable across app versions, and using
  them reveals nothing about the software.

## 9. What is not known

- **Whether the current app publishes its content to the accessibility
  interface** well enough to read replies and step progress. Not tested.
- **Whether the app accepts the debugging switch,** pins its
  certificates, or checks its own integrity. Not tested.
- **What the app stores locally.** The documentation does not say.
- **Whether the vendor regards reading-only tools as within §1.5(viii).**
  No statement or reported action was found.
- **How far the "impermissible under applicable law" carve-out reaches**
  for a given user. It depends on where the user is.
- **Whether the Terms of Service's ownership clause covers Bot output.**
- **The statutes and cases** behind section 4. None was read.
- **The x.ai legal pages**, which refuse automated reads (403), and the
  documentation pages on proxies, the Vault, and the computer view.

## 10. Sources

- `https://cursor.com/terms-of-service` (2026-09-03): §1.2, §1.5, §5.1,
  §5.3, §10.
- `https://cursor.com/terms/grok-bot` (2026-10-31).
- `https://cursor.com/acceptable-use-policy` (2026-08-11).
- `https://docs.x.ai/grok-bot/security` and
  `/approvals-security-and-privacy`.
- `https://forum.cursor.com/t/167778` (staff reply of 2026-08-10).

The pages were read through a tool that returns extracted text; quoted
words were checked against that text, not the rendered page.
