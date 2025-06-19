pub use sea_orm_migration::prelude::*;
mod m20241216_114851_create_table_documents;
mod m20241216_114851_create_table_japanese_holiday;
mod m20241216_114851_create_table_todo_categories;
mod m20241216_114851_create_table_todo_items;
mod m20241216_114851_create_table_users;
mod m20241216_114851_create_table_work_settings;
mod m20241216_114851_create_table_work_times;
mod m20241217_000000_add_status_column_to_todo_items;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
  fn migrations() -> Vec<Box<dyn MigrationTrait>> {
    vec![
      Box::new(m20241216_114851_create_table_users::Migration),
      Box::new(m20241216_114851_create_table_work_times::Migration),
      Box::new(m20241216_114851_create_table_work_settings::Migration),
      Box::new(m20241216_114851_create_table_documents::Migration),
      Box::new(m20241216_114851_create_table_japanese_holiday::Migration),
      Box::new(m20241216_114851_create_table_todo_items::Migration),
      Box::new(m20241216_114851_create_table_todo_categories::Migration),
      Box::new(m20241217_000000_add_status_column_to_todo_items::Migration),
    ]
  }
}
