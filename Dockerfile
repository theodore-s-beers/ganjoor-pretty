FROM rust:latest AS builder

WORKDIR /usr/src/app
COPY . .

# Build and cache the binary and dependent crates in release mode
RUN --mount=type=cache,target=/usr/local/cargo,from=rust:latest,source=/usr/local/cargo \
    --mount=type=cache,target=target \
    cargo build --release && mv ./target/release/ganjoor-pretty ./ganjoor-pretty

# Runtime image
FROM debian:bookworm-slim

# Install Pandoc
RUN apt update && apt install -y curl && apt clean
RUN curl -LO https://github.com/jgm/pandoc/releases/download/3.1.6.2/pandoc-3.1.6.2-1-amd64.deb
RUN dpkg -i pandoc-3.1.6.2-1-amd64.deb

# Run as "app" user
RUN useradd -ms /bin/bash app

USER app
WORKDIR /app

# Get compiled binaries from builder's cargo install directory
COPY --from=builder /usr/src/app/ganjoor-pretty /app/ganjoor-pretty

# Copy static directory
COPY static /app/static

# Run app
CMD ./ganjoor-pretty
