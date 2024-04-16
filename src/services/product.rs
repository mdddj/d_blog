use sea_orm::{ActiveModelTrait, EntityTrait, NotSet, Set};

use crate::dtos::product_sku_value::{ProductSkuNameVecValueModel, SkuFullModel};
use crate::entities::prelude::Product;
use crate::services::product_category::product_category_find_by_id;
use crate::services::product_sku_value::{product_sku_vec_value_save, save_product_all};
use crate::{app_writer::AppResult, db::DB, dtos::product::*, entities::*};

pub async fn add_product(req: ProductAddRequest) -> AppResult<ProductResponse> {
    let db = DB
        .get()
        .ok_or(anyhow::anyhow!("Database connection failed."))?;
    println!("body json is :{:?}", req);
    product_category_find_by_id(req.product_category_id).await?; //验证产品分类是否存在
    let model = product::ActiveModel {
        id: NotSet,
        name: Set(req.name.clone()),
        product_category_id: Set(req.product_category_id.clone()),
        sale_count: Set(0),
        shop_user_id: Set(req.shop_user_id.clone()),
        comment_count: Set(0),
    };
    let result = Product::insert(model).exec(db).await?;

    let product_response = ProductResponse {
        id: result.last_insert_id,
        name: req.name,
        product_category_id: req.product_category_id,
        sale_count: 0,
        shop_user_id: req.shop_user_id,
        comment_count: 0,
    };
    //插入sku
    let sku_params_01: Vec<ProductSkuNameVecValueModel> = req
        .sku_raw_list
        .iter()
        .map(|v| ProductSkuNameVecValueModel::from_product_and_request(&product_response, &v))
        .collect();

    //获取参数2
    let params_02_task = sku_params_01
        .clone()
        .into_iter()
        .map(product_sku_vec_value_save);
    let params_result = futures::future::join_all(params_02_task).await;
    let params_02: Vec<SkuFullModel> = params_result.into_iter().map(|v| v.unwrap()).collect();
    save_product_all(sku_params_01, params_02,req.price, product_response.clone()).await;
    Ok(product_response)
}
pub async fn update_product(req: ProductUpdateRequest) -> AppResult<ProductResponse> {
    let db = DB
        .get()
        .ok_or(anyhow::anyhow!("Database connection failed."))?;

    let find = Product::find_by_id(req.id).one(db).await?;
    if find.is_none() {
        return Err(anyhow::anyhow!("Product does not exist.").into());
    }
    let mut model: product::ActiveModel = find.unwrap().into();

    model.name = Set(req.name);
    model.product_category_id = Set(req.product_category_id);
    model.sale_count = Set(req.sale_count);
    model.shop_user_id = Set(req.shop_user_id);
    model.comment_count = Set(req.comment_count);

    let result: product::Model = model.update(db).await?;

    Ok(ProductResponse {
        id: result.id,
        name: result.name,
        product_category_id: result.product_category_id,
        sale_count: result.sale_count,
        shop_user_id: result.shop_user_id,
        comment_count: result.comment_count,
    })
}
pub async fn delete_product(id: i32) -> AppResult<()> {
    let db = DB
        .get()
        .ok_or(anyhow::anyhow!("Database connection failed."))?;
    Product::delete_by_id(id).exec(db).await?;
    Ok(())
}
pub async fn product_find_all() -> AppResult<Vec<ProductResponse>> {
    let db = DB
        .get()
        .ok_or(anyhow::anyhow!("Database connection failed."))?;
    let product = Product::find().all(db).await?;
    let res = product
        .into_iter()
        .map(|r| ProductResponse {
            id: r.id,
            name: r.name,
            product_category_id: r.product_category_id,
            sale_count: r.sale_count,
            shop_user_id: r.shop_user_id,
            comment_count: r.comment_count,
        })
        .collect::<Vec<_>>();
    Ok(res)
}
