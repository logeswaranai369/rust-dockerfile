# Multi-stage Dockerfile — first stage compiles, second stage ships
# only the static binary on a minimal base image.
#
# Contixo's Dockerfile parser tracks the FINAL stage (after the second
# FROM) for CMD / ENTRYPOINT / EXPOSE / USER decisions. The EXPOSE
# 8000 below is what the wizard auto-detects → State A → customer
# hits Continue without typing.

# ----- Build stage ----------------------------------------------------
# Rust 1.85+ is required: a transitive dependency (sha1 v0.11) now uses
# Cargo's 2024 edition, which 1.74 can't parse. `rust:1-alpine` tracks
# the latest stable 1.x so this won't go stale as deps move forward.
FROM rust:1-alpine AS build
RUN apk add --no-cache musl-dev
WORKDIR /src
COPY Cargo.toml .
COPY src ./src
RUN cargo build --release

# ----- Runtime stage --------------------------------------------------
# Use a current Alpine so the runtime musl is >= the one the binary was
# built against (an older runtime musl can fail with missing symbols).
FROM alpine:3.20
RUN addgroup -S app && adduser -S app -G app
COPY --from=build /src/target/release/contixo-sample-rust /usr/local/bin/app
USER app
EXPOSE 8000
CMD ["/usr/local/bin/app"]
