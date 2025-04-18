use reqwest::Client;
use serde_json::json;
use crate::{models::product::{Product, ProductData}, utils::shopify::product_search::fetch_products_from_graphql};
use sqlx::PgPool;
use chrono::{DateTime, Utc};
use pgvector::Vector; 

pub async fn get_embedding(text: &str) -> Result<Vector, Box<dyn std::error::Error>> {
    let hf_token = std::env::var("HUGGING_FACE_TOKEN")?;
    let client = Client::new();
    
    // Gửi request POST đến Hugging Face API
    let res = client
        .post("https://api-inference.huggingface.co/pipeline/feature-extraction/thenlper/gte-small")
        .bearer_auth(hf_token)
        .json(&json!({ "inputs": text }))
        .send()
        .await?;

    // Lấy text để kiểm tra
    let text_body = res.text().await?;
    // Parse dữ liệu JSON thành Vec<f32> (do API trả về một mảng đơn giản của f32)
    let embedding_result: Result<Vec<f32>, _> = serde_json::from_str(&text_body);

    match embedding_result {
        Ok(embedding) => {
            Ok(Vector::from(embedding))
        }
        Err(e) => {
            println!("Error parsing JSON: {:?}  text: {}", e, text);
            Err("Failed to parse embedding response".into())
        }
    }
}

pub async fn get_embedding_async(pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let mut cursor: Option<String> = None;

    loop {
        let (data ,next_cursor, has_next_page) = fetch_products_from_graphql(cursor.as_deref()).await?;

        // Xử lý dữ liệu sản phẩm ở đây...
        for product in data.data.products.edges.unwrap() {
            if let Some(node) = product.node {
                if let Some(title) = node.title {
                    let title_str = title.as_str();
                    match get_embedding(title_str).await {
                        Ok(embedding) => {
                            let default_date = "2025-01-01T00:00:00Z".to_string();
                            let updated_at_str = node.updated_at.as_ref().unwrap_or(&default_date);
                            let updated_at = DateTime::parse_from_rfc3339(updated_at_str)
                                .unwrap()
                                .with_timezone(&Utc);
                            let created_at_str = node.created_at.as_ref().unwrap_or(&default_date);
                            let created_at = DateTime::parse_from_rfc3339(created_at_str)
                                .unwrap()
                                .with_timezone(&Utc);
                            // Lấy các trường cần thiết từ node một lần
                            let product = Product {
                                id: node.id.unwrap_or_default(), // Dùng unwrap_or_default() nếu id có thể là None
                                title: title_str.to_string(),
                                description: node.description.unwrap_or_default(), // Mặc định nếu description là None
                                vendor: node.vendor.unwrap_or_default(), // Mặc định nếu vendor là None
                                product_type: node.product_type.unwrap_or_default(),
                                tags: node.tags.unwrap_or_else(Vec::new),
                                created_at: created_at,
                                updated_at: updated_at,
                                variants: serde_json::json!(node.variants.edges),
                                media_alt: node.media.edges.unwrap_or_default()
                                    .into_iter()
                                    .filter_map(|x| x.node.and_then(|n| n.alt))
                                    .collect(),
                                min_price: node.price_range_v2.as_ref()
                                    .and_then(|price_range| price_range.min_variant_price.as_ref())
                                    .and_then(|price| price.amount.as_ref())
                                    .and_then(|amount| amount.parse::<f64>().ok())
                                    .unwrap_or_default(),
                                max_price: node.price_range_v2.as_ref()
                                    .and_then(|price_range| price_range.max_variant_price.as_ref())
                                    .and_then(|price| price.amount.as_ref())
                                    .and_then(|amount| amount.parse::<f64>().ok())
                                    .unwrap_or_default(),
                                currency_code: "".to_string(),
                                embedding: embedding,
                            };
                            let result = insert_product(pool.clone(), product).await?;
                            println!("{}", result);
                        }
                        Err(err) => {
                            println!("Error occurred: {}", err);
                        }
                    }
                } else {
                    println!("product title is None.");
                }
            } else {
                println!("product node is None.");
            }
            
        }

        // Nếu không còn sản phẩm tiếp theo, thoát vòng lặp.
        if !has_next_page {
            break;
        }

        // Cập nhật cursor cho lần truy vấn tiếp theo.
        cursor = next_cursor;

        println!("Cursor: {:?}", cursor);
    }

    Ok(())
}

pub async fn get_embedding_data(pool: PgPool, text: &str) -> Result<Vec<ProductData>, Box<dyn std::error::Error>> {
    match get_embedding(text).await{
        Ok(embedding) => {
            Ok(search_product(pool, embedding).await?)
        },
        Err(e) => Err(e.into())
    }
}

async fn insert_product(pool:PgPool , product: Product) -> Result<String, sqlx::Error> {
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
        .execute(&pool)             // thực thi query với pool
        .await?;
    Ok("Product inserted successfully".to_string())
}


async fn search_product(pool:PgPool , embedding: Vector) -> Result<Vec<ProductData>, sqlx::Error> {
    // Truy vấn SQL để tìm kiếm sản phẩm gần nhất
    let query = r#"
        SELECT title
        FROM products
        WHERE embedding <-> $1::vector < 0.5
        LIMIT 10;
    "#;
      // Truy vấn SQL với vector embedding trực tiếp
    let result = sqlx::query_as::<_, ProductData>(query)
        .bind(&embedding.to_vec())
        .fetch_all(&pool)
        .await?;
    Ok(result)  
    

}
    