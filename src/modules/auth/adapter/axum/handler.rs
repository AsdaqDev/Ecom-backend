use crate::shared::error::AppError;
use super::state::AuthState;
use super::feature::{
    register::dto::RegisterRequest,
    login::dto::LoginRequest,
    logout::dto::LogoutRequest,
    forget_pass::dto::ForgetRequest,
};

use axum::{
    extract::{Json,State},
    response::IntoResponse,
};

pub async fn Register(State(state):<AuthState>,Json(request): Json<RegisterRequest>) -> Result<impl IntoResponse,AppError> {
    let response = state.Register.execute(request).await;
    Ok(Json(response))
}
pub async fn Login(State(state):<AuthState>,Json(request): Json<LoginRequest>) -> Result<impl IntoResponse,AppError> {
    let response = state.Login.execute(request).await;
    Ok(Json(response))
}
pub async fn Logout(State(state):<AuthState>,Json(request): Json<LogoutRequest>) -> Result<impl IntoResponse,AppError> {
    let response = state.Logout.execute(request).await;
    Ok(Json(response))
}
pub async fn Forget_pass(State(state):<AuthState>,Json(request): Json<ForgetRequest>) -> Result<impl IntoResponse,AppError> {
    let response = state.Forget.execute(request).await;
    Ok(Json(response))
}
