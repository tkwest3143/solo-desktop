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
          .table(WorkTimes::Table)
          .if_not_exists()
          .col(pk_auto(WorkTimes::Id))
          .col(string(WorkTimes::TargetDay).not_null())
          .col(date_time_null(WorkTimes::Start))
          .col(date_time_null(WorkTimes::End))
          .col(date_time_null(WorkTimes::RestStart))
          .col(date_time_null(WorkTimes::RestEnd))
          .col(string_null(WorkTimes::Memo))
          .col(integer(WorkTimes::UserId))
          .col(date_time(WorkTimes::CreatedAt).not_null())
          .col(date_time(WorkTimes::UpdatedAt).not_null())
          .foreign_key(ForeignKey::create().name("FK_work_times_user_id").from(WorkTimes::Table, WorkTimes::UserId).to(Users::Table, Users::Id))
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager.drop_table(Table::drop().table(WorkTimes::Table).to_owned()).await
  }
}
#[derive(DeriveIden)]
pub enum WorkTimes {
  Table,
  Id,
  TargetDay,
  Start,
  End,
  RestStart,
  RestEnd,
  Memo,
  CreatedAt,
  UpdatedAt,
  UserId,
}
