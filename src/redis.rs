use crate::config;
use redis::AsyncCommands;

pub async fn connect_redis() -> anyhow::Result<redis::aio::MultiplexedConnection>{
    let redis_url = config::redis_url();
    let client = redis::Client::open(redis_url)?;
    let conn = client.get_multiplexed_tokio_connection().await?;
    println!("[Redis] Connected!");
    Ok(conn)
}

pub async fn save_ascii_url_to_redis(request_id: &str, txt_url: &str) -> anyhow::Result<()> {
    let mut conn = connect_redis().await?;
    let key = format!("ascii_result:{}", request_id);
    let ttl_seconds = 3600; // 1시간

    let _: () = conn.set_ex(key, txt_url, ttl_seconds).await?;
    println!("[Redis] Saved txtUrl for {}", request_id);

    Ok(())
}
