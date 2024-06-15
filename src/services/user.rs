use crate::entities::prelude::Users;
use crate::{
    app_writer::AppResult, db::DB, dtos::user::*, entities::*, middleware::jwt::get_token,
    utils::rand_utils,
};
use salvo::Listener;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter, Set};
use uuid::Uuid;
use crate::app_writer::AppWriter;
use crate::db::get_db;
use crate::entities::users::{Column, Model};
use crate::middleware::jwt::{decode_token, JwtClaims};
use crate::services::user_role::find_roles_by_user_id;

pub async fn login(req: UserLoginRequest) -> AppResult<UserLoginResponse> {
    let db = get_db();
    let user = Users::find()
        .filter(Column::Username.eq(req.username))
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
    let db = get_db();
    let model = users::ActiveModel {
        id: Set(Uuid::new_v4().to_string()),
        username: Set(req.username.clone()),
        password: Set(rand_utils::hash_password(req.password).await?),
        avatar: Set(req.avatar.clone()),
        last_login: Default::default(),
        nick_name: Set(req.nick_name.clone()),
    };
    let user = Users::insert(model).exec(db).await?;
    Ok(UserResponse {
        id: user.last_insert_id,
        username: req.username,
        avatar: req.avatar,
        nick_name: req.nick_name,
        permission_information: None,
    })
}

pub async fn update_user(req: UserUpdateRequest) -> AppResult<UserResponse> {
    let db = get_db();

    let user = Users::find_by_id(req.id).one(db).await?;
    if user.is_none() {
        return Err(anyhow::anyhow!("User does not exist.").into());
    }
    let mut user: users::ActiveModel = user.unwrap().into();

    user.username = Set(req.username.to_owned());
    user.password = Set(rand_utils::hash_password(req.password).await?);
    user.avatar = Set(req.avatar.to_owned());
    user.nick_name = Set(req.nick_name.to_owned());
    let user: Model = user.update(db).await?;
    let role_infos = find_roles_by_user_id(&(user.id)).await;
    Ok(UserResponse {
        id: user.id,
        username: user.username,
        avatar: user.avatar,
        nick_name: user.nick_name,
        permission_information: Some(role_infos),
    })
}


pub async fn get_user_info_by_token(jwt_claims: &JwtClaims) -> AppResult<UserResponse> {
    let username = jwt_claims.username.to_owned();
    let db = get_db();
    let user = Users::find().filter(Column::Username.eq(username)).one(db).await?;
    return match user {
        None => {
            Err(anyhow::anyhow!("User info not found!").into())
        }
        Some(find_user) => {
            let role_info = find_roles_by_user_id(&(find_user.id)).await;
            Ok(UserResponse {
                id: find_user.id,
                username: find_user.username,
                avatar: find_user.avatar,
                nick_name: find_user.nick_name,
                permission_information: Some(role_info),
            })
        }
    };
}

pub async fn delete_user(id: String) -> AppResult<()> {
    let db = get_db();
    Users::delete_by_id(id).exec(db).await?;
    Ok(())
}

pub async fn users() -> AppResult<Vec<UserResponse>> {
    let db = get_db();
    let users = Users::find().all(db).await?;
    let res = users
        .into_iter()
        .map(|user| user.into())
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
                    avatar: Some("https://t.alcy.cc/tx".to_owned()),
                    nick_name: Some("梁典典".to_owned()),
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


///获取管理员用户
pub async fn get_admin_user() -> Option<UserResponse> {
    let db = get_db();
   let r =  Users::find().filter(Column::Username.eq("admin")).one(db).await;
    match r {
        Ok(v) => {
            if let Some(u) = v {
                return Some(u.into())
            }
        }
        Err(_) => {}
    }

    None
}