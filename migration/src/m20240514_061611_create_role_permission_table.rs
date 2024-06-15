use sea_orm_migration::prelude::*;
use crate::m20240502_062735_create_permission_table::Permission;
use crate::m20240502_062809_create_role_table::Role;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PermissionRole::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PermissionRole::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PermissionRole::PermissionId).integer().not_null())
                    .col(ColumnDef::new(PermissionRole::RoleId).integer().not_null())
                    .col(ColumnDef::new(PermissionRole::Notes).string().default("-"))
                    .foreign_key(ForeignKeyCreateStatement::new().name("fk_permission").from(
                        PermissionRole::Table, PermissionRole::PermissionId,
                    ).to(Permission::Table, Permission::Id).on_delete(ForeignKeyAction::Cascade))
                    .foreign_key(ForeignKeyCreateStatement::new().name("fk_role").from(PermissionRole::Table, PermissionRole::RoleId).to(Role::Table, Role::Id))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PermissionRole::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum PermissionRole {
    Table,
    Id,
    RoleId,
    PermissionId,
    Notes,
}
