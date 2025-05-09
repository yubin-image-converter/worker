# 1. 빌드 스테이지
FROM rust:1.77 AS builder

WORKDIR /app

# 캐싱을 위해 먼저 dependencies만 복사
COPY ./Cargo.toml ./Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# 실제 소스 복사
COPY . .

# release 빌드
RUN cargo build --release

# 2. 런타임 스테이지
FROM debian:bullseye-slim

# 필요한 라이브러리 설치
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# 실행파일 복사
COPY --from=builder /app/target/release/worker /usr/local/bin/worker

CMD ["worker"]
