use sea_orm_migration::prelude::*;
use crate::m20240407_054555_create_post_table::Post;
use crate::m20240407_055250_create_tag_table::Tag;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(PostTag::Table)
                    .if_not_exists()
                    // .col(ColumnDef::new(PostTag::VirtualId).integer().primary_key().auto_increment())
                    .col(ColumnDef::new(PostTag::PostId).integer().not_null())
                    .col(ColumnDef::new(PostTag::TagId).integer().not_null())
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk_post_tag_post")
                            .from(PostTag::Table,PostTag::PostId)
                            .to(Post::Table,Post::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk_post_tag_tag")
                            .from(PostTag::Table,PostTag::TagId)
                            .to(Tag::Table,Tag::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .primary_key(Index::create().col(PostTag::PostId).col(PostTag::TagId))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(PostTag::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum PostTag {
    Table,
    PostId,
    TagId,
    // VirtualId
}
