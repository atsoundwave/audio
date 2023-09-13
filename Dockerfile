FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

ARG SQLX_OFFLINE=true

COPY --from=planner /recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release

# We do not need the Rust toolchain to run the binary!
FROM debian:buster-slim AS runtime
WORKDIR /
COPY --from=builder /target/release/audio /usr/local/bin

RUN apt-get update && apt-get install -y openssl ca-certificates && rm -rf /var/lib/apt/lists/*

ENTRYPOINT ["/usr/local/bin/audio"]