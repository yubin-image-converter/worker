use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ImageConvertMessage {
    pub request_id: String,
    pub user_id: String,
    pub path: String,
    #[allow(dead_code)]
    pub target_format: String, // ex: "png", "jpeg", "webp"
}

#[derive(Debug, Serialize)]
pub struct ImageProgressMessage {
    pub user_id: String,
    pub request_id: String,
    pub progress: u8, // 0 ~ 100
}