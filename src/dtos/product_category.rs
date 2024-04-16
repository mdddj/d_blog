use salvo::prelude::{Extractible, ToSchema};
use serde::{Deserialize, Serialize};
use validator::Validate;
#[derive(Deserialize, Debug, Validate, ToSchema, Default)]
pub struct ProductCategoryAddRequest {
    pub name: String,
    pub parent_product_category_id: Option<i32>,
    pub shop_user_id: String,
}

#[derive(Debug, Deserialize, Extractible, ToSchema, Default)]
#[salvo(extract(default_source(from = "body", parse = "json")))]
pub struct ProductCategoryUpdateRequest {
    #[salvo(extract(source(from = "param")))]
    pub id: i32,
    pub name: String,
    pub parent_product_category_id: Option<i32>,
    pub shop_user_id: String,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct ProductCategoryResponse {
    pub id: i32,
    pub name: String,
    pub parent_product_category_id: Option<i32>,
    pub shop_user_id: String,
}
