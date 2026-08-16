# Run the test suites covered by CI. bouncy_lab is excluded because it requires
# additional linker setup.
test:
    cd bouncy_instructor && cargo test --verbose
    cd bouncy_backend && cargo test --verbose
    cd bouncy_frontend && npm test

# Run the Rust lint checks configured in CI.
lint:
    cd bouncy_instructor && CARGO_TERM_COLOR=always RUSTFLAGS="-Dwarnings" cargo clippy --all-targets --all-features
    cd bouncy_backend && CARGO_TERM_COLOR=always RUSTFLAGS="-Dwarnings" cargo clippy --all-targets --all-features

# The OCI tag intentionally identifies the Node and Rust toolchains it contains.
ci_image := "docker.io/jakmeier/bouncy-feet:node-22.12.0-rust-1.91.0"
container_engine := env_var_or_default("CONTAINER_ENGINE", "podman")

# Build without the repository as build context; the Containerfile copies no local files.
ci-image-build:
    {{container_engine}} build --file Containerfile --tag {{ci_image}} /tmp

# Push to Docker Hub. Set CONTAINER_ENGINE=docker to use Docker rather than Podman.
ci-image-push: ci-image-build
    {{container_engine}} push {{ci_image}}
