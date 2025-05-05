use crate::handler::handle_message;
use crate::message::ImageConvertMessage;
use lapin::{options::*, types::FieldTable, Connection, ConnectionProperties, Consumer};
use tokio_stream::StreamExt;

pub async fn start_consumer() -> Result<(), Box<dyn std::error::Error>> {
    let amqp_url = std::env::var("AMQP_URL")
        .unwrap_or("amqp://guest:guest@localhost:5672/%2f".to_string());

    let conn = Connection::connect(&amqp_url, ConnectionProperties::default()).await?;
    println!("✅ RabbitMQ 연결 완료");

    let channel = conn.create_channel().await?;
    let queue_name = "convert.image";

    channel.queue_declare(queue_name, QueueDeclareOptions::default(), FieldTable::default()).await?;

    let mut consumer: Consumer = channel
        .basic_consume(queue_name, "worker-tag", BasicConsumeOptions::default(), FieldTable::default())
        .await?;

    println!("🟢 메시지 대기 중...");

    while let Some(result) = consumer.next().await {
        if let Ok(delivery) = result {
            handle_message(delivery).await;
        }
    }

    Ok(())
}
