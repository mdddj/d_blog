use crate::{
    app_writer::{AppResult, AppWriter},
    dtos::permission::*,
    services::permission,
};
use salvo::{Depot, Writer};
use salvo::{
    oapi::endpoint,
    oapi::extract::{JsonBody, PathParam},
    Request,
};

//Router::with_path("/api/permission").get(get_permission_all).post(post_add_permission).push(Router::with_path("<id>").put(put_update_permission).delete(delete_permission))

#[endpoint(tags("permission"))]
pub async fn post_add_permission(new_permission: JsonBody<PermissionAddRequest>) -> AppWriter<PermissionResponse> {
    let result = permission::add_permission(new_permission.0).await;
    AppWriter(result)
}

#[endpoint(tags("permission"))]
pub async fn put_update_permission(req: &mut Request) -> AppResult<AppWriter<PermissionResponse>> {
    let req: PermissionUpdateRequest = req.extract().await?;
    let result = permission::update_permission(req).await;
    Ok(AppWriter(result))
}

#[endpoint(tags("permission"))]
pub async fn delete_permission(id: PathParam<i32>) -> AppWriter<()> {
    let result = permission::delete_permission(id.0).await;
    AppWriter(result)
}

#[endpoint(tags("permission"))]
pub async fn get_permission_all() -> AppWriter<Vec<PermissionResponse>> {
    let result = permission::permission_find_all().await;
    AppWriter(result)
}
