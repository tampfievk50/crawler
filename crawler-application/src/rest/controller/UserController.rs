use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use tracing::info;
use uuid::Uuid;

use crawler_domain::dto::CreateUserCommand::CreateUserCommand;
use crawler_domain::dto::UpdateUserCommand::UpdateUserCommand;

use crate::exception::GlobalExceptionHandler::AppError;
use crate::rest::payload::CreateUserPayload::CreateUserPayload;
use crate::rest::payload::UpdateUserPayload::UpdateUserPayload;
use crate::rest::response::ApiResponse::ApiResponse;
use crate::state::AppState::AppState;

pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUserPayload>,
) -> Result<impl IntoResponse, AppError> {
    info!(username = %payload.username, "Received create user request");

    let command = CreateUserCommand {
        username: payload.username,
        email: payload.email,
        password: payload.password,
        full_name: payload.full_name,
    };

    let response = state.user_service.create_user(command).await.map_err(AppError::from)?;

    Ok((StatusCode::CREATED, Json(ApiResponse::success(response))))
}

pub async fn get_user_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    info!(user_id = %id, "Fetching user by ID");

    let response = state.user_service.find_user_by_id(id).await.map_err(AppError::from)?;

    Ok(Json(ApiResponse::success(response)))
}

pub async fn list_users(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    info!("Listing all users");

    let response = state.user_service.find_all_users().await.map_err(AppError::from)?;

    Ok(Json(ApiResponse::success(response)))
}

pub async fn update_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUserPayload>,
) -> Result<impl IntoResponse, AppError> {
    info!(user_id = %id, "Received update user request");

    let command = UpdateUserCommand {
        email: payload.email,
        password: payload.password,
        full_name: payload.full_name,
        is_active: payload.is_active,
    };

    let response = state.user_service.update_user(id, command).await.map_err(AppError::from)?;

    Ok(Json(ApiResponse::success(response)))
}

pub async fn delete_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    info!(user_id = %id, "Received delete user request");

    state.user_service.delete_user(id).await.map_err(AppError::from)?;

    Ok((StatusCode::NO_CONTENT, Json(ApiResponse::success(()))))
}
