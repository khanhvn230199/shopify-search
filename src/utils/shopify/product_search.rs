
use reqwest::Client;
use std::error::Error;
use crate::utils::shopify::model::*;

#[allow(dead_code)]
pub async fn fetch_products_from_graphql(cursor: Option<&str>) -> Result<(GraphQLResponse, Option<String>, bool), Box<dyn Error>>{ 
let url = "http://localhost:3457/graphiql/graphql.json?key=&api_version=2025-04";

let query = r#"
query($cursor: String){
  products(first: 250, after: $cursor) {
    edges {
      node {
        id
        title
        description
        vendor
        productType
        tags
        createdAt
        updatedAt
        media(first: 5) {
          edges {
            node {
              alt
            }
          }
        }
        variants(first: 5) {
          edges {
            node {
              id
              title
              price 
              sku
              inventoryQuantity
            }
          }
        }
        priceRangeV2 {
          minVariantPrice {
            amount
            currencyCode
          }
          maxVariantPrice {
            amount
            currencyCode
          }
        }
      }
    cursor
    }
    pageInfo{
      endCursor
      hasNextPage
    }
  }
}

"#;

let client = Client::new();

let variables = Variables {
  cursor: cursor.as_deref(),
};
let request_body = Query {
  query,
  variables: Some(variables),
};
  
let res = client.post(url).json(&request_body).send().await?;

let response_text = res.text().await?;

let graphql_response: GraphQLResponse = serde_json::from_str(&response_text)?;

// println!("{:#?}", print_graphql_response(&graphql_response));

let page_info = &graphql_response.data.products.page_info;

let has_next_page = page_info.has_next_page;
let end_cursor = page_info.end_cursor.clone(); 

Ok((graphql_response,end_cursor,has_next_page))
}