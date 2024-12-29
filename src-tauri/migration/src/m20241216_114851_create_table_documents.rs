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
          .table(Documents::Table)
          .if_not_exists()
          .col(pk_auto(Documents::Id))
          .col(string(Documents::Title).not_null())
          .col(string(Documents::DocumentId).not_null())
          .col(integer(Documents::UserId).not_null())
          .col(date_time(Documents::CreatedAt).not_null())
          .col(date_time(Documents::UpdatedAt).not_null())
          .foreign_key(ForeignKey::create().name("FK_users_documents_id").from(Users::Table, Documents::UserId).to(Documents::Table, Users::Id))
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager.drop_table(Table::drop().table(Documents::Table).to_owned()).await
  }
}
#[derive(DeriveIden)]
pub enum Documents {
  Table,
  Id,
  Title,
  DocumentId,
  CreatedAt,
  UpdatedAt,
  UserId,
}
