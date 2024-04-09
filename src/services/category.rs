
use crate::{
    app_writer::AppResult,
    db::DB,
    dtos::category::*,
    entities::*,
};
use sea_orm::{ActiveModelTrait, EntityTrait, Set, NotSet};
use crate::entities::prelude::Category;
pub async fn add_category(req: CategoryAddRequest) -> AppResult<CategoryResponse> {
    let db = DB
        .get()
        .ok_or(anyhow::anyhow!("Database connection failed."))?;
    let model = category::ActiveModel {
			id: NotSet,
			name: Set(req.name.clone()),

    };
    let result = Category::insert(model).exec(db).await?;
    Ok(CategoryResponse {
        id: result.last_insert_id,
		name: req.name,

    })
}
pub async fn update_category(req: CategoryUpdateRequest) -> AppResult<CategoryResponse> {
    let db = DB
        .get()
        .ok_or(anyhow::anyhow!("Database connection failed."))?;

    let find = Category::find_by_id(req.id).one(db).await?;
    if find.is_none() {
        return Err(anyhow::anyhow!("Category does not exist.").into());
    }
    let mut model: category::ActiveModel = find.unwrap().into();

	model.name = Set(req.name);


    let result: category::Model = model.update(db).await?;

    Ok(CategoryResponse {
		id: result.id,
		name: result.name,

    })
}
pub async fn delete_category(id: i32) -> AppResult<()> {
    let db = DB
        .get()
        .ok_or(anyhow::anyhow!("Database connection failed."))?;
    Category::delete_by_id(id).exec(db).await?;
    Ok(())
}
pub async fn category_find_all() -> AppResult<Vec<CategoryResponse>> {
    let db = DB
        .get()
        .ok_or(anyhow::anyhow!("Database connection failed."))?;
    let category = Category::find().all(db).await?;
    let res = category
        .into_iter()
        .map(|r| CategoryResponse {
			id: r.id,
			name: r.name,

        })
        .collect::<Vec<_>>();
    Ok(res)
}





