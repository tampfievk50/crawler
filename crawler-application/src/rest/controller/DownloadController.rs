use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;
use tracing::info;
use uuid::Uuid;

use crawler_domain::dto::DownloadVideoCommand::DownloadVideoCommand;

use crate::exception::GlobalExceptionHandler::AppError;
use crate::rest::payload::DownloadVideoPayload::DownloadVideoPayload;
use crate::rest::response::ApiResponse::ApiResponse;

use crate::state::AppState::AppState;

pub async fn create_download(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<DownloadVideoPayload>,
) -> Result<impl IntoResponse, AppError> {
    info!(url = %payload.url, "Received download request");

    let command = DownloadVideoCommand { url: payload.url };

    let response = state
        .download_service
        .create_download(command)
        .await
        .map_err(AppError::from)?;

    Ok(Json(ApiResponse::success(response)))
}

pub async fn get_download_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    info!(download_id = %id, "Fetching download by ID");

    let response = state
        .download_service
        .find_download_by_id(id)
        .await
        .map_err(AppError::from)?;

    Ok(Json(ApiResponse::success(response)))
}

pub async fn list_downloads(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    info!("Listing all downloads");

    let response = state
        .download_service
        .find_all_downloads()
        .await
        .map_err(AppError::from)?;

    Ok(Json(ApiResponse::success(response)))
}
