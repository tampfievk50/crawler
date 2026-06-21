use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct DownloadVideoPayload {
    pub url: String,
}
