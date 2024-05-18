use crate::{
    app_writer::{AppResult, AppWriter},
    dtos::user::{
        UserAddRequest, UserLoginRequest, UserLoginResponse, UserResponse, UserUpdateRequest,
    },
    services::user,
};
use salvo::{Depot, handler, Response, Writer};
use salvo::{
    oapi::endpoint,
    oapi::extract::{JsonBody, PathParam},
    Request,
};
use salvo::jwt_auth::{JwtAuthDepotExt, JwtAuthState};
use crate::middleware::jwt::JwtClaims;
use crate::services::user::get_user_info_by_token;

#[endpoint(tags("comm"))]
pub async fn post_login(req: JsonBody<UserLoginRequest>) -> AppWriter<UserLoginResponse> {
    let result: AppResult<UserLoginResponse> = user::login(req.0).await;
    AppWriter(result)
}

#[endpoint(tags("users"))]
pub async fn post_add_user(new_user: JsonBody<UserAddRequest>) -> AppWriter<UserResponse> {
    let result = user::add_user(new_user.0).await;
    AppWriter(result)
}

#[endpoint(tags("users"), parameters(
    ("id", description = "用户ID")
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

///获取当前登录用户信息
#[endpoint(tags("get_user_by_token"))]
pub async fn get_user_by_token_api(depot: &mut Depot) -> AppWriter<UserResponse> {
    let state = depot.jwt_auth_state();
    let r = match state {
        JwtAuthState::Authorized => {
            let data = depot.jwt_auth_data::<JwtClaims>().unwrap();
            get_user_info_by_token(&data.claims).await
        }
        JwtAuthState::Unauthorized => {
            Err(anyhow::anyhow!("Unauthorized!").into())
        }
        JwtAuthState::Forbidden => {
            Err(anyhow::anyhow!("Forbidden!").into())
        }
    };
    AppWriter(r)
}


#[handler]
pub async fn set_current_user_hook(req: &mut Request,depot: &mut Depot,res: &mut Response) {
    let method = req.method();
    let url = req.uri().path();
    println!("method: {:?}  uri: {:?}",method,url);
    let state = depot.jwt_auth_state();
    match state {
        JwtAuthState::Authorized => {
            let data = depot.jwt_auth_data::<JwtClaims>().unwrap();
            let user = get_user_info_by_token(&data.claims).await;
            match user {
                Ok(value) => {
                    depot.insert("user", value);
                }
                Err(e) => {
                }
            }

        }
        _ => {}
    };
}