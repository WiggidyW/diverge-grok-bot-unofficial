# diverge-grok-bot-unofficial

An unofficial [Diverge](https://github.com/divergenetwork/Diverge) agent
that hands its messages to your own Grok Bot, and the tool it does it
through.

This project is not made, endorsed or supported by the makers of Grok
Bot. You need your own Grok Bot account, and its terms apply to you.
Neither image contains the Grok Bot app.

## The two images

- **`grok-bot-unofficial-agent`** is the agent: a thin wrapper. It
  declares the tool as its dependency and relays each run to it.
- **`grok-bot-unofficial-tool`** is the agent's tool, one container
  shared by every agent that declares it: where Grok Bot is reached. It
  serves each agent's turn on its API, and each turn names the webhook
  it is for. It will serve the frontend.

## Status

Early and experimental. Neither image has been built or run yet.

- **Written:** a run posts its messages to the tool; the tool sends
  their text to a Bot through a routine's webhook and streams the run's
  chunks back, or refuses up front if the webhook could not be called.
- **Does not exist yet:** reading the Bot's reply, files, a frontend.

See each image's document for what it does and how to set it up:
[the agent's](grok-bot-unofficial-agent/.diverge/DIVERGE.md) and
[the tool's](grok-bot-unofficial-tool/.diverge/DIVERGE.md).

## Layout

- `grok-bot-unofficial-agent/` and `grok-bot-unofficial-tool/` are the
  two programs, members of one Cargo workspace, each with its
  `Containerfile` and its `.diverge/DIVERGE.md`.
- Each image builds from this repository's root:
  `podman build -f grok-bot-unofficial-tool/Containerfile .`
- `reports/` holds the research: what Grok Bot offers, what its terms
  say, and how the pieces might fit.

## License

MIT. See [LICENSE](LICENSE).
