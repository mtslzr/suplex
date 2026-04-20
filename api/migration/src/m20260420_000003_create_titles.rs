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
                    .table(Titles::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Titles::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Titles::PromotionId).uuid().not_null())
                    .col(ColumnDef::new(Titles::CagematchId).integer().not_null())
                    .col(ColumnDef::new(Titles::Name).text().not_null())
                    .col(
                        ColumnDef::new(Titles::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(ColumnDef::new(Titles::CagematchUrl).text())
                    .col(ColumnDef::new(Titles::CurrentChampionDisplay).text())
                    .col(ColumnDef::new(Titles::CurrentChampionCagematchId).integer())
                    .col(ColumnDef::new(Titles::CurrentSinceDate).date())
                    .col(ColumnDef::new(Titles::LastSyncedAt).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(Titles::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Titles::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Titles::Table, Titles::PromotionId)
                            .to(Promotions::Table, Promotions::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name("titles_promotion_cagematch_unique")
                            .table(Titles::Table)
                            .col(Titles::PromotionId)
                            .col(Titles::CagematchId)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Titles::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Titles {
    Table,
    Id,
    PromotionId,
    CagematchId,
    Name,
    IsActive,
    CagematchUrl,
    CurrentChampionDisplay,
    CurrentChampionCagematchId,
    CurrentSinceDate,
    LastSyncedAt,
    CreatedAt,
    UpdatedAt,
}
