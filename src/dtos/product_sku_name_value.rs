use salvo::prelude::{Extractible, ToSchema};
use serde::{Deserialize, Serialize};
use validator::Validate;
#[derive(Deserialize, Debug, Validate, ToSchema, Default)]
pub struct ProductSkuNameValueAddRequest {
    pub product_id: i32,
    pub sku_name_id: i32,
    pub sku_value_id: i32
}

#[derive(Debug, Deserialize, Extractible, ToSchema, Default)]
#[salvo(extract(default_source(from = "body", parse = "json")))]
pub struct ProductSkuNameValueUpdateRequest {
    #[salvo(extract(source(from = "param")))]
    pub id: i32,
    pub product_id: i32,
    pub sku_name_id: i32,
    pub sku_value_id: i32
}

///sku name和value模型
#[derive(Debug, Serialize, ToSchema, Default)]
pub struct ProductSkuNameValueResponse {
    ///自增ID
    pub id: i32,
    ///产品ID
    pub product_id: i32,
    ///sku name id
    pub sku_name_id: i32,
    ///sku value id
    pub sku_value_id: i32
}

