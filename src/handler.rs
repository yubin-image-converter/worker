use crate::message::ImageConvertMessage;
use lapin::{message::Delivery, options::BasicAckOptions};
use serde_json;

pub async fn handle_message(delivery: Delivery) {
    match serde_json::from_slice::<ImageConvertMessage>(&delivery.data) {
        Ok(msg) => {
            println!("📦 변환 요청 도착! request_id = {}", msg.request_id);
            // TODO: 실제 처리 로직 or 더미 응답

            if let Err(e) = delivery.ack(lapin::options::BasicAckOptions::default()).await {
                eprintln!("❌ ack 실패: {:?}", e);
            }
        }
        Err(e) => {
            eprintln!("❌ 파싱 실패: {:?}", e);
        }
    }

}
