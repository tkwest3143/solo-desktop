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
          .table(WorkSettings::Table)
          .if_not_exists()
          .col(pk_auto(WorkSettings::Id))
          .col(string(WorkSettings::Title).not_null())
          .col(date_time(WorkSettings::Start).not_null())
          .col(date_time(WorkSettings::End).not_null())
          .col(date_time(WorkSettings::RestStart).not_null())
          .col(date_time(WorkSettings::RestEnd).not_null())
          .col(string_null(WorkSettings::Memo))
          .col(integer(WorkSettings::WorkingUnit).not_null())
          .col(integer(WorkSettings::UserId).not_null())
          .col(date_time(WorkSettings::CreatedAt).not_null())
          .col(date_time(WorkSettings::UpdatedAt).not_null())
          .foreign_key(ForeignKey::create().name("FK_users_work_settings_id").from(Users::Table, WorkSettings::UserId).to(WorkSettings::Table, Users::Id))
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager.drop_table(Table::drop().table(WorkSettings::Table).to_owned()).await
  }
}
#[derive(DeriveIden)]
pub enum WorkSettings {
  Table,
  Id,
  Title,
  Start,
  End,
  RestStart,
  RestEnd,
  Memo,
  WorkingUnit,
  CreatedAt,
  UpdatedAt,
  UserId,
}
