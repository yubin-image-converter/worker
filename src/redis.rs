use redis::AsyncCommands;

pub async fn connect_redis() -> anyhow::Result<redis::aio::Connection> {
    let client = redis::Client::open("redis://default:local@localhost:6379")?;
    let conn = client.get_tokio_connection().await?;
    println!("[Redis] Connected!");
    Ok(conn)
}

pub async fn save_ascii_url_to_redis(request_id: &str, txt_url: &str) -> anyhow::Result<()> {
    let mut conn = connect_redis().await?;
    let key = format!("ascii_result:{}", request_id);
    let ttl_seconds = 3600; // 1시간

    conn.set_ex(key, txt_url, ttl_seconds).await?;
    println!("[Redis] Saved txtUrl for {}", request_id);

    Ok(())
}
