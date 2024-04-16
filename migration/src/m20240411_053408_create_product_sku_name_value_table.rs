use sea_orm_migration::prelude::*;

use crate::{
    m20240411_032202_create_product_table::Product,
    m20240411_053356_create_product_sku_name_table::ProductSkuName,
    m20240411_053401_create_product_sku_value_table::ProductSkuValue,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ProductSkuNameValue::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ProductSkuNameValue::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ProductSkuNameValue::ProductId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProductSkuNameValue::SkuNameId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProductSkuNameValue::SkuValueId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk_product_sku_name_value_product")
                            .from(ProductSkuNameValue::Table, ProductSkuNameValue::ProductId)
                            .to(Product::Table, Product::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk_product_sku_name_value_sku_name")
                            .from(ProductSkuNameValue::Table, ProductSkuNameValue::SkuNameId)
                            .to(ProductSkuName::Table, ProductSkuName::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk_product_sku_name_value_sku_value")
                            .from(ProductSkuNameValue::Table, ProductSkuNameValue::SkuValueId)
                            .to(ProductSkuValue::Table, ProductSkuValue::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ProductSkuNameValue::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum ProductSkuNameValue {
    Table,
    Id,
    ProductId,
    SkuNameId,
    SkuValueId,
}
