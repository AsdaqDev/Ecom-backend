use async-trait::async-trait;


#[async-trait]
pub trait Repo{
    async fn find_by_email(&self,email:Email) -> Result<Option<User>, RepoError>;
    async fn find_by_id(&self,id:UserId) -> Result<Option<User>, RepoError>;
    async fn create_user(&self,user:User) -> Result<Option<User>, RepoError>;


}

pub enum RepoError{
    Notfound,
    Database,
}
