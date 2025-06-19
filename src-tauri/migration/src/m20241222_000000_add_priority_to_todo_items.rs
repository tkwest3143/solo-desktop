use sea_orm_migration::{prelude::*, schema::*};

use crate::m20241216_114851_create_table_todo_items::TodoItems;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .alter_table(
        Table::alter()
          .table(TodoItems::Table)
          .add_column(string_null(TodoItems::Priority))
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .alter_table(
        Table::alter()
          .table(TodoItems::Table)
          .drop_column(TodoItems::Priority)
          .to_owned(),
      )
      .await
  }
}