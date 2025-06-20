use sea_orm_migration::{prelude::*, schema::*};

use crate::{m20241216_114851_create_table_todo_categories::TodoCategories, m20241216_114851_create_table_users::Users};
#[derive(DeriveMigrationName)]
pub struct Migration;
#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(TodoItems::Table)
          .if_not_exists()
          .col(pk_auto(TodoItems::Id))
          .col(integer_null(TodoItems::CategoryId))
          .col(string_null(TodoItems::Title))
          .col(string_null(TodoItems::Content))
          .col(string_null(TodoItems::Link))
          .col(string_null(TodoItems::Color))
          .col(string_null(TodoItems::Priority))
          .col(date_time(TodoItems::DueDate).not_null())
          .col(integer(TodoItems::UserId).not_null())
          .col(string_null(TodoItems::Status).default("incomplete"))
          .col(date_time(TodoItems::CreatedAt).not_null())
          .col(date_time(TodoItems::UpdatedAt).not_null())
          .foreign_key(
            ForeignKey::create()
              .name("FK_todo_categories_todo_items_id")
              .from(TodoItems::Table, TodoItems::CategoryId)
              .to(TodoCategories::Table, TodoCategories::Id)
              .on_delete(ForeignKeyAction::Cascade)
              .on_update(ForeignKeyAction::Cascade),
          )
          .foreign_key(ForeignKey::create().name("FK_users_todo_items_id").from(TodoItems::Table, TodoItems::UserId).to(Users::Table, Users::Id))
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager.drop_table(Table::drop().table(TodoItems::Table).to_owned()).await
  }
}
#[derive(DeriveIden)]
pub enum TodoItems {
  Table,
  Id,
  Title,
  Content,
  Link,
  Color,
  Priority,
  DueDate,
  CreatedAt,
  UpdatedAt,
  CategoryId,
  Status,
  UserId,
}
