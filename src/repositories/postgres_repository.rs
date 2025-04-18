use sqlx::PgPool;
pub struct PostgresRepository {
    pub pool: PgPool,
}

#[allow(dead_code)]
impl PostgresRepository {
    // Tạo mới PostgresRepository
    pub fn new(pg_pool: PgPool) -> Self {
        PostgresRepository { pool: pg_pool }
    }
}
