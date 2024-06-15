use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, LoaderTrait, NotSet, QueryFilter, Set};

use crate::{
    app_writer::AppResult
    ,
    dtos::user_role::*,
    entities::*,
};
use crate::db::get_db;
use crate::dtos::permission::PermissionResponse;
use crate::dtos::role::RoleResponse;
use crate::dtos::user_role::*;
use crate::entities::prelude::{Role, UserRole};
use crate::entities::user_role::{Column, Model};

pub async fn add_user_role(req: UserRoleAddRequest) -> AppResult<UserRoleResponse> {
    let db = get_db();
    let find = UserRole::find().filter(Column::UserId.eq(req.user_id.clone()).and(Column::RoleId.eq(req.role_id))).one(db).await?;
    if find.is_some() {
        return Err(anyhow::anyhow!("Repeat the addition").into());
    }
    let model = user_role::ActiveModel {
        id: NotSet,
        user_id: Set(req.user_id.clone()),
        role_id: Set(req.role_id.clone()),
    };
    let result = UserRole::insert(model).exec(db).await?;
    Ok(UserRoleResponse {
        id: result.last_insert_id,
        user_id: req.user_id,
        role_id: req.role_id,
    })
}

pub async fn update_user_role(req: UserRoleUpdateRequest) -> AppResult<UserRoleResponse> {
    let db = get_db();
    let find = UserRole::find_by_id(req.id).one(db).await?;
    if find.is_none() {
        return Err(anyhow::anyhow!("UserRole does not exist.").into());
    }
    let mut model: user_role::ActiveModel = find.unwrap().into();
    model.user_id = Set(req.user_id);
    model.role_id = Set(req.role_id);
    let result: Model = model.update(db).await?;
    Ok(UserRoleResponse {
        id: result.id,
        user_id: result.user_id,
        role_id: result.role_id,

    })
}

pub async fn delete_user_role(id: i32) -> AppResult<()> {
    let db = get_db();
    UserRole::delete_by_id(id).exec(db).await?;
    Ok(())
}

pub async fn user_role_find_all() -> AppResult<Vec<UserRoleResponse>> {
    let db = get_db();
    let user_role = UserRole::find().all(db).await?;
    let res = user_role
        .into_iter()
        .map(|r| UserRoleResponse {
            id: r.id,
            user_id: r.user_id,
            role_id: r.role_id,

        })
        .collect::<Vec<_>>();
    Ok(res)
}

///查找用户的角色和权限
pub async fn find_roles_by_user_id(user_id: &str) -> UserRoleDetails {
    let db = get_db();
    let r = UserRole::find()
        .filter(Column::UserId.eq(user_id))
        .find_with_related(Role).all(db).await.expect("查询失败");
    let rrs: Vec<RoleResponse> = r.into_iter().flat_map(|(a, b)| { b }).map(Into::into).collect();
    let ids: Vec<i32> = rrs.iter().map(|x| x.id).collect();
    let all_permissions = permission_role::Entity::find().filter(
        permission_role::Column::RoleId.is_in(ids)
    )
        .find_also_related(permission::Entity)
        .all(db).await.expect("查询权限失败");
    let alls: Vec<PermissionResponse> = all_permissions.into_iter().filter_map(|(a, b)| b.map(|m|m.into())).collect();
    UserRoleDetails {
        roles: rrs,
        permissions: alls,
    }
}