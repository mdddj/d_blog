use salvo::prelude::{Extractible, ToSchema};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Debug, Validate, ToSchema, Default)]
pub struct PermissionRoleAddRequest {
    pub permission_id: i32,
    pub role_id: i32,
    pub notes: Option<String>
}

#[derive(Debug, Deserialize, Extractible, ToSchema, Default)]
#[salvo(extract(default_source(from = "body", parse = "json")))]
pub struct PermissionRoleUpdateRequest {
    #[salvo(extract(source(from = "param")))]
    pub id: i32,
    pub permission_id: i32,
    pub role_id: i32,
    pub notes: Option<String>
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct PermissionRoleResponse {
    pub id: i32,
    pub permission_id: i32,
    pub role_id: i32,
    pub notes: Option<String>
}



///添加角色的请求参数
#[derive(Deserialize, Debug, Validate, ToSchema, Default)]
pub struct NewRolePermissionParam {
    ///角色名称
    pub name: String,
    ///权限列表
    pub permission_ids: Vec<i32>,
    ///备注
    pub description: Option<String>

}
