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
    let url = "ws://localhost:4000/ascii";
    let mut retries = 0;
    let max_retries = 10;

    loop {
        match connect_async(url).await {
            Ok((mut ws_stream, _)) => {
                println!("✅ Connected to WebSocket server");

                let payload = AsciiCompletePayload {
                    user_id,
                    request_id,
                    txt_url,
                };

                let json_msg = serde_json::to_string(&payload)?;
                ws_stream.send(Message::Text(json_msg)).await?;

                // optional: 응답 수신
                if let Some(msg) = ws_stream.next().await {
                    println!("서버 응답: {:?}", msg);
                }

                break Ok(()); // 성공 시 루프 종료
            }
            Err(e) => {
                retries += 1;
                eprintln!("❌ 연결 실패 ({}회): {}", retries, e);

                if retries >= max_retries {
                    eprintln!("🚨 재시도 한계 도달. 중단합니다.");
                    break Err(e.into());
                }

                sleep(Duration::from_secs(2)).await;
            }
        }
    }
}