use sea_orm_migration::prelude::*;
use crate::m20220101_000001_create_table::Users;
use crate::m20240502_062809_create_role_table::Role;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(Table::create().table(UserRole::Table).if_not_exists()
                                 .col(ColumnDef::new(UserRole::Id).integer().not_null().auto_increment().primary_key())
                                 .col(ColumnDef::new(UserRole::UserId).string().not_null())
                                 .col(ColumnDef::new(UserRole::RoleId).integer().not_null())
                                 .foreign_key(ForeignKeyCreateStatement::new()
                                     .name("fk_user_role")
                                     .from(UserRole::Table, UserRole::UserId)
                                     .to(Users::Table, Users::Id)
                                     .on_delete(ForeignKeyAction::Cascade))
                                 .foreign_key(ForeignKeyCreateStatement::new()
                                     .name("fk_role_user")
                                     .from(UserRole::Table, UserRole::RoleId)
                                     .to(Role::Table, Role::Id)
                                     .on_delete(ForeignKeyAction::Cascade))
                                 .to_owned(),
        )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserRole::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum UserRole {
    Table,
    Id,
    UserId,
    RoleId,
}
