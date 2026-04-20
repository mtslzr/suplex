use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Promotions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Promotions::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Promotions::CagematchId)
                            .integer()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(Promotions::Nickname)
                            .text()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Promotions::CanonicalName).text())
                    .col(ColumnDef::new(Promotions::Abbreviation).text())
                    .col(ColumnDef::new(Promotions::Country).text())
                    .col(ColumnDef::new(Promotions::LogoUrl).text())
                    .col(ColumnDef::new(Promotions::CagematchUrl).text())
                    .col(ColumnDef::new(Promotions::AccentColor).text())
                    .col(
                        ColumnDef::new(Promotions::Enabled)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(ColumnDef::new(Promotions::LastSyncedAt).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(Promotions::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Promotions::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Promotions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Promotions {
    Table,
    Id,
    CagematchId,
    Nickname,
    CanonicalName,
    Abbreviation,
    Country,
    LogoUrl,
    CagematchUrl,
    AccentColor,
    Enabled,
    LastSyncedAt,
    CreatedAt,
    UpdatedAt,
}
