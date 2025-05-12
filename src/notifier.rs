use anyhow::Result;
use futures_util::SinkExt;
use once_cell::sync::Lazy;
use serde::Serialize;
use tokio::time::{sleep, Duration};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

const MAX_RETRIES: u8 = 10;
// 📌 여기에!
static WS_URL: Lazy<String> = Lazy::new(|| {
    std::env::var("WS_SERVER_URL").unwrap_or_else(|_| "ws://localhost:4001".to_string())
});

#[derive(Serialize, Debug)]
struct WsEventPayload<'a, T: Serialize> {
    event: &'a str,
    data: T,
}

// 기존: ASCII 변환 완료
#[derive(Serialize, Debug)]
pub struct AsciiCompleteData<'a> {
    #[serde(rename = "userId")]
    user_id: &'a str,
    #[serde(rename = "requestId")]
    request_id: &'a str,
    #[serde(rename = "txtUrl")]
    txt_url: &'a str,
}

// 추가: 진행률
#[derive(Serialize, Debug)]
pub struct ProgressUpdateData<'a> {
    #[serde(rename = "userId")]
    user_id: &'a str,
    #[serde(rename = "requestId")]
    request_id: &'a str,
    progress: u8,
}

// ✅ 공통 전송 함수
async fn send_ws_event<T: Serialize + std::fmt::Debug>(event: &str, data: T) -> Result<()> {
    let payload = WsEventPayload { event, data };
    let msg_json = serde_json::to_string(&payload)?;

    println!("📤 [Rust] WebSocket 메시지 준비됨 → {}", msg_json);

    for attempt in 1..=MAX_RETRIES {
        match connect_async(WS_URL.as_str()).await {
            Ok((mut ws_stream, _)) => {
                println!("✅ WebSocket 연결 성공");
                ws_stream.send(Message::Text(msg_json.clone())).await?;
                ws_stream.close(None).await?;
                println!("🔒 연결 종료 완료");
                return Ok(());
            }
            Err(e) => {
                eprintln!("❌ 연결 실패 ({}회차): {}", attempt, e);
                if attempt == MAX_RETRIES {
                    return Err(e.into());
                }
                println!("⏳ 재시도 중... (2초 대기)");
                sleep(Duration::from_secs(2)).await;
            }
        }
    }

    Ok(())
}

// 외부 공개용 함수 ① ASCII 변환 완료 알림
pub async fn notify_ascii_complete(user_id: &str, request_id: &str, txt_url: &str) -> Result<()> {
    send_ws_event(
        "ascii_complete",
        AsciiCompleteData {
            user_id,
            request_id,
            txt_url,
        },
    )
    .await
}

// 외부 공개용 함수 ② 진행률 업데이트 전송
pub async fn notify_progress_update(user_id: &str, request_id: &str, progress: u8) -> Result<()> {
    send_ws_event(
        "progress_update",
        ProgressUpdateData {
            user_id,
            request_id,
            progress,
        },
    )
    .await
}
