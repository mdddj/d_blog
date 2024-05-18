use sea_orm::{ActiveModelTrait, EntityTrait, NotSet};
use sea_orm::ActiveValue::Set;

use crate::{
    app_writer::AppResult,
    db::DB,
    dtos::permission::*,
    entities::*,
};
use crate::dtos::permission::*;
use crate::entities::prelude::Permission;

pub async fn add_permission(req: PermissionAddRequest) -> AppResult<PermissionResponse> {
    let db = DB
        .get()
        .ok_or(anyhow::anyhow!("Database connection failed."))?;
    let model = permission::ActiveModel {
			id: NotSet,
			name: Set(req.name.clone()),
			description: Set(req.description.clone()),
			create_time: Set(req.create_time.clone()),
			permission_url: Set(req.permission_url.clone()),
        r#type: Set(req.r#type.clone()),
        method: Set(req.method.clone()),
        group: Set(req.group.clone()),
    };
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
pub async fn update_permission(req: PermissionUpdateRequest) -> AppResult<PermissionResponse> {
    let db = DB
        .get()
        .ok_or(anyhow::anyhow!("Database connection failed."))?;

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

    Ok(PermissionResponse {
		id: result.id,
		name: result.name,
		description: result.description,
		create_time: result.create_time,
		permission_url: result.permission_url,
        r#type: result.r#type,
        method: result.method,
        group: result.group,
    })
}
pub async fn delete_permission(id: i32) -> AppResult<()> {
    let db = DB
        .get()
        .ok_or(anyhow::anyhow!("Database connection failed."))?;
    Permission::delete_by_id(id).exec(db).await?;
    Ok(())
}
pub async fn permission_find_all() -> AppResult<Vec<PermissionResponse>> {
    let db = DB
        .get()
        .ok_or(anyhow::anyhow!("Database connection failed."))?;
    let permission = Permission::find().all(db).await?;
    let res = permission
        .into_iter()
        .map(|r| PermissionResponse {
			id: r.id,
			name: r.name,
			description: r.description,
			create_time: r.create_time,
			permission_url: r.permission_url,
            r#type: r.r#type,
            method: r.method,
            group: r.group,
        })
        .collect::<Vec<_>>();
    Ok(res)
}


//初始化权限列表
pub async fn init_permission_init_all(){

}