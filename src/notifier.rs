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

    println!("📤 [Rust] WebSocket 이벤트 전송 준비");
    println!("🧾 이벤트 종류: {}", event);
    println!("📦 페이로드 데이터: {:?}", payload);
    println!("🛰️ WS 서버 주소: {}", WS_URL.as_str());

    for attempt in 1..=MAX_RETRIES {
        println!("🔄 WebSocket 연결 시도 ({}회차)", attempt);
        match connect_async(WS_URL.as_str()).await {
            Ok((mut ws_stream, _)) => {
                println!("✅ WebSocket 연결 성공");
                println!("📨 메시지 전송 중...");
                ws_stream.send(Message::Text(msg_json.clone())).await?;
                println!("📬 메시지 전송 완료");
                ws_stream.close(None).await?;
                println!("🔒 WebSocket 연결 종료 완료");
                return Ok(());
            }
            Err(e) => {
                eprintln!("❌ WebSocket 연결 실패 ({}회차): {}", attempt, e);
                if attempt == MAX_RETRIES {
                    eprintln!("🚨 최대 재시도 횟수 초과. 전송 포기.");
                    return Err(e.into());
                }
                println!("⏳ {}초 후 재시도 예정...", 2);
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
