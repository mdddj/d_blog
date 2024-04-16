use salvo::prelude::{Extractible, ToSchema};
use serde::{Deserialize, Serialize};
use validator::Validate;
#[derive(Deserialize, Debug, Validate, ToSchema, Default)]
pub struct ProductSkuNameAddRequest {
    pub name: String,
    pub product_category_id: i32
}

#[derive(Debug, Deserialize, Extractible, ToSchema, Default)]
#[salvo(extract(default_source(from = "body", parse = "json")))]
pub struct ProductSkuNameUpdateRequest {
    #[salvo(extract(source(from = "param")))]
    pub id: i32,
    pub name: String,
    pub product_category_id: i32
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct ProductSkuNameResponse {
    pub id: i32,
    pub name: String,
    pub product_category_id: i32
}

