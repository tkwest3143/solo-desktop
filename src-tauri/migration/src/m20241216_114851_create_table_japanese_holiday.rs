use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;
#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(JapaneseHoliday::Table)
          .if_not_exists()
          .col(pk_auto(JapaneseHoliday::Id))
          .col(date_time(JapaneseHoliday::Day).not_null())
          .col(string(JapaneseHoliday::Subject).not_null())
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager.drop_table(Table::drop().table(JapaneseHoliday::Table).to_owned()).await
  }
}
#[derive(DeriveIden)]
pub enum JapaneseHoliday {
  Table,
  Id,
  Day,
  Subject,
}
