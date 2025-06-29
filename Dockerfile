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

FROM debian:bookworm-slim AS runtime
WORKDIR /app

RUN apt-get update && \
    apt-get install --no-install-recommends -y ca-certificates curl && \
    curl -Lo pandoc.deb \
    https://github.com/jgm/pandoc/releases/download/3.7.0.2/pandoc-3.7.0.2-1-amd64.deb && \
    apt-get install --no-install-recommends -y ./pandoc.deb && \
    apt-get purge -y curl && \
    apt-get autoremove -y && \
    rm -rf /var/lib/apt/lists/* pandoc.deb

COPY --from=builder /app/target/release/ganjoor-pretty ./
COPY static ./static

EXPOSE 8080
ENTRYPOINT ["./ganjoor-pretty"]
