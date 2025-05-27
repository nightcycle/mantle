pub mod models;

use std::path::PathBuf;

use models::ListDeveloperProductsResponseV2;
use reqwest::{header, multipart::Form};
use serde_json::json;

use crate::{
    errors::RobloxApiResult,
    helpers::{get_file_part, handle, handle_as_json},
    models::AssetId,
    RobloxApi,
};

use self::models::{
    CreateDeveloperProductIconResponse, CreateDeveloperProductResponse,
    GetDeveloperProductResponse, ListDeveloperProductResponseItem, ListDeveloperProductsResponse,
};

impl RobloxApi {
    pub async fn create_developer_product_icon(
        &self,
        developer_product_id: AssetId,
        icon_file: PathBuf,
    ) -> RobloxApiResult<CreateDeveloperProductIconResponse> {
        let res = self
            .csrf_token_store
            .send_request(|| async {
                Ok(self
                    .client
                    .post(format!(
                        "https://apis.roblox.com/developer-products/v1/developer-products/{}/image",
                        developer_product_id
                    ))
                    .multipart(Form::new().part("imageFile", get_file_part(&icon_file).await?)))
            })
            .await;

        handle_as_json(res).await
    }

    pub async fn create_developer_product(
        &self,
        experience_id: AssetId,
        name: String,
        price: u32,
        description: String,
    ) -> RobloxApiResult<CreateDeveloperProductResponse> {
        let res = self
            .csrf_token_store
            .send_request(|| async {
                Ok(self
                    .client
                    .post(format!(
                "https://apis.roblox.com/developer-products/v1/universes/{}/developerproducts",
                experience_id
            ))
                    .header(header::CONTENT_LENGTH, 0)
                    .query(&[
                        ("name", &name),
                        ("priceInRobux", &price.to_string()),
                        ("description", &description),
                    ]))
            })
            .await;

        handle_as_json(res).await
    }

    pub async fn list_developer_products(
        &self,
        experience_id: AssetId,
        cursor: String,
    ) -> RobloxApiResult<ListDeveloperProductsResponse> {
        println!(
            "Fetching developer products for experience ID: {}, page: {}",
            experience_id, cursor
        );


        let res = self
            .csrf_token_store
            .send_request(|| async {
                Ok(self
                    .client
                    .get(format!("https://apis.roblox.com/developer-products/v2/developer-products/universes/{}/creator", &experience_id))
                    .query(&[
                        ("cursor", &cursor.to_string()),
                        ("limit", &"50".to_string()),
                    ]))
            })
            .await;

        let res_v2: RobloxApiResult<ListDeveloperProductsResponseV2> =  handle_as_json(res).await;
        match res_v2 {
            Ok(response) => {
                let developer_products: Vec<ListDeveloperProductResponseItem> = response
                    .developer_products_overview
                    .into_iter()
                    .map(|item| ListDeveloperProductResponseItem {
                        product_id: item.product_id,
                        developer_product_id: item.developer_product_id,
                        name: item.name,
                        description: item.description,
                        icon_image_asset_id: item.icon_image_asset_id,
                        price_in_robux: item.price_information.map_or(0, |p| p.default_price_in_robux.unwrap_or(0)),
                    })
                    .collect();
                let is_final_page = response.next_page_cursor.is_none() || developer_products.len() < 50; // Assuming 50 is the limit per page
                Ok(ListDeveloperProductsResponse {
                    developer_products,
                    next_page_cursor: response.next_page_cursor,
                    final_page: is_final_page, // Assuming 50 is the limit per page
                })
            },
            Err(e) => {
                println!("Error fetching developer products: {}", e);
                Err(e)
            }
        }
    }

    pub async fn get_all_developer_products(
        &self,
        experience_id: AssetId,
    ) -> RobloxApiResult<Vec<ListDeveloperProductResponseItem>> {
        println!(
            "Fetching all developer products for experience ID: {}",
            experience_id
        );
        let mut all_products = Vec::new();

        let mut cursor: String = "".to_string();
        loop {
            let res = self.list_developer_products(experience_id, cursor).await?;
            all_products.extend(res.developer_products);

            if res.final_page {
                break;
            }

            cursor = res.next_page_cursor.unwrap();
        }

        Ok(all_products)
    }

    pub async fn get_developer_product(
        &self,
        developer_product_id: AssetId,
    ) -> RobloxApiResult<GetDeveloperProductResponse> {
        let res = self
            .csrf_token_store
            .send_request(|| async {
                Ok(self.client.get(format!(
                    "https://apis.roblox.com/developer-products/v1/developer-products/{}",
                    developer_product_id
                )))
            })
            .await;

        handle_as_json(res).await
    }

    pub async fn update_developer_product(
        &self,
        experience_id: AssetId,
        product_id: AssetId,
        name: String,
        price: u32,
        description: String,
    ) -> RobloxApiResult<()> {
        let res = self.csrf_token_store.send_request(||async {
Ok(self
            .client
            .post(format!(
                "https://apis.roblox.com/developer-products/v1/universes/{}/developerproducts/{}/update",
                experience_id, product_id
            ))
            .json(&json!({
                "Name": name,
                "PriceInRobux": price,
                "Description": description,
            })))
        }).await;

        handle(res).await?;

        Ok(())
    }
}
