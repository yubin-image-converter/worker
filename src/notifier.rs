use socketio::ClientBuilder;
use serde_json::json;
use anyhow::Result;

pub async fn notify_ascii_complete(request_id: &str, user_id: &str, txt_url: &str) -> Result<()> {
    let socket = ClientBuilder::new("http://localhost:4000/ascii") // OK        .on("connect", |_| {
            println!("✅ Connected to Socket.IO server");
        })
        .on("error", |err, _| {
            eprintln!("❌ Socket.IO error: {:?}", err);
        })
        .connect()
        .await?;

    let payload = json!({
        "event": "ascii_complete",
        "requestId": request_id,
        "userId": user_id,
        "txtUrl": txt_url,
    });

    socket.emit("ascii_complete", payload).await?;

    Ok(())
}
