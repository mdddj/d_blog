use sea_orm::{ActiveModelTrait, EntityTrait, NotSet, Set};

use crate::dtos::product_sku::*;
use crate::entities::prelude::ProductSku;
use crate::{app_writer::AppResult, db::DB, dtos::product_sku::*, entities::*};

pub async fn add_product_sku(req: ProductSkuAddRequest) -> AppResult<ProductSkuResponse> {
    let db = DB
        .get()
        .ok_or(anyhow::anyhow!("Database connection failed."))?;
    let model = product_sku::ActiveModel {
        id: NotSet,
        product_id: Set(req.product_id.clone()),
        sku: Set(req.sku.clone()),
        price: Set(req.price.clone()),
        stock: Set(req.stock.clone()),
        sale_count: Set(req.sale_count.clone()),
    };
    let result = ProductSku::insert(model).exec(db).await?;
    Ok(ProductSkuResponse {
        id: result.last_insert_id,
        product_id: req.product_id,
        sku: req.sku,
        price: req.price,
        stock: req.stock,
        sale_count: req.sale_count,
    })
}
pub async fn update_product_sku(req: ProductSkuUpdateRequest) -> AppResult<ProductSkuResponse> {
    let db = DB
        .get()
        .ok_or(anyhow::anyhow!("Database connection failed."))?;

    let find = ProductSku::find_by_id(req.id).one(db).await?;
    if find.is_none() {
        return Err(anyhow::anyhow!("ProductSku does not exist.").into());
    }
    let mut model: product_sku::ActiveModel = find.unwrap().into();

    model.product_id = Set(req.product_id);
    model.sku = Set(req.sku);
    model.price = Set(req.price);
    model.stock = Set(req.stock);
    model.sale_count = Set(req.sale_count);

    let result: product_sku::Model = model.update(db).await?;

    Ok(ProductSkuResponse {
        id: result.id,
        product_id: result.product_id,
        sku: result.sku,
        price: result.price,
        stock: result.stock,
        sale_count: result.sale_count,
    })
}
pub async fn delete_product_sku(id: i32) -> AppResult<()> {
    let db = DB
        .get()
        .ok_or(anyhow::anyhow!("Database connection failed."))?;
    ProductSku::delete_by_id(id).exec(db).await?;
    Ok(())
}
pub async fn product_sku_find_all() -> AppResult<Vec<ProductSkuResponse>> {
    let db = DB
        .get()
        .ok_or(anyhow::anyhow!("Database connection failed."))?;
    let product_sku = ProductSku::find().all(db).await?;
    let res = product_sku
        .into_iter()
        .map(|r| ProductSkuResponse {
            id: r.id,
            product_id: r.product_id,
            sku: r.sku,
            price: r.price,
            stock: r.stock,
            sale_count: r.sale_count,
        })
        .collect::<Vec<_>>();
    Ok(res)
}

///批量插入数据
pub async fn save_all_sku(sku_vec: Vec<ProductSkuAddRequest>) -> AppResult<()> {
    let db = DB
        .get()
        .ok_or(anyhow::anyhow!("Database connection failed."))?;
    let req_models: Vec<product_sku::ActiveModel> = sku_vec.into_iter().map(|v| v.into()).collect();
    ProductSku::insert_many(req_models).exec(db).await?;
    Ok(())
}
