pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20240401_085218_create_exception_table;
mod m20240407_054555_create_post_table;
mod m20240407_054844_create_category_table;
mod m20240407_055151_create_post_tags_table;
mod m20240407_055250_create_tag_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20240401_085218_create_exception_table::Migration),
            Box::new(m20240407_054555_create_post_table::Migration),
            Box::new(m20240407_054844_create_category_table::Migration),
            Box::new(m20240407_055151_create_post_tags_table::Migration),
            Box::new(m20240407_055250_create_tag_table::Migration),
        ]
    }
}
