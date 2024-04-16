use sea_orm_migration::prelude::*;

use crate::m20240411_053356_create_product_sku_name_table::ProductSkuName;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ProductSkuValue::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ProductSkuValue::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ProductSkuValue::Name).string().not_null())
                    .col(
                        ColumnDef::new(ProductSkuValue::ProductSkuNameId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk_product_sku_value_product_sku_name")
                            .from(ProductSkuValue::Table, ProductSkuValue::ProductSkuNameId)
                            .to(ProductSkuName::Table, ProductSkuName::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ProductSkuValue::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum ProductSkuValue {
    Table,
    Id,
    Name,
    ProductSkuNameId,
}
