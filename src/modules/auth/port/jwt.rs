use async-trait::async-trait;

#[async-trait]
pub trait Jwt{
    async fn Jwt_Gen(&self,id:user.id) -> Result<(), AppError>;
    async fn Jwt_verify(&self,token:Request.token) -> Result<(), AppError>;
    
}
