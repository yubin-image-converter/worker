# 1. 빌드 스테이지
FROM clux/muslrust:stable AS builder

WORKDIR /app
COPY . .

# ✅ OpenSSL 정적 링크를 위한 설정
ENV OPENSSL_STATIC=1
ENV PKG_CONFIG_ALLOW_CROSS=1

RUN cargo build --release

# 2. 런타임 스테이지
FROM alpine:3.19

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/worker /usr/local/bin/worker

ENTRYPOINT ["worker"]
