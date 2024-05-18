use salvo::prelude::{Extractible, ToSchema};
use serde::{Deserialize, Serialize};
use validator::Validate;
#[derive(Deserialize, Debug, Validate, ToSchema, Default)]
pub struct PermissionAddRequest {
    pub name: String,
    pub description: Option<String>,
    pub create_time: Option<String>,
    pub permission_url: Option<String>,
    pub r#type: Option<String>,
    pub method: Option<String>,
    pub group: Option<String>
}

#[derive(Debug, Deserialize, Extractible, ToSchema, Default)]
#[salvo(extract(default_source(from = "body", parse = "json")))]
pub struct PermissionUpdateRequest {
    #[salvo(extract(source(from = "param")))]
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub create_time: Option<String>,
    pub permission_url: Option<String>,
    pub r#type: Option<String>,
    pub method: Option<String>,
    pub group: Option<String>
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct PermissionResponse {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub create_time: Option<String>,
    pub permission_url: Option<String>,
    pub r#type: Option<String>,
    pub method: Option<String>,
    pub group: Option<String>
}

