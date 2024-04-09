use crate::{
    app_writer::{AppResult, AppWriter},
    dtos::post::*,
    services::post,
};
use salvo::Writer;
use salvo::{
    oapi::endpoint,
    oapi::extract::{JsonBody, PathParam},
    Request,
};

//Router::with_path("/api/post").get(get_post_all).post(post_add_post).push(Router::with_path("<id>").put(put_update_post).delete(delete_post))

/// 添加博客
#[endpoint(tags("博客"))]
pub async fn post_add_post(new_post: JsonBody<PostAddRequest>) -> AppWriter<PostResponse> {
    let result = post::add_post(new_post.0).await;
    AppWriter(result)
}

#[endpoint(tags("博客"),parameters(
    ("id",description="博客ID")
),)]
pub async fn put_update_post(req: &mut Request) -> AppResult<AppWriter<PostResponse>> {
    let req: PostUpdateRequest = req.extract().await?;
    let result = post::update_post(req).await;
    Ok(AppWriter(result))
}

#[endpoint(tags("博客"))]
pub async fn delete_post(id: PathParam<i32>) -> AppWriter<()> {
    let result = post::delete_post(id.0).await;
    AppWriter(result)
}

/// 获取所有博客
///
/// 一次加载全部博客
#[endpoint(tags("博客"))]
pub async fn get_post_all() -> AppWriter<Vec<PostResponse>> {
    let result = post::post_find_all().await;
    AppWriter(result)
}

#[endpoint(tags("博客"))]
pub async fn get_post_by_id(id: PathParam<i32>) -> AppWriter<PostResponse> {
    let r = post::find_blog_by_id(id.0).await;
    AppWriter(r)
}
