FROM docker.io/debian:bookworm-slim

# 安装 OpenSSL
RUN --mount=type=cache,target=/var/cache/apt \
    --mount=type=cache,target=/var/lib/apt \
    apt-get update && apt-get install libssl3 ca-certificates --yes && rm -rf /var/lib/apt/lists/*

WORKDIR /app

## copy the main binary
COPY Rocket.toml ./
COPY main ./main

## ensure the container listens globally on port 8080
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080

ENTRYPOINT ["/app/main"]