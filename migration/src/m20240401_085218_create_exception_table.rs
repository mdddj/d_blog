use sea_orm_migration::prelude::*;
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Exception::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Exception::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Exception::Content).string().not_null())
                    .col(ColumnDef::new(Exception::Os).string().not_null())
                    .col(ColumnDef::new(Exception::PostDate).date().not_null())
                    .col(ColumnDef::new(Exception::CreateDate).date_time().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let has_table = manager.has_table("exception").await?;
        if has_table {
            manager
                .drop_table(Table::drop().table(Exception::Table).to_owned())
                .await?;
        }
        Ok(())
    }
}

#[derive(Iden)]
enum Exception {
    Table,
    Id,
    Os,
    Content,
    PostDate,
    CreateDate,
}
