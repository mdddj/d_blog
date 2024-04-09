use crate::dtos::category::CategoryResponse;
use salvo::prelude::{Extractible, ToSchema};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Debug, Validate, ToSchema, Default)]
pub struct PostAddRequest {
    #[salvo(schema(example = "test"))]
    pub title: String,
    #[salvo(schema(example = "test content"))]
    pub content: String,
    #[salvo(schema(example = 1))]
    pub category_id: i32,
}

#[derive(Debug, Deserialize, Extractible, ToSchema, Default)]
#[salvo(extract(default_source(from = "body", parse = "json")))]
pub struct PostUpdateRequest {
    #[salvo(extract(source(from = "param")))]
    pub id: i32,
    pub title: String,
    pub content: String,
    pub category_id: i32,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct PostResponse {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub category_id: i32,
    pub category: Option<CategoryResponse>,
}
