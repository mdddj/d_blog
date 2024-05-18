use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, LoaderTrait, ModelTrait, NotSet, QueryFilter, Set};

use crate::{
    app_writer::AppResult,
    db::DB,
    dtos::permission_role::*,
    entities::*,
};
use crate::dtos::permission_role::*;
use crate::dtos::role::RoleAddRequest;
use crate::entities::prelude::PermissionRole;
use crate::services::role::add_role;

pub async fn add_permission_role(req: PermissionRoleAddRequest) -> AppResult<PermissionRoleResponse> {
    let db = DB
        .get()
        .ok_or(anyhow::anyhow!("Database connection failed."))?;
    let model = permission_role::ActiveModel {
        id: NotSet,
        permission_id: Set(req.permission_id.clone()),
        role_id: Set(req.role_id.clone()),
        notes: Set(req.notes.clone()),
    };
    let result = PermissionRole::insert(model).exec(db).await?;
    Ok(PermissionRoleResponse {
        id: result.last_insert_id,
        permission_id: req.permission_id,
        role_id: req.role_id,
        notes: req.notes,
    })
}

pub async fn update_permission_role(req: PermissionRoleUpdateRequest) -> AppResult<PermissionRoleResponse> {
    let db = DB
        .get()
        .ok_or(anyhow::anyhow!("Database connection failed."))?;

    let find = PermissionRole::find_by_id(req.id).one(db).await?;
    if find.is_none() {
        return Err(anyhow::anyhow!("PermissionRole does not exist.").into());
    }
    let mut model: permission_role::ActiveModel = find.unwrap().into();

    model.permission_id = Set(req.permission_id);
    model.role_id = Set(req.role_id);
    model.notes = Set(req.notes);


    let result: permission_role::Model = model.update(db).await?;

    Ok(PermissionRoleResponse {
        id: result.id,
        permission_id: result.permission_id,
        role_id: result.role_id,
        notes: result.notes,

    })
}

pub async fn delete_permission_role(id: i32) -> AppResult<()> {
    let db = DB
        .get()
        .ok_or(anyhow::anyhow!("Database connection failed."))?;
    PermissionRole::delete_by_id(id).exec(db).await?;
    Ok(())
}

pub async fn permission_role_find_all() -> AppResult<Vec<PermissionRoleResponse>> {
    let db = DB
        .get()
        .ok_or(anyhow::anyhow!("Database connection failed."))?;
    let permission_role = PermissionRole::find().all(db).await?;
    let res = permission_role
        .into_iter()
        .map(|r| PermissionRoleResponse {
            id: r.id,
            permission_id: r.permission_id,
            role_id: r.role_id,
            notes: r.notes,

        })
        .collect::<Vec<_>>();
    Ok(res)
}


///添加一个角色
pub async fn add_new_role(req: NewRolePermissionParam) -> AppResult<i32> {
    let db = DB
        .get()
        .ok_or(anyhow::anyhow!("Database connection failed."))?;
    let new_role = add_role(RoleAddRequest { name: req.name.clone(), description: req.description }).await.unwrap();
    let role_id = new_role.id;
    let mk_models: Vec<permission_role::ActiveModel> = req.permission_ids.iter().map(|x| {
        permission_role::ActiveModel {
            id: NotSet,
            permission_id: Set(x.clone()),
            role_id: Set(role_id),
            notes: Set(None),
        }
    }).collect();
    PermissionRole::insert_many(mk_models).exec(db).await?;
    Ok(role_id)
}


///查询某个role下面的所有permission
pub async fn find_role_permission_list(role_id: i32) -> AppResult<Vec<permission::Model>> {
    let db = DB
        .get()
        .ok_or(anyhow::anyhow!("Database connection failed."))?;
    let all_list = PermissionRole::find().filter(
        permission_role::Column::RoleId.eq(role_id)
    ).all(db).await.unwrap();
    let ids : Vec<i32>= all_list.iter().map(|x| x.permission_id).collect();
    let find: Vec<permission::Model> = permission::Entity::find().filter(permission::Column::Id.is_in(ids)).all(db).await?;
    Ok(find)
}