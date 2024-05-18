use crate::entities::prelude::Users;
use crate::{
    app_writer::AppResult, db::DB, dtos::user::*, entities::*, middleware::jwt::get_token,
    utils::rand_utils,
};
use salvo::Listener;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use uuid::Uuid;
use crate::app_writer::AppWriter;
use crate::entities::users::{Column, Model};
use crate::middleware::jwt::{decode_token, JwtClaims};

pub async fn login(req: UserLoginRequest) -> AppResult<UserLoginResponse> {
    let db = DB
        .get()
        .ok_or(anyhow::anyhow!("Database connection failed."))?;
    let user = Users::find()
        .filter(users::Column::Username.eq(req.username))
        .one(db)
        .await?;
    if user.is_none() {
        return Err(anyhow::anyhow!("用户不存在").into());
    }
    let user = user.unwrap();
    if rand_utils::verify_password(req.password, user.password)
        .await
        .is_err()
    {
        return Err(anyhow::anyhow!("密码不正确").into());
    }
    let (token, exp) = get_token(user.username.clone(), user.id.clone())?;
    let res = UserLoginResponse {
        id: user.id,
        username: user.username,
        token,
        exp,
    };
    Ok(res)
}

pub async fn add_user(req: UserAddRequest) -> AppResult<UserResponse> {
    let db = DB
        .get()
        .ok_or(anyhow::anyhow!("Database connection failed."))?;
    let model = users::ActiveModel {
        id: Set(Uuid::new_v4().to_string()),
        username: Set(req.username.clone()),
        password: Set(rand_utils::hash_password(req.password).await?),
        avatar: Default::default(),
        last_login: Default::default(),
        nick_name: Default::default(),
    };
    let user = Users::insert(model).exec(db).await?;
    Ok(UserResponse {
        id: user.last_insert_id,
        username: req.username,
        avatar: None,
        nick_name: None,
    })
}

pub async fn update_user(req: UserUpdateRequest) -> AppResult<UserResponse> {
    let db = DB
        .get()
        .ok_or(anyhow::anyhow!("Database connection failed."))?;

    let user = Users::find_by_id(req.id).one(db).await?;
    if user.is_none() {
        return Err(anyhow::anyhow!("User does not exist.").into());
    }
    let mut user: users::ActiveModel = user.unwrap().into();

    user.username = Set(req.username.to_owned());
    user.password = Set(rand_utils::hash_password(req.password).await?);
    user.avatar = Set(req.avatar.to_owned());
    user.nick_name = Set(req.nick_name.to_owned());

    let user: users::Model = user.update(db).await?;

    Ok(UserResponse {
        id: user.id,
        username: user.username,
        avatar: user.avatar,
        nick_name: user.nick_name,
    })
}


pub async fn get_user_info_by_token(jwt_claims: &JwtClaims) -> AppResult<UserResponse> {
    let username = jwt_claims.username.to_owned();
    let db = DB
        .get()
        .ok_or(anyhow::anyhow!("Database connection failed."))?;
    let user = Users::find().filter(Column::Username.eq(username)).one(db).await?;
    return match user {
        None => {
            Err(anyhow::anyhow!("User info not found!").into())
        }
        Some(find_user) => {
            Ok(UserResponse {
                id: find_user.id,
                username: find_user.username,
                avatar: find_user.avatar,
                nick_name: find_user.nick_name,
            })
        }
    }
}

pub async fn delete_user(id: String) -> AppResult<()> {
    let db = DB
        .get()
        .ok_or(anyhow::anyhow!("Database connection failed."))?;
    Users::delete_by_id(id).exec(db).await?;
    Ok(())
}

pub async fn users() -> AppResult<Vec<UserResponse>> {
    let db = DB
        .get()
        .ok_or(anyhow::anyhow!("Database connection failed."))?;
    let users = Users::find().all(db).await?;
    let res = users
        .into_iter()
        .map(|user| UserResponse {
            id: user.id,
            username: user.username,
            avatar: user.avatar,
            nick_name: user.nick_name,
        })
        .collect::<Vec<_>>();
    Ok(res)
}

///创建初始管理员用户
pub async fn check_and_init_admin_user() {
    let all_users = users().await;

    match all_users {
        Ok(list) => {
            if list.is_empty() {
                let username = "admin";
                let password = "123456";
                let save_result = add_user(UserAddRequest {
                    username: username.to_owned(),
                    password: password.to_owned(),
                })
                    .await;
                match save_result {
                    Ok(u) => tracing::info!("创建管理员账号成功:{:?}", serde_json::to_string(&u)),
                    Err(_) => tracing::warn!("创建管理员账号失败"),
                }
            }
        }
        Err(_) => {
            tracing::warn!("获取用户列表失败");
        }
    }
}
