mod config;
mod handler;
mod message;
mod notifier;
mod rabbitmq;
mod redis;

use lapin::{
    options::{
        BasicAckOptions, BasicConsumeOptions, ExchangeDeclareOptions, QueueBindOptions,
        QueueDeclareOptions,
    },
    types::FieldTable,
    ExchangeKind,
};
use rabbitmq::get_channel;

use crate::config::{rabbitmq_exchange, rabbitmq_queue, rabbitmq_routing_key};
use futures_util::stream::StreamExt;
use message::ImageConvertMessage;
// use tokio_amqp::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let exchange_name = rabbitmq_exchange();
    let queue_name = rabbitmq_queue();
    let routing_key = rabbitmq_routing_key();

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
                        if let Err(e) = channel
                            .basic_ack(delivery_tag, BasicAckOptions::default())
                            .await
                        {
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
