use crate::modules::auth::port::{
    Repo,
    Jwt,
    Pass,
}
pub struct AuthDeps{
    repo: Repo,
    jwt: Jwt,
    pass: Pass,
}
