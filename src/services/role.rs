use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, ModelTrait, NotSet, QueryFilter, Set};

use crate::{
    app_writer::AppResult,
    dtos::role::*,
    entities::*,
};
use crate::db::get_db;
use crate::dtos::permission_role::NewRolePermissionParam;
use crate::dtos::role::*;
use crate::entities::prelude::Role;
use crate::services::permission::permission_find_all;
use crate::services::permission_role::add_new_role;


///添加角色
pub async fn add_role(req: RoleAddRequest) -> AppResult<RoleResponse> {
    let db = get_db();


    let find = Role::find().filter(role::Column::Name.eq(req.name.clone())).one(db).await?;
    if let Some(f) = find {
        let _ = f.delete(db).await;
    }

    let model = role::ActiveModel {
			id: NotSet,
			name: Set(req.name.clone()),
			description: Set(req.description.clone()),
        can_delete: Set(req.can_delete),
    };
    let result = Role::insert(model).exec(db).await?;
    Ok(RoleResponse {
        id: result.last_insert_id,
		name: req.name,
		description: req.description,
        can_delete: req.can_delete,
    })
}


///修改角色
pub async fn update_role(req: RoleUpdateRequest) -> AppResult<RoleResponse> {
    let db = get_db();
    let find = Role::find_by_id(req.id).one(db).await?;
    if find.is_none() {
        return Err(anyhow::anyhow!("Role does not exist.").into());
    }
    let mut model: role::ActiveModel = find.unwrap().into();

	model.name = Set(req.name);
	model.description = Set(req.description);

    let result: role::Model = model.update(db).await?;

    Ok(RoleResponse {
		id: result.id,
		name: result.name,
		description: result.description,
        can_delete: result.can_delete,
    })
}

///删除角色
pub async fn delete_role(id: i32) -> AppResult<()> {
    let db = get_db();
    Role::delete_by_id(id).exec(db).await?;
    Ok(())
}

///获取全部校色
pub async fn role_find_all() -> AppResult<Vec<RoleResponse>> {
    let db = get_db();
    let role = Role::find().all(db).await?;
    let res = role
        .into_iter()
        .map(|r| RoleResponse {
			id: r.id,
			name: r.name,
			description: r.description,
            can_delete: r.can_delete,
        })
        .collect::<Vec<_>>();
    Ok(res)
}


///初始化超级管理员权限
pub async fn init_admin_role() -> Option<i32> {
    let all_per = permission_find_all().await.unwrap();
    let ids = all_per.iter().map(|x|x.id).collect();
    let admin_role_param = NewRolePermissionParam {
        name: "admin".to_string(),
        permission_ids: ids,
        description: Some("super admin".to_string()),
        can_delete: Some(false),
    };
    let d = add_new_role(admin_role_param).await;
    d.ok()
}