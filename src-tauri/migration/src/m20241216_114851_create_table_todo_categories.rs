use sea_orm_migration::{prelude::*, schema::*};

use crate::m20241216_114851_create_table_users::Users;
#[derive(DeriveMigrationName)]
pub struct Migration;
#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(TodoCategories::Table)
          .if_not_exists()
          .col(pk_auto(TodoCategories::Id))
          .col(string(TodoCategories::Name))
          .col(string_null(TodoCategories::Memo))
          .col(integer(TodoCategories::UserId))
          .col(date_time(TodoCategories::CreatedAt).not_null())
          .col(date_time(TodoCategories::UpdatedAt).not_null())
          .col(string_null(TodoCategories::Color))
          .foreign_key(ForeignKey::create().name("FK_users_todo_categories_id").from(Users::Table, TodoCategories::UserId).to(TodoCategories::Table, Users::Id))
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager.drop_table(Table::drop().table(TodoCategories::Table).to_owned()).await
  }
}
#[derive(DeriveIden)]
pub enum TodoCategories {
  Table,
  Id,
  Name,
  Memo,
  Color,
  CreatedAt,
  UpdatedAt,
  UserId,
}
