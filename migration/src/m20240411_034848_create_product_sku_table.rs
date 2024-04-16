use sea_orm_migration::prelude::*;

use crate::m20240411_032202_create_product_table::Product;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ProductSku::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ProductSku::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ProductSku::ProductId).integer().not_null())
                    .col(ColumnDef::new(ProductSku::Sku).string().not_null())
                    .col(
                        ColumnDef::new(ProductSku::Price)
                            .float()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(ProductSku::Stock)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(ProductSku::SaleCount)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk_sku_product")
                            .from(ProductSku::Table, ProductSku::ProductId)
                            .to(Product::Table, Product::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ProductSku::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum ProductSku {
    Table,
    //主键
    Id,
    //产品ID
    ProductId,
    //sku[1:1,2:2]
    Sku,
    //价格
    Price,
    //库存
    Stock,
    //销量
    SaleCount,
}
