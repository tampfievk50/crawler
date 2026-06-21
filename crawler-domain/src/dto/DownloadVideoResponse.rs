use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct DownloadVideoResponse {
    pub id: Uuid,
    pub video_url: String,
    pub status: String,
    pub message: String,
    pub created_at: DateTime<Utc>,
}
