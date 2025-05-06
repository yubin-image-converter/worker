use std::fs::{self, File};
use std::io::{Cursor, Write};
use std::path::PathBuf;

use chrono::Utc;
use image::io::Reader as ImageReader;
use image::ImageOutputFormat;
use crate::message::ImageConvertMessage;
use crate::rabbitmq::publish_progress;

const NFS_ROOT: &str = "./uploads"; // 로컬 개발용 경로

/// 포맷 문자열을 ImageOutputFormat으로 변환
fn parse_format(format_str: &str) -> Option<ImageOutputFormat> {
    match format_str.to_lowercase().as_str() {
        "png" => Some(ImageOutputFormat::Png),
        "jpeg" | "jpg" => Some(ImageOutputFormat::Jpeg(80)),
        "webp" => Some(ImageOutputFormat::WebP),
        _ => None,
    }
}

pub async fn handle_image_convert(msg: ImageConvertMessage) -> anyhow::Result<()> {
    let bytes = fs::read(&msg.path)?;
    let img = ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()?
        .decode()?;

    // 0% 시작 메시지
    publish_progress(&msg.user_id, &msg.request_id, 0).await?;

    // 포맷 변환
    let Some(format) = parse_format(&msg.target_format) else {
        anyhow::bail!("Unsupported image format: {}", msg.target_format);
    };

    let mut buf = Cursor::new(Vec::new());
    img.write_to(&mut buf, format)?;
    let buf = buf.into_inner();

    // 저장
    let saved_path = save_image_to_nfs(&msg.user_id, &msg.request_id, &buf, &msg.target_format)?;
    println!("[Worker] Saved converted image to {:?}", saved_path);

    // 100% 완료 메시지
    publish_progress(&msg.user_id, &msg.request_id, 100).await?;

    Ok(())
}

/// NFS 저장 함수
fn save_image_to_nfs(
    user_id: &str,
    request_id: &str,
    image_bytes: &[u8],
    ext: &str,
) -> anyhow::Result<PathBuf> {
    let folder_name = format!("{}-{}", user_id, request_id);
    let dir_path = PathBuf::from(NFS_ROOT).join(folder_name);
    fs::create_dir_all(&dir_path)?; // 폴더 없으면 생성

    let timestamp = Utc::now().format("%Y%m%dT%H%M%SZ").to_string();
    let file_path = dir_path.join(format!("{}.{}", timestamp, ext));

    let mut file = File::create(&file_path)?;
    file.write_all(image_bytes)?;
    file.flush()?;

    Ok(file_path)
}