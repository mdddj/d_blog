use sea_orm_migration::prelude::*;

use crate::m20240411_032637_create_product_category_table::ProductCategory;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ProductSkuName::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ProductSkuName::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ProductSkuName::Name).string().not_null())
                    .col(
                        ColumnDef::new(ProductSkuName::ProductCategoryId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk_product_sku_name_product_category")
                            .from(ProductSkuName::Table, ProductSkuName::ProductCategoryId)
                            .to(ProductCategory::Table, ProductCategory::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ProductSkuName::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum ProductSkuName {
    Table,
    Id,
    Name,
    ProductCategoryId,
}
