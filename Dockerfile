# 1. 빌드 스테이지
FROM rust:1.81 AS builder

WORKDIR /app

COPY ./Cargo.toml ./Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

COPY . .
RUN cargo build --release

# 2. 런타임 스테이지
FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/worker /usr/local/bin/worker

CMD ["worker"]
