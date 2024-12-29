

use sea_orm_migration::{prelude::*, schema::*};
#[derive(DeriveMigrationName)]
pub struct Migration;
#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(Users::Table)
          .if_not_exists()
          .col(pk_auto(Users::Id))
          .col(string(Users::Name).not_null())
          .col(string_null(Users::Email))
          .col(integer_null(Users::DefaultWorkSettingId))
          .col(date_time_null(Users::LastLoginTime))
          .col(date_time(Users::CreatedAt).not_null())
          .col(date_time(Users::UpdatedAt).not_null())
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager.drop_table(Table::drop().table(Users::Table).to_owned()).await
  }
}

#[derive(DeriveIden)]
pub enum Users {
  Table,
  Id,
  Name,
  Email,
  LastLoginTime,
  DefaultWorkSettingId,
  CreatedAt,
  UpdatedAt,
}
