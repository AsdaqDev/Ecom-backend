use sqlx::{
    postgres::{PgPool,PgPoolOptions},
    Error,
}

pub async fn create_pool(config:&Config) -> Result<PgPool,Error> {
    PgPoolOptions::new()
        .max_connections(10)
        .min_connections(1)
        .connect(config.db_url)
        .await
}
