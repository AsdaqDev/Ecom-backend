use axum::{
    http::StatusCode,
    response::{IntoResponse,Response},
    Json,
};
use serde_json::json;

pub enum AppError {
    duplicate,
    does_not_exist,
    lacks_permission,
    Auth_required,
}

impl IntoResponse for AppError{
    pub async fn into_response(self) -> Response {
        match self {
            AppError::duplicate => (
                StatusCode::CONFLICT,
                Json(json!({"error":"credencial already exists"}))
                )
                .into_response(),

            AppError::does_not_exist => (
                StatusCode::Not_Found,
                Json(json!({"error":"Credencial not found "}))
                )
                .into_response(),

            AppError::lacks_permission => (
                StatusCode::FORBIDDEN,
                Json(json!({"error":"User don't have permission"}))
                )
                .into_response(),

            AppError::Auth_required => (
                StatusCode::UNAUTHORIAZED,
                Json(json!({"error":"Please Login"}))
                )
                .into_response(),
        
        }
    }
}
