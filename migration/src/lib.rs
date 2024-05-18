pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20240401_085218_create_exception_table;
mod m20240407_054555_create_post_table;
mod m20240407_054844_create_category_table;
mod m20240407_055151_create_post_tags_table;
mod m20240407_055250_create_tag_table;
mod m20240411_032202_create_product_table;
mod m20240411_032637_create_product_category_table;
mod m20240411_034848_create_product_sku_table;
mod m20240411_053356_create_product_sku_name_table;
mod m20240411_053401_create_product_sku_value_table;
mod m20240411_053408_create_product_sku_name_value_table;
mod m20240502_062735_create_permission_table;
mod m20240502_062809_create_role_table;
mod m20240514_061611_create_role_permission_table;

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
            Box::new(m20240411_032202_create_product_table::Migration),
            Box::new(m20240411_032637_create_product_category_table::Migration),
            Box::new(m20240411_034848_create_product_sku_table::Migration),
            Box::new(m20240411_053356_create_product_sku_name_table::Migration),
            Box::new(m20240411_053401_create_product_sku_value_table::Migration),
            Box::new(m20240411_053408_create_product_sku_name_value_table::Migration),
            Box::new(m20240502_062735_create_permission_table::Migration),
            Box::new(m20240502_062809_create_role_table::Migration),
            Box::new(m20240514_061611_create_role_permission_table::Migration),
        ]
    }
}
