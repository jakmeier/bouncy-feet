# Forgejo CI runner image. Its tag identifies the Node and Rust toolchains.
ARG NODE_VERSION=22.12.0
ARG RUST_VERSION=1.91.0

FROM docker.io/node:${NODE_VERSION}-bookworm AS node
FROM docker.io/rust:${RUST_VERSION}-bookworm AS rust

RUN rustup target add wasm32-unknown-unknown \
 && rustup component add clippy \
 && cargo install wasm-pack --version 0.13.1 --locked \
 && cargo install cargo-deny --version 0.18.6 --locked

# Retain the full runner image's Docker, browser, and general build tooling.
FROM docker.io/catthehacker/ubuntu:full-24.04
ARG NODE_VERSION
ARG RUST_VERSION

# Copy only Node's runtime files. Copying all of /usr/local conflicts with the
# full runner image's pre-existing man-page directory.
COPY --from=node /usr/local/bin/node /usr/local/bin/node
COPY --from=node /usr/local/bin/npm /usr/local/bin/npm
COPY --from=node /usr/local/bin/npx /usr/local/bin/npx
COPY --from=node /usr/local/bin/corepack /usr/local/bin/corepack
COPY --from=node /usr/local/include/node /usr/local/include/node
COPY --from=node /usr/local/lib/node_modules /usr/local/lib/node_modules
COPY --from=rust /usr/local/cargo /usr/local/cargo
COPY --from=rust /usr/local/rustup /usr/local/rustup

ENV PATH="/usr/local/cargo/bin:/usr/local/bin:${PATH}" \
    RUSTUP_HOME=/usr/local/rustup

# Keep these versions in sync with the workflow and runner-image tag. Do not
# set CARGO_HOME here: the runner user needs a writable Cargo registry cache.
RUN test "$(node --version)" = "v${NODE_VERSION}" \
 && test "$(rustc --version | awk '{print $2}')" = "${RUST_VERSION}" \
 && wasm-pack --version \
 && cargo deny --version
