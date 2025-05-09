use anyhow::Result;
use serde::Serialize;
use tokio::time::{sleep, Duration};
use tokio_tungstenite::connect_async;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::tungstenite::Message;

#[derive(Serialize)]
struct AsciiCompletePayload<'a> {
    #[serde(rename = "userId")]
    user_id: &'a str,
    #[serde(rename = "requestId")]
    request_id: &'a str,
    #[serde(rename = "txtUrl")]
    txt_url: &'a str,
}

pub async fn notify_ascii_complete(user_id: &str, request_id: &str, txt_url: &str) -> Result<()> {
    let url = "ws://localhost:4001";
    let mut retries = 0;
    let max_retries = 10;

    println!("🚀 [Rust] ASCII 변환 완료 이벤트 준비됨");
    println!(
        "🧾 Payload → userId={}, requestId={}, txtUrl={}",
        user_id, request_id, txt_url
    );

    loop {
        match connect_async(url).await {
            Ok((mut ws_stream, _)) => {
                println!("✅ [Rust] WebSocket 서버 연결됨: {}", url);

                let payload = AsciiCompletePayload {
                    user_id,
                    request_id,
                    txt_url,
                };

                let json_msg = serde_json::to_string(&payload)?;
                println!("📤 [Rust] 메시지 전송중...");

                ws_stream.send(Message::Text(json_msg)).await?;

                println!("✅ [Rust] 메시지 전송 완료");

                // optional: 서버 응답 수신
                if let Some(msg) = ws_stream.next().await {
                    println!("📥 [Rust] 서버 응답 수신: {:?}", msg);
                }

                break Ok(());
            }
            Err(e) => {
                retries += 1;
                eprintln!("❌ [Rust] 연결 실패 ({}회): {}", retries, e);

                if retries >= max_retries {
                    eprintln!("🚨 [Rust] 재시도 한계 도달. 중단합니다.");
                    break Err(e.into());
                }

                println!("⏳ [Rust] {}초 후 재시도 중...", 2);
                sleep(Duration::from_secs(2)).await;
            }
        }
    }
}