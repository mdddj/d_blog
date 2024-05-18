use crate::{
    app_writer::{AppResult, AppWriter},
    dtos::permission_role::*,
    services::permission_role,
};
use salvo::Writer;
use salvo::{
    oapi::endpoint,
    oapi::extract::{JsonBody, PathParam},
    Request,
};
use crate::dtos::role::RoleResponse;
use crate::entities::permission;
use crate::services::role::role_find_all;

//Router::with_path("/api/permission_role").get(get_permission_role_all).post(post_add_permission_role).push(Router::with_path("<id>").put(put_update_permission_role).delete(delete_permission_role))

///添加角色
#[endpoint(tags("permission_role"))]
pub async fn post_add_permission_role(new_permission_role: JsonBody<NewRolePermissionParam>) -> AppWriter<i32> {
    let result = permission_role::add_new_role(new_permission_role.0).await;
    AppWriter(result)
}

///id获取role详情
#[endpoint(tags("permission_role"))]
pub async fn get_role_by_id(id: PathParam<i32>) -> AppWriter<Vec<permission::Model>> {
    println!("params:{:?}",id);
    let result = permission_role::find_role_permission_list(id.0).await;
    AppWriter(result)
}

///获取所有的角色
#[endpoint(tags("permission_role"))]
pub async fn get_all_roles() -> AppWriter<Vec<RoleResponse>>{
    let result = role_find_all().await;
    AppWriter(result)
}
