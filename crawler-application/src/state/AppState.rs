use std::sync::Arc;
use crawler_domain::port::input::DownloadPort::DownloadPort;

pub struct AppState {
    pub download_service: Arc<dyn DownloadPort>,
}
