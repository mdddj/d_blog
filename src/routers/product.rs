use crate::{
    app_writer::{AppResult, AppWriter},
    dtos::product::*,
    services::product,
};
use salvo::Writer;
use salvo::{
    oapi::endpoint,
    oapi::extract::{JsonBody, PathParam},
    Request,
};

//Router::with_path("/api/product").get(get_product_all).post(post_add_product).push(Router::with_path("<id>").put(put_update_product).delete(delete_product))

///添加产品
#[endpoint(tags("产品接口"))]
pub async fn post_add_product(
    new_product: JsonBody<ProductAddRequest>,
) -> AppWriter<ProductResponse> {
    let result = product::add_product(new_product.0).await;
    AppWriter(result)
}

///修改产品
#[endpoint(tags("产品接口"))]
pub async fn put_update_product(req: &mut Request) -> AppResult<AppWriter<ProductResponse>> {
    let req: ProductUpdateRequest = req.extract().await?;
    let result = product::update_product(req).await;
    Ok(AppWriter(result))
}

///删除产品
#[endpoint(tags("产品接口"))]
pub async fn delete_product(id: PathParam<i32>) -> AppWriter<()> {
    let result = product::delete_product(id.0).await;
    AppWriter(result)
}
///获取所有产品
#[endpoint(tags("产品接口"))]
pub async fn get_product_all() -> AppWriter<Vec<ProductResponse>> {
    let result = product::product_find_all().await;
    AppWriter(result)
}
