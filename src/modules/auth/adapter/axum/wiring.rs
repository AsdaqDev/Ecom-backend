pub async fn build_auth(config:&Config,pool:&Pool) -> Router{
    let repo = UserRepo::new(pool)
    let jwt = Jwt::new(config.jwt_secret)
    let pass = Pass::new(pool)

    let deps = AuthDeps{
        repo,
        jwt,
        pass,
    }

    let register = RegisterService::new(deps.clone())

    let login = LoginService::new(deps.clone())

    let logout = LogoutService::new(deps.clone())

    let state = AuthState{
        register,
        login,
        logout,
    }
    routes.with_state(state)
}
