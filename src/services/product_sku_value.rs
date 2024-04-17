use crate::dtos::product::ProductResponse;
use crate::dtos::product_sku::ProductSkuAddRequest;
use crate::dtos::product_sku_name_value::{
    ProductSkuNameValueAddRequest, ProductSkuNameValueResponse,
};
use crate::dtos::product_sku_value::*;
use crate::entities::prelude::{ProductSkuName, ProductSkuValue};
use crate::services::product_sku::add_product_sku;
use crate::services::product_sku_name_value::add_product_sku_name_value;
use crate::utils::sku::list_all_possible_skus;
use crate::{app_writer::AppResult, db::DB, dtos::product_sku_value::*, entities::*};
use futures::future;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, EntityTrait, NotSet};
use tokio::test;

use super::product_sku::save_all_sku;

pub async fn add_product_sku_value(
    req: ProductSkuValueAddRequest,
) -> AppResult<ProductSkuValueResponse> {
    let db = DB
        .get()
        .ok_or(anyhow::anyhow!("Database connection failed."))?;
    let model = product_sku_value::ActiveModel {
        id: NotSet,
        name: Set(req.name.clone()),
        product_sku_name_id: Set(req.product_sku_name_id.clone()),
    };
    let result = ProductSkuValue::insert(model).exec(db).await?;
    Ok(ProductSkuValueResponse {
        id: result.last_insert_id,
        name: req.name,
        product_sku_name_id: req.product_sku_name_id,
    })
}

pub async fn update_product_sku_value(
    req: ProductSkuValueUpdateRequest,
) -> AppResult<ProductSkuValueResponse> {
    let db = DB
        .get()
        .ok_or(anyhow::anyhow!("Database connection failed."))?;

    let find = ProductSkuValue::find_by_id(req.id).one(db).await?;
    if find.is_none() {
        return Err(anyhow::anyhow!("ProductSkuValue does not exist.").into());
    }
    let mut model: product_sku_value::ActiveModel = find.unwrap().into();

    model.name = Set(req.name);
    model.product_sku_name_id = Set(req.product_sku_name_id);

    let result: product_sku_value::Model = model.update(db).await?;

    Ok(ProductSkuValueResponse {
        id: result.id,
        name: result.name,
        product_sku_name_id: result.product_sku_name_id,
    })
}

pub async fn delete_product_sku_value(id: i32) -> AppResult<()> {
    let db = DB
        .get()
        .ok_or(anyhow::anyhow!("Database connection failed."))?;
    ProductSkuValue::delete_by_id(id).exec(db).await?;
    Ok(())
}

pub async fn product_sku_value_find_all() -> AppResult<Vec<ProductSkuValueResponse>> {
    let db = DB
        .get()
        .ok_or(anyhow::anyhow!("Database connection failed."))?;
    let product_sku_value = ProductSkuValue::find().all(db).await?;
    let res = product_sku_value
        .into_iter()
        .map(|r| ProductSkuValueResponse {
            id: r.id,
            name: r.name,
            product_sku_name_id: r.product_sku_name_id,
        })
        .collect::<Vec<_>>();
    Ok(res)
}

///保存sku值-value的映射
pub async fn product_sku_vec_value_save(
    param: ProductSkuNameVecValueModel,
) -> AppResult<SkuFullModel> {
    let db = DB.get().ok_or(anyhow::anyhow!("server error!"))?;
    let sku_name_set = param.name;
    let product_id = param.product_id;
    let sku_name = product_sku_name::ActiveModel {
        name: Set(sku_name_set.clone()),
        product_category_id: Set(product_id),
        id: NotSet,
    };
    ///保存sku的name
    let insert_result = ProductSkuName::insert(sku_name).exec(db).await?;
    let sku_name_id = insert_result.last_insert_id;
    ///保存sku的值
    let sku_value_req_param: Vec<ProductSkuValueAddRequest> = param
        .values
        .iter()
        .map(move |name| ProductSkuValueAddRequest {
            name: name.to_owned(),
            product_sku_name_id: param.product_category_id,
        })
        .collect();
    let task_list = sku_value_req_param.into_iter().map(add_product_sku_value);
    let task_result_list = future::join_all(task_list).await; //保存sku数据
    let task_all_response: Vec<ProductSkuValueResponse> = task_result_list
        .into_iter()
        .map(move |x2| x2.unwrap())
        .collect();

    let sku_value_vec = task_all_response
        .clone()
        .into_iter()
        .map(move |x1| x1)
        .collect();

    ///保存映射关系
    let sku_name_and_value_param: Vec<ProductSkuNameValueAddRequest> = task_all_response
        .into_iter()
        .map(move |x| ProductSkuNameValueAddRequest {
            product_id,
            sku_name_id,
            sku_value_id: x.id,
        })
        .collect();
    let sku_name_value_task = sku_name_and_value_param
        .into_iter()
        .map(add_product_sku_name_value);
    let _sku_name_value_param_result: Vec<AppResult<ProductSkuNameValueResponse>> =
        future::join_all(sku_name_value_task).await; //保存成功映射关系product+sku+name+value

    Ok(SkuFullModel {
        id: sku_name_id,
        sku_name: sku_name_set.clone(),
        sku_list: sku_value_vec,
    })
}

fn find_matching_id(
    sku_name: &str,
    sku_value: &str,
    sku_full_model: &Vec<SkuFullModel>,
) -> Option<(i32, i32)> {
    for sku_model in sku_full_model {
        if sku_model.sku_name == sku_name {
            for sku_value_response in &sku_model.sku_list {
                if sku_value_response.name == sku_value {
                    return Some((sku_model.id, sku_value_response.id));
                }
            }
        }
    }
    None
}

fn find_matching_price_and_stock<'a>(
    names_str: &str,
    price_stock_vec: &'a Vec<ProductPriceAndStock>,
) -> Option<&'a ProductPriceAndStock> {
    for item in price_stock_vec {
        if item.sku_full_name == names_str {
            return Some(item);
        }
    }
    None
}
///
pub async fn save_product_all(
    sku_list: Vec<ProductSkuNameVecValueModel>,
    sku_full_model: Vec<SkuFullModel>,
    price_stock: Vec<ProductPriceAndStock>,
    product: ProductResponse,
) {
    let all_sku = list_all_possible_skus(&sku_list);

    let mut sku_task = Vec::new();
    for (_index, sku_combination) in all_sku.clone().iter().enumerate() {
        let mut ids = Vec::new();
        let mut names = Vec::new();
        for (sku_name, sku_value) in sku_combination.combinations.iter() {
            if let Some(id) = find_matching_id(&sku_name, &sku_value, &sku_full_model) {
                ids.push(id);
            }
            names.push(format!("{}:{}", sku_name, sku_value))
        }
        let formatted_strings: Vec<String> =
            ids.iter().map(|&(x, y)| format!("{}:{}", x, y)).collect();
        let json_str = formatted_strings.join(",");
        let names_format = names.join(",");
        println!(
            "ids: {:?}   组装: {:?}   名字:{:?}  名字组装:{:?}",
            ids, json_str, names, names_format
        );
        if let Some(price_item) = find_matching_price_and_stock(&names_format, &price_stock) {
            println!("找到了价格:{:?}", price_item);
            let make_price_model = ProductSkuAddRequest {
                product_id: product.id,
                sku: "test_sku".to_owned(),
                price: price_item.price,
                stock: price_item.stock,
                sale_count: 0,
            };
            sku_task.push(make_price_model);
        }
    }
    let _ = save_all_sku(sku_task).await;
    println!("insert success")
}

///测试sku
#[test]
async fn test_save_all() {
    let sku_full_model = vec![
        SkuFullModel {
            id: 1,
            sku_name: "颜色".to_string(),
            sku_list: vec![
                ProductSkuValueResponse {
                    id: 1,
                    name: "红色".to_string(),
                    product_sku_name_id: 1,
                },
                ProductSkuValueResponse {
                    id: 3,
                    name: "蓝色".to_string(),
                    product_sku_name_id: 1,
                },
            ],
        },
        SkuFullModel {
            id: 2,
            sku_name: "尺码".to_string(),
            sku_list: vec![
                ProductSkuValueResponse {
                    id: 2,
                    name: "小".to_string(),
                    product_sku_name_id: 2,
                },
                ProductSkuValueResponse {
                    id: 4,
                    name: "中".to_string(),
                    product_sku_name_id: 2,
                },
            ],
        },
    ];

    let sku_list = vec![
        ProductSkuNameVecValueModel {
            name: "颜色".to_string(),
            values: vec!["红色".to_string(), "蓝色".to_string()],
            product_id: 1,
            product_category_id: 1,
        },
        ProductSkuNameVecValueModel {
            name: "尺码".to_string(),
            values: vec!["小".to_string(), "中".to_string()],
            product_id: 1,
            product_category_id: 1,
        },
    ];

    let price = vec![ProductPriceAndStock {
        sku_full_name: "颜色:红色,尺码:小".to_string(),
        price: 120.0,
        stock: 100,
    }];
    let product = ProductResponse {
        id: 1,
        name: "test product".to_owned(),
        product_category_id: 1,
        sale_count: 0,
        shop_user_id: "1454002e-6fc3-4a90-bb9d-37c7802f6a43".to_owned(),
        comment_count: 0,
    };

    // 调用函数进行测试
    save_product_all(sku_list, sku_full_model, price, product).await;
}

#[cfg(test)]
mod tests {

    use salvo::prelude::*;
    use salvo::test::{ResponseExt, TestClient};

    use crate::dtos::product::{
        ProductAddRequest, ProductSkuNameRequestParam, ProductSkuValueRequestParam,
    };

    #[tokio::test]
    async fn test_hello_world() {
        let service = Service::new(crate::routers::router());

        let content = TestClient::get(format!("http://127.0.0.1:5800/"))
            .send(&service)
            .await
            .take_string()
            .await
            .unwrap();
        assert_eq!(content, "Hello World");
    }

    ///测试保存产品
    #[tokio::test]
    async fn test_save_product() {
        let sku_list = vec![
            ProductSkuNameRequestParam {
                name: "颜色".to_owned(),
                sku_list: vec![
                    ProductSkuValueRequestParam {
                        value: "黄色".to_owned(),
                    },
                    ProductSkuValueRequestParam {
                        value: "蓝色".to_owned(),
                    },
                    ProductSkuValueRequestParam {
                        value: "红色".to_owned(),
                    },
                ],
            },
            ProductSkuNameRequestParam {
                name: "尺码".to_owned(),
                sku_list: vec![
                    ProductSkuValueRequestParam {
                        value: "xl".to_owned(),
                    },
                    ProductSkuValueRequestParam {
                        value: "xxl".to_owned(),
                    },
                    ProductSkuValueRequestParam {
                        value: "xxxl".to_owned(),
                    },
                ],
            },
        ];
        let json_body = ProductAddRequest {
            name: "test product".to_owned(),
            product_category_id: 1,
            shop_user_id: "".to_owned(),
            sku_raw_list: sku_list,
            price: vec![],
        };
        let json_str = serde_json::to_string(&json_body).unwrap();
        println!("{}", json_str);
        // let response = TestClient::post(format!("http://127.0.0.1:5800/api/product"))
        //     .json(&json_body)
        //     .send(&service)
        //     .await
        //     .take_string()
        //     .await
        //     .unwrap();
        // println!("response:{:?}", response)
    }
}
