mod rabbitmq;
mod message;
mod handler;

#[tokio::main]
async fn main() {
    println!("🚀 워커 시작");

    if let Err(e) = rabbitmq::start_consumer().await {
        eprintln!("❌ 에러 발생: {:?}", e);
    }
}
