use crate::{
    app_writer::ErrorResponseBuilder,
    app_writer::{AppResult, AppWriter},
    dtos::user::{
        UserAddRequest, UserLoginRequest, UserLoginResponse, UserResponse, UserUpdateRequest,
    },
    services::user,
};
use salvo::Writer;
use salvo::{
    http::cookie::Cookie,
    oapi::endpoint,
    oapi::extract::{JsonBody, PathParam},
    Request, Response,
};

#[endpoint(tags("comm"))]
pub async fn post_login(req: JsonBody<UserLoginRequest>, res: &mut Response) {
    let result: AppResult<UserLoginResponse> = user::login(req.0).await;
    match result {
        Ok(data) => {
            let jwt_token = data.token.clone();
            let cookie = Cookie::build(("jwt_token", jwt_token))
                .path("/")
                .http_only(true)
                .build();
            res.add_cookie(cookie);
        }
        Err(e) => ErrorResponseBuilder::with_err(e).into_response(res),
    }
}

#[endpoint(tags("users"))]
pub async fn post_add_user(new_user: JsonBody<UserAddRequest>) -> AppWriter<UserResponse> {
    let result = user::add_user(new_user.0).await;
    AppWriter(result)
}

#[endpoint(tags("users"),parameters(
    ("id",description="用户ID")
))]
pub async fn put_update_user(req: &mut Request) -> AppResult<AppWriter<UserResponse>> {
    let req: UserUpdateRequest = req.extract().await?;
    let result = user::update_user(req).await;
    Ok(AppWriter(result))
}

#[endpoint(tags("users"))]
pub async fn delete_user(id: PathParam<String>) -> AppWriter<()> {
    let result = user::delete_user(id.0).await;
    AppWriter(result)
}

#[endpoint(tags("users"))]
pub async fn get_users() -> AppWriter<Vec<UserResponse>> {
    let result = user::users().await;
    AppWriter(result)
}
