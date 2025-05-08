use std::fs::{self, File};
use std::io::{Cursor, Write};
use std::path::PathBuf;

use chrono::Utc;
use image::io::Reader as ImageReader;
use image::ImageOutputFormat;
use crate::message::ImageConvertMessage;
use crate::rabbitmq::publish_progress;
use image::GenericImageView;
use crate::notifier::notify_ascii_complete;

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


/// NFS 저장 함수
fn save_ascii_to_nfs(
    user_id: &str,
    request_id: &str,
    ascii_art: &str,
) -> anyhow::Result<PathBuf> {
    let folder_name = format!("{}-{}", user_id, request_id);
    let dir_path = PathBuf::from(NFS_ROOT).join(folder_name);
    fs::create_dir_all(&dir_path)?;

    let timestamp = Utc::now().format("%Y%m%dT%H%M%SZ").to_string();
    let file_path = dir_path.join(format!("{}.txt", timestamp));

    let mut file = File::create(&file_path)?;
    file.write_all(ascii_art.as_bytes())?;
    file.flush()?;

    Ok(file_path)
}

fn convert_to_ascii(img: &image::DynamicImage) -> String {
    let grayscale = img.grayscale();
    let resized = grayscale.resize(160, 80, image::imageops::FilterType::Nearest); // 크기 조절

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

pub async fn handle_image_convert(msg: ImageConvertMessage) -> anyhow::Result<()> {
    let bytes = fs::read(&msg.path)?;
    let img = ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()?
        .decode()?;

    publish_progress(&msg.user_id, &msg.request_id, 0).await?;

    let ascii = convert_to_ascii(&img);

    let saved_path = save_ascii_to_nfs(&msg.user_id, &msg.request_id, &ascii)?;
    println!("[Worker] Saved ASCII art to {:?}", saved_path);

    notify_ascii_complete(&msg.request_id, &msg.user_id, saved_path.to_string_lossy().as_ref()).await?;

    publish_progress(&msg.user_id, &msg.request_id, 100).await?;

    Ok(())
}
