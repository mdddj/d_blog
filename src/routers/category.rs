use crate::{
    app_writer::{AppResult, AppWriter},
    dtos::category::*,
    services::category,
};
use salvo::Writer;
use salvo::{
    oapi::endpoint,
    oapi::extract::{JsonBody, PathParam},
    Request,
};

//Router::with_path("/api/category").get(get_category_all).post(post_add_category).push(Router::with_path("<id>").put(put_update_category).delete(delete_category))

#[endpoint(tags("category"))]
pub async fn post_add_category(
    new_category: JsonBody<CategoryAddRequest>,
) -> AppWriter<CategoryResponse> {
    let result = category::add_category(new_category.0).await;
    AppWriter(result)
}

#[endpoint(tags("category"),parameters(
    ("id",description="分类ID")
))]
pub async fn put_update_category(req: &mut Request) -> AppResult<AppWriter<CategoryResponse>> {
    let req: CategoryUpdateRequest = req.extract().await?;
    let result = category::update_category(req).await;
    Ok(AppWriter(result))
}

#[endpoint(tags("category"))]
pub async fn delete_category(id: PathParam<i32>) -> AppWriter<()> {
    let result = category::delete_category(id.0).await;
    AppWriter(result)
}

#[endpoint(tags("category"))]
pub async fn get_category_all() -> AppWriter<Vec<CategoryResponse>> {
    let result = category::category_find_all().await;
    AppWriter(result)
}
