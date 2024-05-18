use salvo::prelude::{Extractible, ToSchema};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Debug, Validate, ToSchema, Default)]
pub struct UserAddRequest {
    ///用户名
    #[validate(length(min = 5, message = "username length must be greater than 5"))]
    pub username: String,
    ///密码
    pub password: String,
}

#[derive(Debug, Deserialize, ToSchema, Default)]
pub struct UserLoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Extractible, ToSchema, Default)]
#[salvo(extract(default_source(from = "body", parse = "json")))]
pub struct UserUpdateRequest {
    #[salvo(extract(source(from = "param")))]
    pub id: String,
    pub username: String,
    pub password: String,
    ///用户名
    pub nick_name: Option<String>,
    ///头像
    pub avatar: Option<String>
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct UserResponse {
    pub id: String,
    pub username: String,
    pub avatar: Option<String>,
    pub nick_name: Option<String>
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct UserLoginResponse {
    pub id: String,
    pub username: String,
    pub token: String,
    pub exp: i64,
}
