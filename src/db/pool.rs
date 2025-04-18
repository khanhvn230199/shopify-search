use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;

pub struct PostgresPool {
    pub pool: PgPool,
}

impl PostgresPool {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;

        Ok(PostgresPool { pool })
    }

    pub fn get_pool(&self) -> &PgPool {
        &self.pool
    }
}
