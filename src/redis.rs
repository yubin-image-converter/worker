use crate::config;
use redis::AsyncCommands;

pub async fn connect_redis() -> anyhow::Result<redis::aio::MultiplexedConnection> {
    let redis_url = config::redis_url();
    println!("🧪 [Redis] Connecting to URL: {}", redis_url); // 디버깅 로그

    let client = redis::Client::open(redis_url.clone())
        .map_err(|e| {
            eprintln!("❌ [Redis] Failed to create client with URL {}: {:?}", redis_url, e);
            e
        })?;

    let conn = client
        .get_multiplexed_tokio_connection()
        .await
        .map_err(|e| {
            eprintln!("❌ [Redis] Failed to get connection for URL {}: {:?}", redis_url, e);
            e
        })?;

    println!("✅ [Redis] Connected successfully to {}", redis_url);
    Ok(conn)
}

pub async fn save_ascii_url_to_redis(request_id: &str, txt_url: &str) -> anyhow::Result<()> {
    println!("📥 [Redis] Preparing to save txtUrl → request_id: {}, url: {}", request_id, txt_url);

    let mut conn = connect_redis().await?;

    let key = format!("ascii_result:{}", request_id);
    let ttl_seconds = 3600;

    let result = conn.set_ex(&key, txt_url, ttl_seconds).await;

    match result {
        Ok(_) => println!("✅ [Redis] Saved key: {}, ttl: {}s", key, ttl_seconds),
        Err(e) => eprintln!("❌ [Redis] Failed to save key: {}, error: {:?}", key, e),
    }

    Ok(())
}
