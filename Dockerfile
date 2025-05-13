# 1. 빌드 스테이지 - musl로 빌드
FROM rust:1.81 AS builder

WORKDIR /app

RUN rustup target add x86_64-unknown-linux-musl

COPY ./Cargo.toml ./Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release --target x86_64-unknown-linux-musl
RUN rm -rf src

COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

# 2. 런타임 스테이지 - 매우 슬림한 베이스
FROM alpine:3.19

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/worker /usr/local/bin/worker

ENTRYPOINT ["worker"]
