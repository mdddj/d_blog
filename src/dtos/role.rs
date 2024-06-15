use salvo::prelude::{Extractible, ToSchema};
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::entities::role;

#[derive(Deserialize, Debug, Validate, ToSchema, Default)]
pub struct RoleAddRequest {
    pub name: String,
    pub description: Option<String>,
    //是否能删除
    pub can_delete: Option<bool>
}

#[derive(Debug, Deserialize, Extractible, ToSchema, Default)]
#[salvo(extract(default_source(from = "body", parse = "json")))]
pub struct RoleUpdateRequest {
    #[salvo(extract(source(from = "param")))]
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub can_delete: Option<bool>
}

#[derive(Debug, Serialize, ToSchema, Default,Clone)]
pub struct RoleResponse {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub can_delete: Option<bool>
}


impl Into<RoleResponse> for role::Model {
    fn into(self) -> RoleResponse {
        return RoleResponse {
            id: self.id,
            name: self.name.clone(),
            description: self.description.clone(),
            can_delete: self.can_delete,
        }
    }
}