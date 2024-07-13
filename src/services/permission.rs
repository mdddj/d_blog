use sea_orm::{ActiveModelTrait, EntityTrait, PaginatorTrait};
use sea_orm::ActiveValue::Set;
use tracing::info;

use crate::{app_writer::AppResult, db::DB, dtos::permission::*, entities::*};
use crate::db::get_db;
use crate::dtos::permission::*;
use crate::dtos::user_role::UserRoleAddRequest;
use crate::entities::prelude::Permission;
use crate::services::role::init_admin_role;
use crate::services::user::get_admin_user;
use crate::services::user_role::add_user_role;

pub async fn add_permission(req: PermissionAddRequest) -> AppResult<PermissionResponse> {
    let db = get_db();
    let model = permission::ActiveModel::from_permission_add_request(&req);
    let result = Permission::insert(model).exec(db).await?;
    Ok(PermissionResponse {
        id: result.last_insert_id,
        name: req.name,
        description: req.description,
        create_time: req.create_time,
        permission_url: req.permission_url,
        r#type: req.r#type,
        method: req.method,
        group: req.group,
    })
}

///批量插入权限列表
async fn permission_batch_add(reqs: Vec<PermissionAddRequest>) {
    let db = get_db();
    let models: Vec<permission::ActiveModel> = reqs.iter().map(|x| permission::ActiveModel::from_permission_add_request(x)).collect();
    let _ = Permission::insert_many(models).exec(db).await;
}

pub async fn update_permission(req: PermissionUpdateRequest) -> AppResult<PermissionResponse> {
    let db = get_db();

    let find = Permission::find_by_id(req.id).one(db).await?;
    if find.is_none() {
        return Err(anyhow::anyhow!("Permission does not exist.").into());
    }
    let mut model: permission::ActiveModel = find.unwrap().into();

    model.name = Set(req.name);
    model.description = Set(req.description);
    model.create_time = Set(req.create_time);
    model.permission_url = Set(req.permission_url);
    model.r#type = Set(req.r#type);
    model.method = Set(req.method);
    model.group = Set(req.group);

    let result: permission::Model = model.update(db).await?;

    Ok(result.into())
}

pub async fn delete_permission(id: i32) -> AppResult<()> {
    let db = get_db();
    Permission::delete_by_id(id).exec(db).await?;
    Ok(())
}

pub async fn permission_find_all() -> AppResult<Vec<PermissionResponse>> {
    let db = get_db();
    let permission = Permission::find().all(db).await?;
    let res = permission
        .into_iter()
        .map(|r| r.into())
        .collect::<Vec<_>>();
    Ok(res)
}

/// 初始化权限列表,只有在为空的情况下执行
pub async fn init_permission_init_all() {
    let count = Permission::find().count(get_db()).await.unwrap();
    if count != 0 { return; }

    let exception = vec![
        PermissionAddRequest::from_simple_string_url_type("添加异常:POST:/api/exception:异常".to_string())
    ];

    let role = vec![
        PermissionAddRequest::from_simple_string_url_type("添加角色:POST:/api/user_role:角色管理".to_string()),
        PermissionAddRequest::from_regex("删除角色".to_string(), r"^/api/user_role/\d+$".to_string(), "DELETE".to_string(), "角色管理".to_string()),
        PermissionAddRequest::from_regex("获取角色".to_string(), r"^/api/user_role/\d+$".to_string(), "GET".to_string(), "角色管理".to_string()),
    ];

    let permission = vec![
        PermissionAddRequest::from_simple_string_url_type("获取角色权限中间列表:GET:/api/permission_role:权限管理".to_string()),
        PermissionAddRequest::from_simple_string_url_type("获取权限列表:GET:/api/permission:权限管理".to_string()),
        PermissionAddRequest::from_simple_string_url_type("添加修改权限:POST:/api/permission_role:权限管理".to_string()),
        PermissionAddRequest::from_regex("获取权限详情".to_string(), r"^/api/permission_role/\d+$".to_string(), "GET".to_string(), "权限管理".to_string()),
    ];

    let users = vec![
        PermissionAddRequest::from_simple_string_url_type("获取用户:GET:/api/users:用户管理".to_string()),
        PermissionAddRequest::from_simple_string_url_type("添加用户:POST:/api/users:用户管理".to_string()),
        PermissionAddRequest::from_regex("修改用户".to_string(), r"^/api/users/\d+$".to_string(), "PUT".to_string(), "用户管理".to_string()),
        PermissionAddRequest::from_regex("删除用户".to_string(), r"^/api/users/\d+$".to_string(), "DELETE".to_string(), "用户管理".to_string()),
    ];

    let mut all = Vec::new();
    all.extend(exception);
    all.extend(role);
    all.extend(permission);
    all.extend(users);

    permission_batch_add(all).await;
    let id = init_admin_role().await;
    if let Some(r_id) = id {
        //设置用户超级权限
        if let Some(admin_user) = get_admin_user().await {
            let r = add_user_role(UserRoleAddRequest { user_id: admin_user.id, role_id: r_id }).await;
            match r {
                Ok(_) => {
                    info!("设置{}超级管理员成功",admin_user.username)
                }
                Err(_) => {}
            }
        }
    }
}
