use sea_orm_migration::prelude::*;
use crate::m20240502_062809_create_role_table::Role;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.alter_table(
            Table::alter()
                .table(Role::Table)
                .add_column(ColumnDef::new(Role::CanDelete).boolean().default(true))
                .to_owned()
        )
            .await
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}

