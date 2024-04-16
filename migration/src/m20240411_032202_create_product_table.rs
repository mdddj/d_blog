use sea_orm_migration::prelude::*;

use crate::{
    m20220101_000001_create_table::Users,
    m20240411_032637_create_product_category_table::ProductCategory,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Product::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Product::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Product::Name).string().not_null())
                    .col(
                        ColumnDef::new(Product::ProductCategoryId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Product::SaleCount)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(Product::ShopUserId).string().not_null())
                    .col(
                        ColumnDef::new(Product::CommentCount)
                            .integer()
                            .default(0)
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk_product_user")
                            .from(Product::Table, Product::ShopUserId)
                            .to(Users::Table, Users::Id),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk_product_category")
                            .from(Product::Table, Product::ProductCategoryId)
                            .to(ProductCategory::Table, ProductCategory::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Product::Table).to_owned())
            .await
    }
}

//产品实体
#[derive(Iden)]
pub enum Product {
    Table,
    //主键
    Id,
    //名称
    Name,
    //产品分类ID
    ProductCategoryId,
    //商家ID
    ShopUserId,
    //销量
    SaleCount,
    //评价
    CommentCount,
}
