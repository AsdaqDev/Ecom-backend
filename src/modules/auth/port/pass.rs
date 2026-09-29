use async-trait::async-trait;

#[async-trait]
pub trait Password{
    async fn Hash_Pass(&self,pass:ReqPass.password) -> Result<(), AppError>;
    async fn Pass_verify(&self,pass:ReqPass.password) -> Result<(), AppError>;
    
}
