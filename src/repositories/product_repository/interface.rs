use async_trait::async_trait;
use crate::models::product::Product;
use crate::models::error::{RepositoryResult};

#[async_trait]
pub trait ProductRepository: Send + Sync{
   // Insert product vào database
   async fn insert_product(&self, product: Product) -> RepositoryResult<()>;
}