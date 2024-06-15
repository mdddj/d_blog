use salvo::prelude::{Extractible, ToSchema};
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::dtos::permission::PermissionResponse;
use crate::dtos::role::RoleResponse;

#[derive(Deserialize, Debug, Validate, ToSchema, Default)]
pub struct UserRoleAddRequest {
    pub user_id: String,
    pub role_id: i32
}

#[derive(Debug, Deserialize, Extractible, ToSchema, Default)]
#[salvo(extract(default_source(from = "body", parse = "json")))]
pub struct UserRoleUpdateRequest {
    #[salvo(extract(source(from = "param")))]
    pub id: i32,
    pub user_id: String,
    pub role_id: i32
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct UserRoleResponse {
    pub id: i32,
    pub user_id: String,
    pub role_id: i32
}

//用户权限信息
#[derive(Debug,Serialize,ToSchema,Default,Clone)]
pub struct UserRoleDetails {
    pub roles: Vec<RoleResponse>,
    pub permissions: Vec<PermissionResponse>
}