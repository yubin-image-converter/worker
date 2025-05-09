use std::env;

pub fn nfs_root() -> String {
    env::var("NFS_ROOT").unwrap_or_else(|_| "../uploads".to_string())
}

pub fn public_upload_base_url() -> String {
    env::var("PUBLIC_UPLOAD_BASE_URL").unwrap_or_else(|_| "/uploads".to_string())
}
