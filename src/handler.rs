use std::fs::{self, File};
use std::io::{Cursor, Write};
use std::path::{Path, PathBuf};
use anyhow::Context;
use crate::config::{nfs_root, public_upload_base_url};
use crate::message::ImageConvertMessage;
use crate::notifier::notify_ascii_complete;
use crate::rabbitmq::publish_progress;
use crate::redis::save_ascii_url_to_redis;

use chrono::Utc;
use image::io::Reader as ImageReader;
use image::{DynamicImage, GenericImageView};

pub async fn handle_image_convert(msg: ImageConvertMessage) -> anyhow::Result<()> {
    let img = load_image_from_path(&msg.path)?;

    publish_progress(&msg.user_id, &msg.request_id, 0).await?;

    let ascii = convert_to_ascii(&img);
    let saved_path = save_ascii_to_nfs(&msg.user_id, &msg.request_id, &ascii)?;
    println!("[Worker] Saved ASCII art to {:?}", saved_path);

    let txt_url = build_txt_url(&msg.user_id, &msg.request_id, &saved_path);

    if let Err(e) = save_ascii_url_to_redis(&msg.request_id, &txt_url).await {
        eprintln!("❌ Redis 저장 실패: {:?}", e);
    }

    notify_ascii_complete(&msg.request_id, &msg.user_id, &txt_url).await?;
    publish_progress(&msg.user_id, &msg.request_id, 100).await?;

    Ok(())
}

/// 이미지 파일 경로에서 Image 객체 로드
fn load_image_from_path(path: &str) -> anyhow::Result<DynamicImage> {
    let path = Path::new(path);

    ImageReader::new(Cursor::new(
        fs::read(path)
            .with_context(|| format!("파일 읽기 실패: {}", path.display()))?,
    ))
        .with_guessed_format()
        .context("이미지 포맷 자동 감지 실패")?
        .decode()
        .context("이미지 디코딩 실패") // png/jpg 등에서 깨졌을 때
}

/// 이미지 → ASCII 텍스트로 변환
fn convert_to_ascii(img: &DynamicImage) -> String {
    let grayscale = img.grayscale();
    let resized = grayscale.resize(160, 80, image::imageops::FilterType::Nearest);

    let chars = ["@", "#", "S", "%", "?", "*", "+", ";", ":", ",", "."];
    let mut ascii = String::new();

    for y in 0..resized.height() {
        for x in 0..resized.width() {
            let pixel = resized.get_pixel(x, y);
            let luma = pixel[0] as f32 / 255.0;
            let idx = (luma * (chars.len() - 1) as f32).round() as usize;
            ascii.push_str(chars[idx]);
        }
        ascii.push('\n');
    }

    ascii
}

/// ASCII 텍스트를 저장하고 파일 경로 반환
fn save_ascii_to_nfs(user_id: &str, request_id: &str, ascii_art: &str) -> anyhow::Result<PathBuf> {
    let folder_name = format!("{}-{}", user_id, request_id);
    let dir_path = PathBuf::from(nfs_root()).join(folder_name);
    fs::create_dir_all(&dir_path)?;

    let timestamp = Utc::now().format("%Y%m%dT%H%M%SZ").to_string();
    let file_path = dir_path.join(format!("{}.txt", timestamp));

    let mut file = File::create(&file_path)?;
    file.write_all(ascii_art.as_bytes())?;
    file.flush()?;

    Ok(file_path)
}

/// 저장된 ASCII 파일 경로로부터 공개 txtUrl 생성
fn build_txt_url(user_id: &str, request_id: &str, file_path: &PathBuf) -> String {
    let filename = file_path
        .file_name()
        .and_then(|f| f.to_str())
        .unwrap_or("output.txt");

    let folder = format!("{}-{}", user_id, request_id);
    format!("{}/{}/{}", public_upload_base_url(), folder, filename)
}
