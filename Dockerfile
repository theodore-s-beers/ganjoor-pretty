FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare

FROM chef AS builder
COPY --from=planner /app/recipe.json ./
RUN cargo chef cook --release

COPY . .
RUN cargo build --release

FROM debian:trixie-slim AS runtime
WORKDIR /app

RUN apt-get update && \
    apt-get install --no-install-recommends -y ca-certificates curl jq && \
    PANDOC_VERSION=$(curl -s https://api.github.com/repos/jgm/pandoc/releases/latest | jq -r '.tag_name') && \
    curl -Lo pandoc.deb \
    https://github.com/jgm/pandoc/releases/download/${PANDOC_VERSION}/pandoc-${PANDOC_VERSION}-1-amd64.deb && \
    apt-get install --no-install-recommends -y ./pandoc.deb && \
    apt-get purge -y curl jq && \
    apt-get autoremove -y && \
    rm -rf /var/lib/apt/lists/* pandoc.deb

COPY --from=builder /app/target/release/ganjoor-pretty ./
COPY static ./static

EXPOSE 8080
ENTRYPOINT ["./ganjoor-pretty"]
