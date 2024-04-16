use salvo::prelude::{Extractible, ToSchema};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::product::{ProductResponse, ProductSkuNameRequestParam};

///sku的库存和价格
#[derive(Debug, Serialize, Deserialize,Clone,ToSchema)]
pub struct ProductPriceAndStock {
    ///sku完整的名称
    pub sku_full_name: String,
    ///产品价格
    pub price: f64,
    ///产品库存
    pub stock: i32,

}

///
impl ProductSkuNameVecValueModel {
    //参数转换
    pub fn from_product_and_request(
        product: &ProductResponse,
        sku_model: &ProductSkuNameRequestParam,
    ) -> ProductSkuNameVecValueModel {
        ProductSkuNameVecValueModel {
            name: sku_model.name.clone(),
            values: sku_model.sku_list.iter().map(|v| v.value.clone()).collect(),
            product_id: product.id,
            product_category_id: product.product_category_id,
        }
    }
}

/// sku名称-sku值模型
#[derive(Deserialize, Serialize, Debug, ToSchema, Clone)]
pub struct ProductSkuNameVecValueModel {
    ///sku名称
    pub name: String,
    ///sku值列表
    pub values: Vec<String>,
    ///产品ID
    pub product_id: i32,
    ///产品分类ID
    pub product_category_id: i32,
}

#[derive(Deserialize, Debug, Validate, ToSchema, Default)]
pub struct ProductSkuValueAddRequest {
    pub name: String,
    pub product_sku_name_id: i32,
}

#[derive(Debug, Deserialize, Extractible, ToSchema, Default)]
#[salvo(extract(default_source(from = "body", parse = "json")))]
pub struct ProductSkuValueUpdateRequest {
    #[salvo(extract(source(from = "param")))]
    pub id: i32,
    pub name: String,
    pub product_sku_name_id: i32,
}

///
#[derive(Debug, Serialize, ToSchema, Deserialize, Default, Clone)]
pub struct ProductSkuValueResponse {
    pub id: i32,
    pub name: String,
    pub product_sku_name_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkuFullModel {
    pub id: i32,
    pub sku_name: String,
    pub sku_list: Vec<ProductSkuValueResponse>,
}
