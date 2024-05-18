use salvo::prelude::{Extractible, ToSchema};
use serde::{Deserialize, Serialize};
use validator::Validate;use crate::{
    app_writer::AppResult,
    db::DB,
    dtos::role::*,
    entities::*,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, NotSet};
use crate::dtos::role::*;
use crate::entities::prelude::Role;
pub async fn add_role(req: RoleAddRequest) -> AppResult<RoleResponse> {
    let db = DB
        .get()
        .ok_or(anyhow::anyhow!("Database connection failed."))?;
    let model = role::ActiveModel {
			id: NotSet,
			name: Set(req.name.clone()),
			description: Set(req.description.clone())
    };
    let result = Role::insert(model).exec(db).await?;
    Ok(RoleResponse {
        id: result.last_insert_id,
		name: req.name,
		description: req.description
    })
}
pub async fn update_role(req: RoleUpdateRequest) -> AppResult<RoleResponse> {
    let db = DB
        .get()
        .ok_or(anyhow::anyhow!("Database connection failed."))?;

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

    })
}
pub async fn delete_role(id: i32) -> AppResult<()> {
    let db = DB
        .get()
        .ok_or(anyhow::anyhow!("Database connection failed."))?;
    Role::delete_by_id(id).exec(db).await?;
    Ok(())
}
pub async fn role_find_all() -> AppResult<Vec<RoleResponse>> {
    let db = DB
        .get()
        .ok_or(anyhow::anyhow!("Database connection failed."))?;
    let role = Role::find().all(db).await?;
    let res = role
        .into_iter()
        .map(|r| RoleResponse {
			id: r.id,
			name: r.name,
			description: r.description,

        })
        .collect::<Vec<_>>();
    Ok(res)
}
