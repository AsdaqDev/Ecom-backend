pub struct Config{
    db_url,
    redis_url,
    jwt_secret,
}

impl Config {
    pub fn from_env() -> Result<Self,env::VarError> {
        Ok(
        Self{
            db_url: env::var("DATABASE_URL")?,
            redis_url: env::var("REDIS_URL")?,
            jwt_secret: env::var("JWT_SECRET")?,
            })
    }
}
