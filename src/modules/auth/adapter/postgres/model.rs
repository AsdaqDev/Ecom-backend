use Uuid::Uuid;
use sqlx::ToSchema;

#[derive(ToSchema)]
pub struct UserModel{
    id: Uuid,
    name: Option<String>,
    email: Email,
    password: Password,
    DateTime: <TimeStamp>
}
