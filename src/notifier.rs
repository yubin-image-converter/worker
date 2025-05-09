use anyhow::Result;
use serde::Serialize;
use tokio::time::{sleep, Duration};
use tokio_tungstenite::connect_async;
use futures_util::SinkExt;
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
    let ws_url = "ws://localhost:4001"; // ← path 없이 루트로 연결
    let max_retries = 10;

    let payload = AsciiCompletePayload {
        user_id,
        request_id,
        txt_url,
    };

    println!("🚀 [Rust] ASCII 변환 완료 알림 준비됨");
    println!("🧾 Payload → {:?}", serde_json::to_string(&payload)?);

    for attempt in 1..=max_retries {
        match connect_async(ws_url).await {
            Ok((mut ws_stream, _)) => {
                println!("✅ [Rust] WebSocket 연결 성공 ({})", ws_url);

                let json_msg = serde_json::to_string(&payload)?;
                ws_stream.send(Message::Text(json_msg)).await?;
                println!("📤 [Rust] 메시지 전송 완료");

                ws_stream.close(None).await?;
                println!("🔒 [Rust] 연결 종료 완료");

                return Ok(());
            }
            Err(e) => {
                eprintln!("❌ [Rust] 연결 실패 ({}회차): {}", attempt, e);
                if attempt >= max_retries {
                    eprintln!("🚨 [Rust] 최대 재시도 도달. 종료합니다.");
                    return Err(e.into());
                }

                println!("⏳ {}초 후 재시도 예정...", 2);
                sleep(Duration::from_secs(2)).await;
            }
        }
    }

    Ok(())
}