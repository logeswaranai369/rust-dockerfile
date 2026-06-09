# Multi-stage Dockerfile — first stage compiles, second stage ships
# only the static binary on a minimal base image.
#
# Contixo's Dockerfile parser tracks the FINAL stage (after the second
# FROM) for CMD / ENTRYPOINT / EXPOSE / USER decisions. The EXPOSE
# 8000 below is what the wizard auto-detects → State A → customer
# hits Continue without typing.

# ----- Build stage ----------------------------------------------------
FROM rust:1.74-alpine AS build
RUN apk add --no-cache musl-dev
WORKDIR /src
COPY Cargo.toml .
COPY src ./src
RUN cargo build --release

# ----- Runtime stage --------------------------------------------------
FROM alpine:3.18
RUN addgroup -S app && adduser -S app -G app
COPY --from=build /src/target/release/contixo-sample-rust /usr/local/bin/app
USER app
EXPOSE 8000
CMD ["/usr/local/bin/app"]
