use chrono::{DateTime, Utc};
use sqlx::FromRow;
use pgvector::Vector;

#[derive(Debug, FromRow)]
pub struct Product {
    pub id: String,
    pub title: String,
    pub description: String,
    pub vendor: String,
    pub product_type: String,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub media_alt: Vec<String>,
    pub variants: serde_json::Value,
    pub min_price: f64,
    pub max_price: f64,
    pub currency_code: String,
    pub embedding: Vector, // vector để dùng pgvector
}

#[derive(Debug, FromRow)]
pub struct ProductData {
    pub title: String,
}