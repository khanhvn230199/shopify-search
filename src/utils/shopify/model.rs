use std::str;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GraphQLResponse {
    pub data: Data,
    pub extensions: Option<Extensions>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Query<'a, V> {
    pub query: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<V>,
}

#[derive(Serialize, Debug)]
pub struct Variables<'a> {
    pub cursor: Option<&'a str>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub products: Products,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Products {
    pub edges: Option<Vec<ProductEdge>>,
    pub page_info: PageInfo,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductEdge {
    pub node: Option<ProductNode>,
    pub cursor: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductNode {
    pub id: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub vendor: Option<String>,
    pub product_type: Option<String>,
    pub tags: Option<Vec<String>>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub media: Media,
    pub variants: Variants,
    pub price_range_v2: Option<PriceRangeV2>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Media {
    pub edges: Option<Vec<MediaEdge>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MediaEdge {
    pub node: Option<MediaNode>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MediaNode {
    pub alt: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Variants {
    pub edges: Vec<VariantEdge>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VariantEdge {
    pub node: VariantNode,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VariantNode {
    pub id: Option<String>,
    pub title: Option<String>,
    pub price: Option<String>,
    pub sku: Option<String>,
    pub inventory_quantity: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PriceRangeV2 {
    pub min_variant_price: Option<Price>,
    pub max_variant_price: Option<Price>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Price {
    pub amount: Option<String>,
    pub currency_code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
    pub cost: Cost,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Cost {
    pub requested_query_cost: Option<i32>,
    pub actual_query_cost: Option<i32>,
    pub throttle_status: Option<ThrottleStatus>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ThrottleStatus {
    pub maximum_available: Option<i32>,
    pub currently_available: Option<i32>,
    pub restore_rate: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PageInfo {
    #[serde(rename = "endCursor")]
    pub end_cursor: Option<String>,

    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
}