use crate::dtos::product_sku_name::*;
use crate::entities::prelude::ProductSkuName;
use crate::{app_writer::AppResult, db::DB, dtos::product_sku_name::*, entities::*};
use salvo::prelude::{Extractible, ToSchema};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, NotSet, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use validator::Validate;
pub async fn add_product_sku_name(
    req: ProductSkuNameAddRequest,
) -> AppResult<ProductSkuNameResponse> {
    let db = DB
        .get()
        .ok_or(anyhow::anyhow!("Database connection failed."))?;
    let model = product_sku_name::ActiveModel {
        id: NotSet,
        name: Set(req.name.clone()),
        product_category_id: Set(req.product_category_id.clone()),
    };
    let result = ProductSkuName::insert(model).exec(db).await?;
    Ok(ProductSkuNameResponse {
        id: result.last_insert_id,
        name: req.name,
        product_category_id: req.product_category_id,
    })
}
pub async fn update_product_sku_name(
    req: ProductSkuNameUpdateRequest,
) -> AppResult<ProductSkuNameResponse> {
    let db = DB
        .get()
        .ok_or(anyhow::anyhow!("Database connection failed."))?;

    let find = ProductSkuName::find_by_id(req.id).one(db).await?;
    if find.is_none() {
        return Err(anyhow::anyhow!("ProductSkuName does not exist.").into());
    }
    let mut model: product_sku_name::ActiveModel = find.unwrap().into();

    model.name = Set(req.name);
    model.product_category_id = Set(req.product_category_id);

    let result: product_sku_name::Model = model.update(db).await?;

    Ok(ProductSkuNameResponse {
        id: result.id,
        name: result.name,
        product_category_id: result.product_category_id,
    })
}
pub async fn delete_product_sku_name(id: i32) -> AppResult<()> {
    let db = DB
        .get()
        .ok_or(anyhow::anyhow!("Database connection failed."))?;
    ProductSkuName::delete_by_id(id).exec(db).await?;
    Ok(())
}
pub async fn product_sku_name_find_all() -> AppResult<Vec<ProductSkuNameResponse>> {
    let db = DB
        .get()
        .ok_or(anyhow::anyhow!("Database connection failed."))?;
    let product_sku_name = ProductSkuName::find().all(db).await?;
    let res = product_sku_name
        .into_iter()
        .map(|r| ProductSkuNameResponse {
            id: r.id,
            name: r.name,
            product_category_id: r.product_category_id,
        })
        .collect::<Vec<_>>();
    Ok(res)
}

///保存一个
pub async fn product_sku_value_save() {}
