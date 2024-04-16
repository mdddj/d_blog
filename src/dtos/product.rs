use salvo::prelude::{Extractible, ToSchema};
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::dtos::product_sku_value::ProductPriceAndStock;

#[derive(Deserialize, Debug, Validate, ToSchema, Default, Serialize)]
pub struct ProductSkuValueRequestParam {
    pub value: String,
}

#[derive(Deserialize, Debug, Validate, ToSchema, Default, Serialize)]
pub struct ProductSkuNameRequestParam {
    pub name: String,
    pub sku_list: Vec<ProductSkuValueRequestParam>,
}

#[derive(Deserialize, Debug, Validate, ToSchema, Default, Serialize)]
pub struct ProductAddRequest {
    pub name: String,
    pub product_category_id: i32,
    pub shop_user_id: String,
    pub sku_raw_list: Vec<ProductSkuNameRequestParam>,
    pub price: Vec<ProductPriceAndStock>
}

#[derive(Debug, Deserialize, Extractible, ToSchema, Default)]
#[salvo(extract(default_source(from = "body", parse = "json")))]
pub struct ProductUpdateRequest {
    #[salvo(extract(source(from = "param")))]
    pub id: i32,
    pub name: String,
    pub product_category_id: i32,
    pub sale_count: i32,
    pub shop_user_id: String,
    pub comment_count: i32,
}

#[derive(Debug, Serialize, ToSchema, Default, Clone)]
pub struct ProductResponse {
    pub id: i32,
    pub name: String,
    pub product_category_id: i32,
    pub sale_count: i32,
    pub shop_user_id: String,
    pub comment_count: i32,
}
