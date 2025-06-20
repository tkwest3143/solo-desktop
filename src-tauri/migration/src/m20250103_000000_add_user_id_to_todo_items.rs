use sea_orm_migration::{prelude::*, schema::*};

use crate::m20241216_114851_create_table_todo_items::TodoItems;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    // For SQLite, we need to add the column without the foreign key constraint
    // since SQLite doesn't support modifying foreign key constraints on existing tables
    manager
      .alter_table(
        Table::alter()
          .table(TodoItems::Table)
          .add_column(integer(TodoItems::UserId).not_null().default(1))
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    // Drop the column
    manager
      .alter_table(
        Table::alter()
          .table(TodoItems::Table)
          .drop_column(TodoItems::UserId)
          .to_owned(),
      )
      .await
  }
}