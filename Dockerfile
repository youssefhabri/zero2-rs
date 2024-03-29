# syntax=docker/dockerfile:1.0-experimental

ARG DEBIAN_RELEASE=bullseye

FROM rust:slim-${DEBIAN_RELEASE} AS builder

RUN apt-get update
RUN apt-get install -y build-essential pkg-config libssl-dev libpq-dev ca-certificates
WORKDIR /usr/src
RUN USER=root cargo new zero-two
WORKDIR /usr/src/zero-two

COPY Cargo.toml Cargo.lock ./
COPY assets ./assets
COPY src ./src
COPY z2-anilist ./z2-anilist
COPY z2-database ./z2-database
COPY z2-interactions ./z2-interactions
COPY z2-menu ./z2-menu

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=target \
    cargo build --release
RUN mkdir /tmp/zero-two
RUN --mount=type=cache,target=target cp target/release/zero-two /tmp/zero-two/


FROM bitnami/minideb:${DEBIAN_RELEASE}
RUN install_packages openssl libpq5 ca-certificates

COPY --from=builder /tmp/zero-two/zero-two .
COPY --from=builder /usr/src/zero-two/assets ./assets

RUN rm -rf /var/lib/{apt,dpkg,cache,log}/

ENTRYPOINT ["/zero-two"]
