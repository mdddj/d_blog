use crate::dtos::category::CategoryResponse;
use salvo::prelude::{Extractible, ToSchema};
use serde::{Deserialize, Serialize};
use validator::Validate;

///添加博客模型
#[derive(Deserialize, Debug, Validate, ToSchema, Default)]
pub struct PostAddRequest {
    ///标题
    #[salvo(schema(example = "test"), parameter(description = "标题"))]
    pub title: String,
    ///正文内容
    #[salvo(schema(example = "test content"))]
    pub content: String,
    ///分类ID
    #[salvo(schema(example = 1))]
    pub category_id: i32,
}

///修改博客模型
#[derive(Debug, Deserialize, Extractible, ToSchema, Default)]
#[salvo(extract(default_source(from = "body", parse = "json")))]
pub struct PostUpdateRequest {
    ///主键ID
    #[salvo(extract(source(from = "param")))]
    pub id: i32,
    ///标题
    pub title: String,
    ///正文内容
    pub content: String,
    ///分类ID
    pub category_id: i32,
}

///返回博客模型
#[derive(Debug, Serialize, ToSchema, Default)]
pub struct PostResponse {
    ///主键ID
    pub id: i32,
    ///标题
    pub title: String,
    ///正文内容
    pub content: String,
    ///分类ID
    pub category_id: i32,
    ///分类模型
    pub category: Option<CategoryResponse>,
}
