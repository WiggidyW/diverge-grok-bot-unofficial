# The frontend: the real Grok Bot app, shown as a website

Written 2026-11-25. This report covers how this agent's Diverge frontend
could be the literal Grok Bot app. It states how a Diverge frontend
works, why the app cannot simply be served as a web page, the ways it can
be shown as one anyway, and what Diverge would need to add.

Nothing here has been built or run. Statements about Diverge come from
its source (`objectiveai` at `bc3b52f46`). Statements about the remote
display tools are from my general knowledge of them and are marked where
they matter. Statements about the app are from the vendor's pages and
from community projects, as the other reports record.

## 1. Conclusion

- **It can be a website, with Diverge as it is today.** The app's window
  is drawn on a display inside the container and streamed to the
  frontend as pictures over a WebSocket. The person sees and operates the
  real app. No new Diverge feature is required for this to work.
- **It cannot be the app's own pages served directly.** The app's
  interface is not a website and cannot be made one without taking the
  app apart.
- **Start with a plain VNC display and the noVNC web client.** It has the
  fewest parts and is the easiest to debug. Move to Xpra if the fit and
  finish matter once it works.
- **Diverge is missing conveniences, not essentials.** The frame is
  allowed to write the clipboard and nothing else. Pasting, full screen,
  the microphone and opening links on the person's own machine would each
  need a small addition.
- **This is also the cleanest arrangement on the terms.** A person
  operating the real app, through a picture of it, is a person using the
  app.

## 2. How a Diverge frontend works

**What the image provides**

- The program is one HTTP server on the container's loopback. An image
  whose `DIVERGE.md` says `"frontend": true` serves pages beneath
  `/frontend`.
- Only paths beneath `/frontend` are reachable. The program's other
  routes never are.

**How a request gets there**

1. The Diverge viewer opens a frame. The viewer's own Rust serves that
   frame from `http://127.0.0.1:<port>/frontend/`, a new port for every
   frontend tab, so each frontend is its own web origin.
2. Every request the frame makes is sent through the daemon and the
   provider to the container's proxy, which makes the same request to
   the program.
3. The answer streams back as it is produced.

**What it carries**

- **Any HTTP request and response**, bodies streamed in both directions.
- **WebSockets.** A request the program answers with `101` becomes a
  two-way pipe for as long as either side keeps it open.
- **Nothing else.** There is no path for UDP or for a direct connection
  between the browser and the container.

**What else is true of it**

- **The caller is attested.** The daemon stamps every request with the
  account that made it, in the `diverge.network` header. The page cannot
  forge it.
- **The container is started for a request** and held up while the
  request is answered. An open WebSocket therefore keeps the container
  running for as long as the tab is open.
- **The frame's permissions are narrow.** The viewer sets
  `allow="clipboard-write"` and no other permission.
- **The sheet can fill the window.** The viewer's sheet resizes from 20%
  to 90% of the window and has a control that makes it cover the whole
  window.
- **A theme is offered.** The viewer answers `/diverge/theme.css` and
  `/diverge/theme.js` on every frontend.

## 3. Why the app's interface is not a website

The Grok Bot desktop app is reported to be built on Electron. Its
interface is made with web technology, but it is not a site:

- The pages load from inside the application package, not from a server.
- They depend on the app's own background process for sign-in, storage,
  the network and the operating system. A page taken out of the app has
  nothing to talk to.
- The vendor offers no web version of the chat. The documentation says
  there is none.

Serving those pages from our program would mean unpacking the app,
re-hosting its files and imitating its background process. That is
taking the app apart and modifying it, which the terms forbid by name
and which would break on every release. It is ruled out.

What remains is to leave the app whole and show its window.

## 4. The approach: stream the window

Inside the container:

1. A display server runs with no screen attached.
2. The Grok Bot app starts on that display, unmodified.
3. A small server turns the display into a stream and accepts mouse and
   keyboard input for it.
4. Our program serves a web client beneath `/frontend` and passes the
   client's WebSocket through to that server.

The person opens the frontend tab and sees the app. Clicks and keys go to
the real app. Everything the app can do is available, because it is the
app: approvals, Secure Forms, the computer view, step-by-step progress,
settings.

This fits the frontend as it exists. It needs HTTP for the client's
files and one WebSocket for the stream, and Diverge carries both.

## 5. The options

All of these put the same real app on screen. They differ in how the
picture travels and how polished the result is. The descriptions of the
tools are from general knowledge and are not verified against this app.

### A. VNC display with the noVNC web client

A virtual display that speaks VNC (TigerVNC's `Xvnc`), a WebSocket
bridge, and noVNC, a web page that draws the display.

- **For:** the simplest and most widely used arrangement. Few parts,
  easy to see which one has failed. Each part is a Debian package. The
  display can follow the size of the browser pane.
- **Against:** shows a whole desktop, so the app has to be started
  maximised with nothing around it. No sound. Clipboard is text only,
  and pasting goes through a panel unless the frame may read the
  clipboard.
- **Under `/frontend`:** noVNC loads with relative addresses and takes
  the WebSocket's path as a setting, so it can live beneath a prefix.

### B. Xpra with its HTML5 client

Xpra forwards individual application windows, not a desktop, and ships
its own web client over WebSocket.

- **For:** the app's window is the page, with no desktop around it.
  Windows the app opens, such as a file chooser or the sign-in browser,
  appear as their own windows. It handles clipboard, sound and file
  transfer, and follows the pane's size.
- **Against:** more parts and more settings than VNC, and harder to
  debug. Whether its client behaves well beneath a path prefix needs
  checking.

### C. KasmVNC

A VNC server with a web server and a web client built in.

- **For:** one program does the display, the stream and the page. Better
  picture compression than plain VNC.
- **Against:** not a standard Debian package, so it is one more thing to
  fetch or to ship. Whether it serves cleanly beneath a prefix needs
  checking.

### D. Selkies

A streaming stack built for games and desktops. One community project
already runs Grok Bot on an image built around it.

- **For:** the smoothest picture, and sound.
- **Against:** its original transport is WebRTC, which needs UDP or a
  relay server. A Diverge frontend carries neither. Newer versions are
  said to have a WebSocket mode; that would need confirming before this
  option is considered. It is also the heaviest of the four.

### E. The debugging port's screencast

Chromium can send pictures of a page through its debugging port and
accept input the same way. A small page of our own would draw them.

- **For:** no VNC stack, and only the app's content is sent.
- **Against:** the app still needs a display to run on. Menus, dialogs
  and the sign-in browser are outside the page and would not appear. It
  relies on the debugging port, which [READING_APP.md](READING_APP.md)
  rates below a plain view of the screen. It would be our own client to
  write and maintain.

### F. A page of our own, built from what we read

A chat page that we write, filled from the Bot's replies.

- This is not the Grok Bot interface. It is listed only to set it aside:
  it depends on reading the app, covers a fraction of what the app does,
  and is the opposite of what was asked for.

### Comparison

| | Shows | Sound | Clipboard | Parts | Fits `/frontend` today |
|---|---|---|---|---|---|
| A. VNC + noVNC | A desktop holding the app | No | Text, limited | Fewest | Yes |
| B. Xpra | The app's windows | Yes | Yes | More | Likely; check the prefix |
| C. KasmVNC | A desktop holding the app | Possible | Yes | One program | Likely; check the prefix |
| D. Selkies | A desktop holding the app | Yes | Yes | Most | Only if its WebSocket mode exists |
| E. Screencast | The app's page only | No | Ours to build | Ours to build | Yes |

### Recommendation

Build A first. The first question is whether the app runs at all in a
container with no screen and no graphics card, signs in, and stays
signed in. A plain VNC display answers that with the least in the way.
If it does, B is the one to move to for a result that feels like the app
and not like a remote desktop.

## 6. What the container needs

- **A display with no screen.** `Xvnc` provides one.
- **The app's flags.** As root in a container, Chromium's sandbox cannot
  start, so the app runs with `--no-sandbox`. Containers give little
  shared memory, so it also runs with `--disable-dev-shm-usage`. Both
  are given on the command line; neither changes the app.
- **Software drawing.** There is no graphics card. The app draws on the
  processor, which costs more of it.
- **A browser.** Sign-in opens a web page. A browser has to be installed
  in the container and shown on the same display.
- **Memory.** An Electron app, a browser and a display server need more
  than a typical agent. The image's document asks for 2 GiB; that figure
  is a guess until measured.
- **Our program as the door.** It serves the web client beneath
  `/frontend`, checks the caller, and passes the WebSocket to the
  display server on the loopback.

## 7. Problems to solve, and whether Diverge covers them

### Signing in

The app signs in through a web page and is then handed back to with a
`grokbot://` link.

- **Inside the container this can work as it does on a desktop.** The
  browser is on the same display, and the app's package registers itself
  for that link.
- **It cannot be done in the person's own browser.** The link would be
  handed to their machine, not to the container. Community clients sign
  in by polling instead, which suggests another route exists, but that
  is their method and not the app's.
- **Diverge:** nothing needed.

### Staying signed in

A container can be stopped and started again. The app's session lives in
its settings folder and, on Linux, in the system keyring.

- **The disk may not be enough.** I have not confirmed what of a
  container's disk survives a restart.
- **A rotating secret fits.** Diverge hands a `rotating` secret to the
  program at every activation and collects it back at every inactive.
  The app's session could be packed into one and unpacked on start.
  Whether a secret may be large enough for that is not known.
- **A keyring has to exist.** With no desktop session there is none
  unless one is started, or the app is told to store its data plainly.
- **Diverge:** possibly nothing. A stated size limit for secrets, or a
  volume, would settle it.

### Who may look

The display is the person's own signed-in Grok Bot. Anyone who reaches
it can act as them.

- The daemon refuses accounts that may not open the agent's frontend,
  and stamps the rest.
- Our program must check the stamp on every request, the WebSocket
  included, as the example agent `human` does.
- Two people with access would share one screen and one mouse.
- **Diverge:** nothing needed.

### Clipboard

- **Copying out works.** The frame may write the clipboard.
- **Pasting in does not, directly.** The frame may not read the
  clipboard, so the web client cannot pass a paste to the app. The
  fallback is a text box in the client.
- **Diverge:** add `clipboard-read` to the frame's permissions.

### Size

- The sheet can fill the window, and the display can follow the pane.
- **Diverge:** add `fullscreen` to the frame's permissions if true full
  screen is wanted. Not needed otherwise.

### Voice

The app has voice chat and dictation.

- Sound out needs a client that carries sound, which rules out A.
- Sound in needs the microphone, which the frame may not use, and a
  client that carries it.
- **Diverge:** add `microphone` to the frame's permissions. Even then
  this is the hardest feature to deliver, and I would leave it out at
  first.

### Files

- The app's file chooser sees the container's files, not the person's.
- The viewer's filesystem tab already uploads files into a container.
  Upload there, then choose the file in the app.
- **Diverge:** nothing needed.

### Links

- A link the app opens goes to the browser inside the container.
- **Diverge:** optional. A message from the frame asking the viewer to
  open an address on the person's own machine would make links behave as
  people expect. The viewer already listens for one kind of message from
  the frame, for the theme.

### Notifications

- The app's desktop notifications are raised inside the container and
  are seen only while the tab is open.
- **Diverge:** nothing that fits the frontend. An agent's run output is
  the natural place for anything the person must be told.

### Start-up time

- Each fresh container downloads about 118 MB and installs it before the
  app can open, then the app itself starts.
- The frontend should show a plain page saying so until the display is
  ready.
- **Diverge:** nothing needed. A cache on the host would shorten it; see
  [DOWNLOAD_APP_IN_IMAGE.md](DOWNLOAD_APP_IN_IMAGE.md).

### When nobody is looking

- With no tab open and no run, the container may stop. The app stops
  with it.
- The Bot does not. It works on the vendor's hosted computer whether or
  not any app is open.
- **Diverge:** nothing needed.

## 8. What Diverge would need to add

None of these is required for the frontend to work.

| Addition | For | Without it |
|---|---|---|
| `clipboard-read` on the frame | Pasting into the app | A paste box in the web client |
| `fullscreen` on the frame | True full screen | The sheet's fill-the-window control |
| `microphone` on the frame | Voice chat, dictation | No voice input |
| `autoplay` on the frame | Sound without a click first | Sound starts after the first click |
| "Open this address" message to the viewer | Links open on the person's machine | Links open in the container's browser |
| A stated size limit for secrets, or a volume for the agent | Keeping the sign-in across restarts | Signing in again after a restart |

The first four are one line each in the viewer, in the frame's `allow`
attribute. Whether to grant them to every frontend or only to images
that ask for them in `DIVERGE.md` is a design choice for Diverge: a
per-image declaration is the safer form.

## 9. How this sits with the terms

- **The image still contains none of the app.** It is fetched from the
  vendor at start.
- **The app is unmodified.** It is given two command-line flags.
- **A person is the user.** They look at the real app and operate it
  with their own hands, from another window. Nothing reads the app's
  internals and no program acts in the person's place. Of everything
  considered in these reports, this is the arrangement closest to
  ordinary use.
- **The agent's runs are a separate matter.** If a run later reads the
  same screen or types into the same app, the questions in
  [READING_APP.md](READING_APP.md) and
  [SUPERINTELLIGENCE_USING_APP.md](SUPERINTELLIGENCE_USING_APP.md) apply
  to the run. The frontend does not raise them.
- **One caution.** The vendor's app is designed to run on a person's own
  device. Running it on a provider's machine and viewing it remotely is
  not addressed anywhere in the terms or the documentation.

## 10. What is not known

- **Whether the current app runs at all** in a container with no screen
  and no graphics card. One community project reports that it does, on a
  different base image.
- **Whether sign-in completes** with a browser on a virtual display, and
  where the session is kept afterwards.
- **Whether the app refuses to start as root,** or needs other flags.
- **How much memory and processor** the whole arrangement takes.
- **Whether Xpra, KasmVNC and Selkies serve correctly beneath a path
  prefix,** and whether Selkies has a WebSocket mode.
- **How large a secret may be,** and what of a container's disk survives
  a restart.
- **How smooth the picture is** after passing through the viewer, the
  daemon, the provider and the proxy.

## 11. Suggested order of work

1. Run the app in a container on a VNC display by hand, outside Diverge,
   and see whether it starts and signs in.
2. Find where the session is kept and whether it survives a restart when
   that folder is restored.
3. Put noVNC beneath `/frontend` in our program, with the stamp checked,
   and set `"frontend": true`.
4. Try it through the viewer and judge the speed.
5. Decide on Xpra, and on which of the additions in section 8 are worth
   asking Diverge for.

## 12. Sources

- Diverge, at `bc3b52f46`:
  `diverge-sdk-rs/src/shared/containers/frontend/`,
  `diverge-container-proxy/src/serve/frontend.rs`,
  `diverge-viewer/src/sheet/frontend.ts`,
  `diverge-viewer/src-tauri/src/frontend.rs`,
  `diverge-viewer/FEATURES.md`,
  `diverge-examples/agents/human/`,
  `diverge-sdk-rs/src/shared/containers/secrets/mode.rs`.
- The vendor's documentation and community projects, as recorded in
  [GROK_BOT_INTERACTION.md](GROK_BOT_INTERACTION.md) and
  [GROK_BOT_IMAGE_TOS.md](GROK_BOT_IMAGE_TOS.md).
