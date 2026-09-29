use crate::modules::auth::feature::{
    register::service::RegisterService,
    login::service::LoginService,
    logout::service::LogoutService,
    forget_pass::service::ForgetService,
};

#[derive(Clone)]
pub struct AuthState{
    pub Register: RegisterService,
    pub Login : LoginService,
    pub Logout : LogoutService,
    pub Forget : ForgetService,
}
