use sea_orm_migration::prelude::*;

use crate::m20220101_000001_create_table::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // let has_table = manager.has_table("product_category").await?;
        // println!("has product category table: {}", has_table);
        // if has_table {
        //     TableAlterStatement::new()
        //         .table(ProductCategory::ParentProductCategoryId)
        //         .modify_column(ColumnDef::new(ProductCategory::ParentProductCategoryId).integer());
        // }
        manager
            .create_table(
                Table::create()
                    .table(ProductCategory::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ProductCategory::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ProductCategory::Name).string().not_null())
                    .col(ColumnDef::new(ProductCategory::ParentProductCategoryId).integer())
                    .col(
                        ColumnDef::new(ProductCategory::ShopUserId)
                            .string()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk_product_category_user")
                            .from(ProductCategory::Table, ProductCategory::ShopUserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ProductCategory::Table).to_owned())
            .await
    }
}

//产品分类
#[derive(Iden)]
pub enum ProductCategory {
    Table,
    //主键
    Id,
    //名称
    Name,
    //父分类ID
    ParentProductCategoryId,
    //商家ID
    ShopUserId,
}
