use axum::{Router,routing::post}
use super::handler::{
    Register,
    Login,
    Logout,
    Forget_pass,
}

pub async fn auth_routes() -> Router{
    Router::new()
        .route("/register" , post(Register)),
        .route("/login" , post(Login)),
        .route("/logout" , post(Logout)),
        .route("/forget_pass" , post(Forget_pass),

}
