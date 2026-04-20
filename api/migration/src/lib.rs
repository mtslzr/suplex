pub use sea_orm_migration::prelude::*;

mod m20260420_000001_create_promotions;
mod m20260420_000002_create_sync_log;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260420_000001_create_promotions::Migration),
            Box::new(m20260420_000002_create_sync_log::Migration),
        ]
    }
}
