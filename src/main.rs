mod handler;
mod message;
mod rabbitmq;

use dotenv::dotenv;
use lapin::{
    options::{BasicAckOptions, BasicConsumeOptions, ExchangeDeclareOptions, QueueBindOptions, QueueDeclareOptions},
    types::FieldTable,
    ExchangeKind,
};
use rabbitmq::get_channel;

use futures_util::stream::StreamExt;
use message::ImageConvertMessage;
use std::env;
use tokio_amqp::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let exchange_name = env::var("RABBITMQ_EXCHANGE").unwrap_or("image.convert.exchange".to_string());
    let queue_name = env::var("RABBITMQ_QUEUE").unwrap_or("image.convert.queue".to_string());
    let routing_key = env::var("RABBITMQ_ROUTING_KEY").unwrap_or("image.convert.routingKey".to_string());

    let channel = get_channel().await?;

    channel
        .exchange_declare(
            &exchange_name,
            ExchangeKind::Direct,
            ExchangeDeclareOptions {
                durable: true,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await?;

    channel
        .queue_declare(
            &queue_name,
            QueueDeclareOptions {
                durable: true,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await?;

    channel
        .queue_bind(
            &queue_name,
            &exchange_name,
            &routing_key,
            QueueBindOptions::default(),
            FieldTable::default(),
        )
        .await?;

    let mut consumer = channel
        .basic_consume(
            &queue_name,
            "image_consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    println!("✅ Waiting for image convert messages...");

    while let Some(delivery_result) = consumer.next().await {
        if let Ok(delivery) = delivery_result {
            let data = delivery.data.clone();

            // 메시지 파싱 및 처리
            let parsed: Result<ImageConvertMessage, _> = serde_json::from_slice(&data);

            match parsed {
                Ok(msg) => {
                    println!("📦 Received message: {:?}", msg);
                    let delivery_tag = delivery.delivery_tag;

                    // spawn async task
                    let channel = channel.clone();
                    tokio::spawn(async move {
                        if let Err(e) = handler::handle_image_convert(msg).await {
                            eprintln!("❌ Failed to handle message: {:?}", e);
                            return;
                        }

                        // ack 처리
                        if let Err(e) = channel.basic_ack(delivery_tag, BasicAckOptions::default()).await {
                            eprintln!("❌ Failed to ack message: {:?}", e);
                        }
                    });
                }
                Err(e) => {
                    eprintln!("❌ Failed to parse message: {:?}", e);
                }
            }
        }
    }

    Ok(())
}