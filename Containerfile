# The unofficial Grok Bot agent image: the program alone.
#
# The container proxy is NOT in this image. The host injects it at
# runtime, and it may not be up at all until a request comes.
#
# The image ships NO Grok Bot app. The app is the vendor's software and
# is not redistributed here: the program fetches the current release
# from the vendor the moment it starts (src/install.rs) and installs it
# with apt, so a starting container needs the vendor's download hosts
# and the Debian mirrors reachable, and runs as root.
#
# Build from this repository's root:
#
#     podman build -f Containerfile .
#
# The Diverge SDK comes from GitHub at the commit Cargo.toml pins, so
# the build needs github.com reachable and nothing else beside this
# repository.

# The same Rust image, by the same digest, as the Diverge agent images.
FROM docker.io/library/rust:1.98.0@sha256:271849e998ffce5776454bbf98c5dc21baafc854ff8e566197908d3aca9a81e8 AS build

WORKDIR /build

COPY Cargo.toml Cargo.lock ./
COPY src src

RUN cargo build --release --locked

# Debian stable. Not pinned by digest yet; pin it when the image is
# first published.
FROM docker.io/library/debian:trixie-slim

# CA certificates so TLS works at all, for the webhook, the vendor's
# release feed and the package's download. Nothing of the app's own
# dependencies is installed here: apt resolves them at startup from
# what the package itself declares.
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=build /build/target/release/diverge-grok-bot-unofficial /usr/local/bin/diverge-grok-bot-unofficial

# The image's arguments schema and its description, read by the proxy.
COPY .diverge /.diverge

# The program's own server, on the loopback: 8080, or whatever `PORT`
# names. Nothing is exposed: the proxy the host injects is the only
# thing that dials it, and it is beside it.

ENTRYPOINT ["/usr/local/bin/diverge-grok-bot-unofficial"]
