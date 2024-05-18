use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Permission::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Permission::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Permission::Name).string().not_null())
                    .col(ColumnDef::new(Permission::Description).string().default(""))
                    .col(ColumnDef::new(Permission::CreateTime).date())
                    .col(ColumnDef::new(Permission::PermissionUrl).string())
                    .col(ColumnDef::new(Permission::Type).string())
                    .col(ColumnDef::new(Permission::Method).string())
                    .col(ColumnDef::new(Permission::Group).string())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Permission::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Permission {
    Table,
    Id,
    //权限名称
    Name,
    //介绍
    Description,
    //创建时间
    CreateTime,
    //访问URL
    PermissionUrl,
    //类型,URL,PAGE, url:接口URL,page:页面
    Type,
    //URL请求类型的函数,比如GET,POST这种
    Method,
    //分组,标识
    Group,
}
