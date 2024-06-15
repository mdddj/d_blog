use crate::{
    app_writer::{AppResult, AppWriter},
    dtos::user_role::*,
    services::user_role,
};
use salvo::Writer;
use salvo::{
    oapi::endpoint,
    oapi::extract::{JsonBody, PathParam},
    Request,
};

//Router::with_path("/api/user_role").get(get_user_role_all).post(post_add_user_role).push(Router::with_path("<id>").put(put_update_user_role).delete(delete_user_role))

///添加角色权限
#[endpoint(tags("user_role"))]
pub async fn post_add_user_role(new_user_role: JsonBody<UserRoleAddRequest>) -> AppWriter<UserRoleResponse> {
    let result = user_role::add_user_role(new_user_role.0).await;
    AppWriter(result)
}

///删除角色权限
#[endpoint(tags("user_role"))]
pub async fn delete_user_role(id: PathParam<i32>) -> AppWriter<()> {
    let result = user_role::delete_user_role(id.0).await;
    AppWriter(result)
}

///获取用户权限列表
#[endpoint(tags("user_roles_get"))]
pub async fn get_user_role_by_user_id(id: PathParam<String>) {
    user_role::find_roles_by_user_id(&(id.0)).await;
}

