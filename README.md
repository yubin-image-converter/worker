# Worker (Rust)

이 워커는 이미지 변환 작업을 비동기적으로 처리하는 Rust 기반 마이크로서비스입니다. RabbitMQ로부터 메시지를 수신하여 이미지를 ASCII 텍스트로 변환한 뒤, 결과를 Redis에 저장하거나 WebSocket을 통해 알림을 전송합니다.

---

## Links
<p>
  <a href="https://image-converter.yubinshin.com/" target="_blank">
    <img src="https://github.com/user-attachments/assets/6662efe8-3793-4128-aaf0-39d46b08a67e" width="600" alt="Image Converter Thumbnail" />
  </a>
</p>


- View Live Demo: [https://image-converter.yubinshin.com/](https://image-converter.yubinshin.com/)
- Architecture Overview: [Project README](https://github.com/yubin-image-converter)

---

## Directory Structure

```bash
.
├── src
│   ├── config.rs         # 설정 로딩 및 파싱
│   ├── handler.rs        # 이미지 처리 핸들러
│   ├── main.rs           # 애플리케이션 진입점
│   ├── message.rs        # 메시지 포맷 정의
│   ├── notifier.rs       # WebSocket 및 Redis 결과 전파
│   ├── rabbitmq.rs       # RabbitMQ 소비자 및 연결 관리
│   └── redis.rs          # Redis 클라이언트 래퍼
├── Cargo.toml            # Rust 프로젝트 설정 및 의존성
├── Dockerfile            # 컨테이너 이미지 정의
```

---

## Tech Stack

* 언어: Rust 2021 Edition
* 메시징: RabbitMQ + lapin
* 비동기 실행: Tokio
* 이미지 처리: image crate (PNG, JPEG, WebP 지원)
* 실시간 알림: tokio-tungstenite + Redis pub/sub
* 설정 관리: dotenv + config

---

## Features

* RabbitMQ 메시지 수신 (변환 요청)
* JPEG, PNG, WebP 이미지를 ASCII 텍스트로 변환
* 변환 결과를 Redis에 저장 또는 WebSocket으로 전송
* 실패한 작업에 대한 오류 처리 및 로깅
* Config 기반 유연한 설정 지원
* http polling 기반 health 체크 기능 구현 예정

---

## Build & Run

### Local

```bash
cargo build --release
cargo run
```

### Docker

```bash
docker build -t ascii-worker .
docker run --env-file .env ascii-worker
```

---


## Storage

변환된 ASCII 텍스트 파일은 Kubernetes 클러스터 내 uploads라는 이름의 PersistentVolumeClaim(PVC)을 통해 외부 NFS(Network File System)에 저장됩니다. 이 스토리지는 GCP VM에 구성된 NFS 서버를 기반으로 하며, API 서버 및 프론트엔드에서도 동일 경로를 통해 접근할 수 있습니다.

---

## Configuration

`.env` 또는 환경 변수로 설정:

```env
AMQP_URL=amqp://guest:guest@localhost:5672
RABBITMQ_EXCHANGE=image.convert.exchange
RABBITMQ_QUEUE=image.convert.queue
RABBITMQ_ROUTING_KEY=image.convert.routingKey
RABBITMQ_RESULT_QUEUE=image.convert.result.queue
NFS_ROOT=your-nfs-path
PUBLIC_UPLOAD_BASE_URL=/uploads
REDIS_HOST=localhost
REDIS_PORT=6379
REDIS_USER=default
REDIS_PASSWORD=your-redis-password
```

---

## ASCII Conversion Algorithm

이미지는 그레이스케일로 변환된 후, 픽셀 밝기 값을 기반으로 아래와 같은 문자셋을 이용해 ASCII 아트로 변환됩니다:

```txt
@%#*+=-:. 
```

밝기가 어두울수록 `@`, 밝을수록 공백에 가까운 문자를 사용합니다. 각 픽셀은 비율 보정 후 일정 너비의 문자 행으로 출력됩니다.

---

## WebSocket Message Format

변환 결과 전송 시 사용하는 메시지 포맷 예시는 다음과 같습니다:

```json
{
  "type": "ascii_complete",
  "requestId": "ulid-generated-id",
  "txtUrl": "https://cdn.image-converter.yubinshin.com/ascii/ulid.txt"
}
```

클라이언트는 이 메시지를 수신하여 상태를 `success`로 전환하고, 결과 파일을 로드합니다.

---

## Error Handling & Retry Strategy

* 모든 실패 로그는 `log` crate를 통해 기록되며, `anyhow::Result` 기반 에러 전파로 디버깅을 용이하게 합니다.
* RabbitMQ 컨슈머는 실패한 메시지를 ack하지 않음으로써 재전송이 유도되며, `lapin`의 QoS 설정으로 병렬 처리량을 제어합니다.
* WebSocket 연결 실패 시에는 로그만 기록하고, 변환 결과는 Redis에 저장되어 클라이언트에서 polling으로 조회할 수 있습니다.

---

## Author

**Yubin Shin**
Rust 기반 이미지 처리 워커 구현, 메시징/비동기 통신 설계
