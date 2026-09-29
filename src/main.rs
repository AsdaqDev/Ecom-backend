pub mod app;
pub mod infrastructure;
pub mod modules;
pub mod shared;

use super::infrastructure::connection::connect_pool();
use super::app::state::AppState;
use tracing_subscriber::fmt


#[tokio(main)]
fn main() -> Result<(),>{
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let app = build_app()
    
    let app = app::router::route(state);
    let listener = tokio::net::Lisntener("0.0.0.0/3000").await?;

    axum::surve(listener,app).await?;
    ok(())
}

