use async_trait::async_trait;
use crate::models::product::Product;
use crate::models::error::{RepositoryError, RepositoryResult};
use crate::repositories::postgres_repository::PostgresRepository; // Giả sử bạn có repo Postgres
use crate::repositories::product_repository::interface::ProductRepository;

#[async_trait]
impl ProductRepository for PostgresRepository {
    // Insert sản phẩm vào database
    async fn insert_product(&self, product: Product) -> RepositoryResult<()> {
        // Truy vấn SQL để insert dữ liệu vào bảng products
        let query = r#"
            INSERT INTO products (
                id, title, description, vendor, product_type, tags, 
                created_at, updated_at, media_alt, variants, 
                min_price, max_price, currency_code, embedding
            )
            VALUES (
                $1, $2, $3, $4, $5, $6, 
                $7, $8, $9, $10, 
                $11, $12, $13, $14
            )
            ON CONFLICT (id) DO NOTHING
        "#;

        // Thực hiện truy vấn
        sqlx::query(query)
            .bind(&product.id)               // id
            .bind(&product.title)            // title
            .bind(&product.description)      // description
            .bind(&product.vendor)           // vendor
            .bind(&product.product_type)     // product_type
            .bind(&product.tags)             // tags
            .bind(product.created_at)        // created_at
            .bind(product.updated_at)        // updated_at
            .bind(&product.media_alt)        // media_alt
            .bind(&product.variants)         // variants (JSONB)
            .bind(product.min_price)         // min_price
            .bind(product.max_price)         // max_price
            .bind(&product.currency_code)    // currency_code
            .bind(&product.embedding)        // embedding (pgvector)
            .execute(&self.pool)             // thực thi query với pool
            .await
            .map_err(|err| RepositoryError {
                message: err.to_string(),
            })?;

        Ok(()) // Trả về Ok nếu không có lỗi
    }
}