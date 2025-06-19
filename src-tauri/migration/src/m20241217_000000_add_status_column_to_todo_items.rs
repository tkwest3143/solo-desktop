use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .alter_table(
        Table::alter()
          .table(TodoItems::Table)
          .add_column(string_null(TodoItems::Status).default("incomplete"))
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .alter_table(
        Table::alter()
          .table(TodoItems::Table)
          .drop_column(TodoItems::Status)
          .to_owned(),
      )
      .await
  }
}

#[derive(DeriveIden)]
enum TodoItems {
  Table,
  Status,
}