FROM docker.io/rust:1.74.1-slim-bookworm AS builder
ARG TARGETARCH=amd64

RUN apt-get update && apt-get install -y curl ca-certificates clang && rm -rf /var/lib/apt/lists/*
WORKDIR /app

COPY ./src ./src
COPY ./Cargo.toml ./
COPY ./Cargo.lock ./
RUN cargo build --release

FROM docker.io/debian:bookworm-slim AS runtime
WORKDIR /app
RUN mkdir /app/data
COPY --from=builder /app/target/release/bouncy_stats /usr/local/bin
EXPOSE 3000
CMD ["/usr/local/bin/bouncy_stats"]
