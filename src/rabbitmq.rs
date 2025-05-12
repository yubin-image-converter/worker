use crate::config;
use lapin::{
    options::ExchangeDeclareOptions, types::FieldTable, Channel, Connection, ConnectionProperties,
    ExchangeKind,
};
use tokio::sync::OnceCell;

static CHANNEL: OnceCell<Channel> = OnceCell::const_new();

/// 채널 초기화 + exchange 선언
pub async fn get_channel() -> anyhow::Result<Channel> {
    if let Some(channel) = CHANNEL.get() {
        return Ok(channel.clone());
    }

    let addr = config::amqp_url(); // ✅ .env에서 불러온 값 사용
    let conn = Connection::connect(&addr, ConnectionProperties::default()).await?;
    let channel = conn.create_channel().await?;

    channel
        .exchange_declare(
            &config::progress_exchange(), // ✅ 여기서도 불러오기
            ExchangeKind::Direct,
            ExchangeDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;

    CHANNEL.set(channel.clone())?;
    Ok(channel)
}
// 진행률 메시지 전송
// pub async fn publish_progress(
//     user_id: &str,
//     request_id: &str,
//     progress: u8,
// ) -> anyhow::Result<()> {
//     let message = ImageProgressMessage {
//         user_id: user_id.to_string(),
//         request_id: request_id.to_string(),
//         progress,
//     };
//
//     let payload = to_vec(&message)?; // JSON 직렬화
//     let channel = get_channel().await?;
//
//     channel
//         .basic_publish(
//             "progress_exchange",
//             "progress", // 라우팅 키
//             BasicPublishOptions::default(),
//             &payload,
//             BasicProperties::default(),
//         )
//         .await?
//         .await?; // confirm
//
//     Ok(())
// }
