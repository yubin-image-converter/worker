use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ImageConvertMessage {
    pub request_id: String,
    pub user_id: String,
    pub original_filename: String,
    pub format: String,
    pub file: Vec<u8>,
}
