use axum::Router;

use state::AppState;

use crate::modules::auth::adapter::axum::route::auth_routes;

pub async fn App_route(state:AppState) -> Router{
    Router::new()
        .nest("/v1/auth",auth_routes())
        .with_state(AppState.auth)
}
