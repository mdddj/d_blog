use salvo::prelude::{Extractible, ToSchema};
use serde::{Deserialize, Serialize};
use validator::Validate;#[derive(Deserialize, Debug, Validate, ToSchema, Default)]
pub struct RoleAddRequest {
    pub name: String,
    pub description: Option<String>
}

#[derive(Debug, Deserialize, Extractible, ToSchema, Default)]
#[salvo(extract(default_source(from = "body", parse = "json")))]
pub struct RoleUpdateRequest {
    #[salvo(extract(source(from = "param")))]
    pub id: i32,
    pub name: String,
    pub description: Option<String>
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct RoleResponse {
    pub id: i32,
    pub name: String,
    pub description: Option<String>
}

