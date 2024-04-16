use salvo::prelude::{Extractible, ToSchema};
use sea_orm::{ActiveValue::NotSet, Set};
use serde::{Deserialize, Serialize};
use validator::Validate;
#[derive(Deserialize, Debug, Validate, ToSchema, Default)]
pub struct ProductSkuAddRequest {
    pub product_id: i32,
    pub sku: String,
    pub price: f64,
    pub stock: i32,
    pub sale_count: i32,
}

impl From<ProductSkuAddRequest> for crate::entities::product_sku::ActiveModel {
    fn from(request: ProductSkuAddRequest) -> Self {
        return crate::entities::product_sku::ActiveModel {
            id: NotSet,
            product_id: Set(request.product_id),
            sku: Set(request.sku),
            price: Set(request.price),
            stock: Set(request.stock),
            sale_count: Set(request.sale_count),
        };
    }
}

#[derive(Debug, Deserialize, Extractible, ToSchema, Default)]
#[salvo(extract(default_source(from = "body", parse = "json")))]
pub struct ProductSkuUpdateRequest {
    #[salvo(extract(source(from = "param")))]
    pub id: i32,
    pub product_id: i32,
    pub sku: String,
    pub price: f64,
    pub stock: i32,
    pub sale_count: i32,
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct ProductSkuResponse {
    pub id: i32,
    pub product_id: i32,
    pub sku: String,
    pub price: f64,
    pub stock: i32,
    pub sale_count: i32,
}

#[derive(Debug, Clone)]
pub struct SkuCombination {
    pub combinations: Vec<(String, String)>,
}

impl SkuCombination {
    pub fn new(combinations: Vec<(String, String)>) -> Self {
        SkuCombination { combinations }
    }
}
