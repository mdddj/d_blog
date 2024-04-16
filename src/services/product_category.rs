use sea_orm::{ActiveModelTrait, EntityTrait, NotSet, Set};

use crate::entities::prelude::ProductCategory;
use crate::{app_writer::AppResult, db::DB, dtos::product_category::*, entities::*};

pub async fn add_product_category(
    req: ProductCategoryAddRequest,
) -> AppResult<ProductCategoryResponse> {
    let db = DB
        .get()
        .ok_or(anyhow::anyhow!("Database connection failed."))?;
    let model = product_category::ActiveModel {
        id: NotSet,
        name: Set(req.name.clone()),
        parent_product_category_id: Set(req.parent_product_category_id.clone()),
        shop_user_id: Set(req.shop_user_id.clone()),
    };
    let result = ProductCategory::insert(model).exec(db).await?;
    Ok(ProductCategoryResponse {
        id: result.last_insert_id,
        name: req.name,
        parent_product_category_id: req.parent_product_category_id,
        shop_user_id: req.shop_user_id,
    })
}
pub async fn update_product_category(
    req: ProductCategoryUpdateRequest,
) -> AppResult<ProductCategoryResponse> {
    let db = DB
        .get()
        .ok_or(anyhow::anyhow!("Database connection failed."))?;

    let find = ProductCategory::find_by_id(req.id).one(db).await?;
    if find.is_none() {
        return Err(anyhow::anyhow!("ProductCategory does not exist.").into());
    }
    let mut model: product_category::ActiveModel = find.unwrap().into();

    model.name = Set(req.name);
    model.parent_product_category_id = Set(req.parent_product_category_id);
    model.shop_user_id = Set(req.shop_user_id);

    let result: product_category::Model = model.update(db).await?;

    Ok(ProductCategoryResponse {
        id: result.id,
        name: result.name,
        parent_product_category_id: result.parent_product_category_id,
        shop_user_id: result.shop_user_id,
    })
}
pub async fn delete_product_category(id: i32) -> AppResult<()> {
    let db = DB
        .get()
        .ok_or(anyhow::anyhow!("Database connection failed."))?;
    ProductCategory::delete_by_id(id).exec(db).await?;
    Ok(())
}
pub async fn product_category_find_all() -> AppResult<Vec<ProductCategoryResponse>> {
    let db = DB
        .get()
        .ok_or(anyhow::anyhow!("Database connection failed."))?;
    let product_category = ProductCategory::find().all(db).await?;
    let res = product_category
        .into_iter()
        .map(|r| ProductCategoryResponse {
            id: r.id,
            name: r.name,
            parent_product_category_id: r.parent_product_category_id,
            shop_user_id: r.shop_user_id,
        })
        .collect::<Vec<_>>();
    Ok(res)
}

pub async fn product_category_find_by_id(id: i32) -> AppResult<ProductCategoryResponse> {
    let db = DB
        .get()
        .ok_or(anyhow::anyhow!("Database connection failed."))?;
    let find = ProductCategory::find_by_id(id).one(db).await?;
    let find = find.ok_or(anyhow::anyhow!("产品分类不存在"))?;
    Ok(ProductCategoryResponse {
        id,
        name: find.name,
        parent_product_category_id: find.parent_product_category_id,
        shop_user_id: find.shop_user_id,
    })
}
