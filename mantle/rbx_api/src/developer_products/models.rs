use serde::Deserialize;

use crate::models::AssetId;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDeveloperProductIconResponse {
    pub image_asset_id: AssetId,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDeveloperProductResponse {
    pub id: AssetId,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetDeveloperProductResponse {
    pub id: AssetId,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ListDeveloperProductResponseItem {
    pub product_id: AssetId,
    pub developer_product_id: AssetId,
    pub name: String,
    pub description: Option<String>,
    pub icon_image_asset_id: Option<AssetId>,
    pub price_in_robux: u32,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PriceInformation {
    pub default_price_in_robux: Option<u32>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListDeveloperProductResponseItemV2 {
    pub product_id: AssetId,
    pub developer_product_id: AssetId,
    pub name: String,
    pub description: Option<String>,
    pub icon_image_asset_id: Option<AssetId>,
    pub price_information: Option<PriceInformation>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListDeveloperProductsResponse {
    pub developer_products: Vec<ListDeveloperProductResponseItem>,
    pub next_page_cursor: Option<String>,
    pub final_page: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListDeveloperProductsResponseV2 {
    pub developer_products_overview: Vec<ListDeveloperProductResponseItemV2>,
    pub next_page_cursor: Option<String>,
}
