# diverge-grok-bot-unofficial

An unofficial [Diverge](https://github.com/divergenetwork/Diverge) agent
container that hands its messages to your own Grok Bot.

This project is not made, endorsed or supported by the makers of Grok
Bot. You need your own Grok Bot account, and its terms apply to you. The
image does not contain the Grok Bot app.

## Status

Early and experimental.

- **Works:** a run sends its messages' text to a Bot through a routine's
  webhook.
- **Does not exist yet:** reading the Bot's reply, files, a frontend.

See [.diverge/DIVERGE.md](.diverge/DIVERGE.md) for what the image does
and how to set it up.

## Layout

- `src/` is the program: an HTTP server the Diverge container proxy
  calls (`/register`, `/secrets`, `/secrets/collect`, `/run`,
  `/enqueue`).
- `.diverge/DIVERGE.md` is the image's document.
- `Containerfile` builds the image: `podman build -f Containerfile .`
- `reports/` holds the research: what Grok Bot offers, what its terms
  say, and how the pieces might fit.

## License

MIT. See [LICENSE](LICENSE).
