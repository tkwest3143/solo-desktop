use sea_orm_migration::{prelude::*, schema::*};

use crate::m20241216_114851_create_table_todo_items::TodoItems;
use crate::m20241216_114851_create_table_users::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .alter_table(
        Table::alter()
          .table(TodoItems::Table)
          .add_column(integer(TodoItems::UserId).not_null().default(1))
          .to_owned(),
      )
      .await?;

    // Add foreign key constraint
    manager
      .create_foreign_key(
        ForeignKey::create()
          .name("FK_users_todo_items_user_id")
          .from(TodoItems::Table, TodoItems::UserId)
          .to(Users::Table, Users::Id)
          .on_delete(ForeignKeyAction::Cascade)
          .on_update(ForeignKeyAction::Cascade)
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    // Drop foreign key constraint
    manager
      .drop_foreign_key(
        ForeignKey::drop()
          .name("FK_users_todo_items_user_id")
          .table(TodoItems::Table)
          .to_owned(),
      )
      .await?;

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