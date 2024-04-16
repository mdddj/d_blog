//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "product_sku_name")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub product_category_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::product_category::Entity",
        from = "Column::ProductCategoryId",
        to = "super::product_category::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    ProductCategory,
    #[sea_orm(has_many = "super::product_sku_name_value::Entity")]
    ProductSkuNameValue,
    #[sea_orm(has_many = "super::product_sku_value::Entity")]
    ProductSkuValue,
}

impl Related<super::product_category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ProductCategory.def()
    }
}

impl Related<super::product_sku_name_value::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ProductSkuNameValue.def()
    }
}

impl Related<super::product_sku_value::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ProductSkuValue.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}