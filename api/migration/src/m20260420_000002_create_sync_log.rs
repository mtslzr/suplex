use sea_orm_migration::prelude::*;

use crate::m20260420_000001_create_promotions::Promotions;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SyncLog::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(SyncLog::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(SyncLog::PromotionId).uuid())
                    .col(ColumnDef::new(SyncLog::Scope).text().not_null())
                    .col(
                        ColumnDef::new(SyncLog::StartedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(SyncLog::CompletedAt).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(SyncLog::Status)
                            .text()
                            .not_null()
                            .default("running"),
                    )
                    .col(
                        ColumnDef::new(SyncLog::ItemsCreated)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(SyncLog::ItemsUpdated)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(SyncLog::Error).text())
                    .foreign_key(
                        ForeignKey::create()
                            .from(SyncLog::Table, SyncLog::PromotionId)
                            .to(Promotions::Table, Promotions::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SyncLog::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum SyncLog {
    Table,
    Id,
    PromotionId,
    Scope,
    StartedAt,
    CompletedAt,
    Status,
    ItemsCreated,
    ItemsUpdated,
    Error,
}
