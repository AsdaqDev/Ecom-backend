use sqlx::{PgPool,RedisPool}

#[derive(Clone,Debug)]
pub struct AppState{
    pub pool: PgPool,
    pub redis: RedisPool,

    pub auth: Authstate,
    
}
