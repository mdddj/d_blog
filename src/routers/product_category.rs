use crate::{
    app_writer::{AppResult, AppWriter},
    dtos::product_category::*,
    services::product_category,
};
use salvo::Writer;
use salvo::{
    oapi::endpoint,
    oapi::extract::{JsonBody, PathParam},
    Request,
};

//Router::with_path("/api/product_category").get(get_product_category_all).post(post_add_product_category).push(Router::with_path("<id>").put(put_update_product_category).delete(delete_product_category))

///添加产品分类
#[endpoint(tags("产品分类"))]
pub async fn post_add_product_category(
    new_product_category: JsonBody<ProductCategoryAddRequest>,
) -> AppWriter<ProductCategoryResponse> {
    let result = product_category::add_product_category(new_product_category.0).await;
    AppWriter(result)
}

///修改产品分类
#[endpoint(tags("产品分类"))]
pub async fn put_update_product_category(
    req: &mut Request,
) -> AppResult<AppWriter<ProductCategoryResponse>> {
    let req: ProductCategoryUpdateRequest = req.extract().await?;
    let result = product_category::update_product_category(req).await;
    Ok(AppWriter(result))
}

///删除产品分类
#[endpoint(tags("产品分类"))]
pub async fn delete_product_category(id: PathParam<i32>) -> AppWriter<()> {
    let result = product_category::delete_product_category(id.0).await;
    AppWriter(result)
}

///获取所有产品分类
#[endpoint(tags("产品分类"))]
pub async fn get_product_category_all() -> AppWriter<Vec<ProductCategoryResponse>> {
    let result = product_category::product_category_find_all().await;
    AppWriter(result)
}
