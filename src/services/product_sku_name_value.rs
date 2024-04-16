use sea_orm::{ActiveModelTrait, EntityTrait, NotSet, Set};

use crate::{
    app_writer::AppResult,
    db::DB,
    dtos::product_sku_name_value::*,
    entities::*,
};
use crate::dtos::product_sku_name_value::*;
use crate::entities::prelude::ProductSkuNameValue;

pub async fn add_product_sku_name_value(req: ProductSkuNameValueAddRequest) -> AppResult<ProductSkuNameValueResponse> {
    let db = DB
        .get()
        .ok_or(anyhow::anyhow!("Database connection failed."))?;
    let model = product_sku_name_value::ActiveModel {
			id: NotSet,
			product_id: Set(req.product_id.clone()),
			sku_name_id: Set(req.sku_name_id.clone()),
			sku_value_id: Set(req.sku_value_id.clone())
    };
    let result = ProductSkuNameValue::insert(model).exec(db).await?;
    Ok(ProductSkuNameValueResponse {
        id: result.last_insert_id,
		product_id: req.product_id,
		sku_name_id: req.sku_name_id,
		sku_value_id: req.sku_value_id
    })
}
pub async fn update_product_sku_name_value(req: ProductSkuNameValueUpdateRequest) -> AppResult<ProductSkuNameValueResponse> {
    let db = DB
        .get()
        .ok_or(anyhow::anyhow!("Database connection failed."))?;

    let find = ProductSkuNameValue::find_by_id(req.id).one(db).await?;
    if find.is_none() {
        return Err(anyhow::anyhow!("ProductSkuNameValue does not exist.").into());
    }
    let mut model: product_sku_name_value::ActiveModel = find.unwrap().into();

	model.product_id = Set(req.product_id);
	model.sku_name_id = Set(req.sku_name_id);
	model.sku_value_id = Set(req.sku_value_id);


    let result: product_sku_name_value::Model = model.update(db).await?;

    Ok(ProductSkuNameValueResponse {
		id: result.id,
		product_id: result.product_id,
		sku_name_id: result.sku_name_id,
		sku_value_id: result.sku_value_id,

    })
}
pub async fn delete_product_sku_name_value(id: i32) -> AppResult<()> {
    let db = DB
        .get()
        .ok_or(anyhow::anyhow!("Database connection failed."))?;
    ProductSkuNameValue::delete_by_id(id).exec(db).await?;
    Ok(())
}
pub async fn product_sku_name_value_find_all() -> AppResult<Vec<ProductSkuNameValueResponse>> {
    let db = DB
        .get()
        .ok_or(anyhow::anyhow!("Database connection failed."))?;
    let product_sku_name_value = ProductSkuNameValue::find().all(db).await?;
    let res = product_sku_name_value
        .into_iter()
        .map(|r| ProductSkuNameValueResponse {
			id: r.id,
			product_id: r.product_id,
			sku_name_id: r.sku_name_id,
			sku_value_id: r.sku_value_id,

        })
        .collect::<Vec<_>>();
    Ok(res)
}
