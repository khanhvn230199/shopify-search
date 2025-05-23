use async_trait::async_trait;
use crate::models::user::User;
use crate::models::error::ServiceResult;


#[async_trait]
#[allow(dead_code)]
pub trait UserService: 'static + Sync + Send {
    async fn find_user_by_email(&self, email: &str) -> ServiceResult<Option<User>>;
    async fn find_user_by_id(&self, user_id: &i32) -> ServiceResult<Option<User>>;
}