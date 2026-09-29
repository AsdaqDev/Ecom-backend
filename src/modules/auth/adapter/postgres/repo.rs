use sqlx::Pgpool;
use crate::modules::auth::{
    domain::{
        value_obj::{
            email::Email,
            userid:UserId,
        },
        entity::User,
    },
    port::{
        jwt::,
        pass::,
        user_repo::AuthRepo,
    }
}

pub struct PostgresRepo{
    pool: Pgpool,
}

impl AuthRepo for PostgresRepo{
    async fn find_by_email(&self,email:Email) -> Result<Option<User>, RepoError>{
        let user = sqlx::query_as!{User,r#"Select * from user where id : $1"#, id}.fetch_optional($self.pool)
            .await
            .map_err(RepoError::Database)?;
        Ok(user)
    }
    async fn find_by_id(&self,id:UserId) -> Result<Option<User>, RepoError>{
        let user = sqlx::query_as!{User,r#"Select * from user where email: $1"#, email}.fetch_optional(&self.pool)
        Ok(user)
    }
    async fn insert_user(&self,user:User) -> Result<Option<User>, RepoError>{
        let user =  sqlx::query!{r#"insert into user value($1,$2)"#, email user.password}.execute($self.pool)
    }


}
