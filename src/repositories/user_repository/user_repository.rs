use async_trait::async_trait;
use crate::models::user::User;
use crate::models::error::{RepositoryError, RepositoryResult};
use crate::repositories::postgres_repository::PostgresRepository; // Giả sử bạn có repo Postgres
use crate::repositories::user_repository::interface::UserRepository;

#[async_trait]
impl UserRepository for PostgresRepository {
    // Tìm người dùng theo email
    async fn find_user_by_email(&self, email: &str) -> Result<Option<User>, RepositoryError> {
        // Truy vấn SQL
        let query = "SELECT id, email, name FROM users WHERE email = $1";
        let result: Option<User> = sqlx::query_as(query)
            .bind(email)
            .fetch_optional(&self.pool) // fetch_optional trực tiếp với pool
            .await
            .map_err(|err| RepositoryError {
                message: err.to_string(),
            })?;

        Ok(result)
    }

    // Tìm người dùng theo id
    async fn find_user_by_id(&self, user_id: &i32) -> RepositoryResult<Option<User>> {
        // Truy vấn SQL
        let query = "SELECT id, name, email FROM users WHERE id = $1"; // PostgreSQL sử dụng $1 thay vì :user_id
        // Thực hiện truy vấn
        let result: Option<User> = sqlx::query_as(query)
            .bind(user_id)
            .fetch_optional(&self.pool) // fetch_optional trực tiếp với pool
            .await
            .map_err(|err| RepositoryError {
                message: err.to_string(),
            })?;

        Ok(result) // Trả về `Option<User>`
    }
}
