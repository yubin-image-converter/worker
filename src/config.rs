use std::env;

/// AMQP 브로커 URL (RabbitMQ 접속용)
pub fn amqp_url() -> String {
    env::var("AMQP_URL").unwrap_or_else(|_| "amqp://guest:guest@localhost:5672".to_string())
}

/// 이미지 변환 요청 메시지를 받을 RabbitMQ 익스체인지
pub fn rabbitmq_exchange() -> String {
    env::var("RABBITMQ_EXCHANGE").unwrap_or_else(|_| "image.convert.exchange".to_string())
}

/// 변환 요청 메시지를 받을 큐 이름
pub fn rabbitmq_queue() -> String {
    env::var("RABBITMQ_QUEUE").unwrap_or_else(|_| "image.convert.queue".to_string())
}

/// 메시지 라우팅 키
pub fn rabbitmq_routing_key() -> String {
    env::var("RABBITMQ_ROUTING_KEY").unwrap_or_else(|_| "image.convert.routingKey".to_string())
}

// /// ASCII 변환 결과를 받을 큐 이름
// pub fn rabbitmq_result_queue() -> String {
//     env::var("RABBITMQ_RESULT_QUEUE").unwrap_or_else(|_| "image.convert.result.queue".to_string())
// }

/// 로컬 파일 저장 루트 (NFS 공유 루트)
pub fn nfs_root() -> String {
    env::var("NFS_ROOT")
        .unwrap_or_else(|_| "../uploads".to_string())
}

/// 퍼블릭 URL 경로 (클라이언트에서 접근하는 파일 경로 베이스)
pub fn public_upload_base_url() -> String {
    env::var("PUBLIC_UPLOAD_BASE_URL")
        .unwrap_or_else(|_| "/uploads".to_string())
}

/// Redis 접속 URL
pub fn redis_url() -> String {
    env::var("REDIS_URL").unwrap_or_else(|_| "redis://default:local@localhost:6379".to_string())
}

pub fn progress_exchange() -> String {
    std::env::var("PROGRESS_EXCHANGE").unwrap_or_else(|_| "progress_exchange".to_string())
}