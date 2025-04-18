use reqwest::Client;
use serde_json::json;
use crate::utils::shopify::product_search::fetch_products_from_graphql;

pub async fn get_embedding(text: &str) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
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
            Ok(embedding)
        }
        Err(e) => {
            println!("Error parsing JSON: {:?}", e);
            Err("Failed to parse embedding response".into())
        }
    }
}

pub async fn get_embedding_async() -> Result<(), Box<dyn std::error::Error>> {
    let mut cursor: Option<String> = None;

    loop {
        let (data ,next_cursor, has_next_page) = fetch_products_from_graphql(cursor.as_deref()).await?;

        // Xử lý dữ liệu sản phẩm ở đây...
        for product in data.data.products.edges.unwrap() {
            if let Some(title) = product.node.unwrap().title {
                let title_str = title.as_str(); 
                match get_embedding(title_str).await {
                    Ok(_) => {
                        println!("Received embedding: {:?}", title_str);
                    }
                    Err(err) => {
                        println!("Error occurred: {}", err);
                    }
                }
            }else {
                println!("product description is None.");
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