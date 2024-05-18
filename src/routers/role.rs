use crate::{
    app_writer::{AppResult, AppWriter},
    dtos::role::*,
    services::role,
};
use salvo::Writer;
use salvo::{
    oapi::endpoint,
    oapi::extract::{JsonBody, PathParam},
    Request,
};

//Router::with_path("/api/role").get(get_role_all).post(post_add_role).push(Router::with_path("<id>").put(put_update_role).delete(delete_role))

#[endpoint(tags("role"))]
pub async fn post_add_role(new_role: JsonBody<RoleAddRequest>) -> AppWriter<RoleResponse> {
    let result = role::add_role(new_role.0).await;
    AppWriter(result)
}

#[endpoint(tags("role"))]
pub async fn put_update_role(req: &mut Request) -> AppResult<AppWriter<RoleResponse>> {
    let req: RoleUpdateRequest = req.extract().await?;
    let result = role::update_role(req).await;
    Ok(AppWriter(result))
}

#[endpoint(tags("role"))]
pub async fn delete_role(id: PathParam<i32>) -> AppWriter<()> {
    let result = role::delete_role(id.0).await;
    AppWriter(result)
}

#[endpoint(tags("role"))]
pub async fn get_role_all() -> AppWriter<Vec<RoleResponse>> {
    let result = role::role_find_all().await;
    AppWriter(result)
}
