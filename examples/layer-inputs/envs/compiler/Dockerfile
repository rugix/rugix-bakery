FROM docker.io/library/rust:1.95.0-slim-bookworm
RUN apt-get update \
    && apt-get install -y --no-install-recommends build-essential dpkg-dev liblzma-dev pkg-config \
    && rm -rf /var/lib/apt/lists/*
RUN cargo install cargo-deb --version 3.8.0 --locked
WORKDIR /work
