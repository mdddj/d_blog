use salvo::prelude::{Extractible, ToSchema};
use serde::{Deserialize, Serialize};
use tracing::info;
use validator::Validate;
use crate::config::CFG;
use crate::dtos::user_role::UserRoleDetails;
use crate::entities::users;

#[derive(Deserialize, Debug, Validate, ToSchema, Default)]
pub struct UserAddRequest {
    ///用户名
    #[validate(length(min = 5, message = "username length must be greater than 5"))]
    pub username: String,
    ///密码
    pub password: String,
    ///头像
    pub avatar: Option<String>,
    ///用户名
    pub nick_name: Option<String>,
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
    pub avatar: Option<String>,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct UserResponse {
    pub id: String,
    pub username: String,
    pub avatar: Option<String>,
    pub nick_name: Option<String>,
    pub permission_information: Option<UserRoleDetails>,
}


impl UserResponse {
    pub fn has_role(&self, method: &str, url: &str) -> bool {


        fn trim_trailing_slash(url: &str) -> &str {
            if url.ends_with('/') {
                &url[..url.len() - 1]
            } else {
                url
            }
        }

        //是否忽略的
        let is_ig = CFG.auth.is_match(url, method);
        if is_ig {
            return true;
        }

        //判断url匹配
        let target = format!("{}:{}", method, trim_trailing_slash(url));
        if let Some(per_list) = self.permission_information.clone() {
            let atts: Vec<String> = per_list.permissions.iter().filter_map(|x| x.try_get_url()).collect();
            if atts.contains(&target) {
                return true;
            }
        }
        // ToDo 匹配正则


        return false;
    }

}

impl Into<UserResponse> for users::Model {
    fn into(self) -> UserResponse {
        UserResponse {
            id: self.id,
            username: self.username,
            avatar: self.avatar,
            nick_name: self.nick_name,
            permission_information: None,
        }
    }
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct UserLoginResponse {
    pub id: String,
    pub username: String,
    pub token: String,
    pub exp: i64,
}
