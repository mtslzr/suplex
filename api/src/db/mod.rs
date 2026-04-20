use sea_orm::{ConnectOptions, Database, DatabaseConnection};

/// Connect to PostgreSQL and run pending migrations.
pub async fn connect(database_url: &str) -> DatabaseConnection {
    let mut opts = ConnectOptions::new(database_url);
    opts.max_connections(10)
        .min_connections(2)
        .sqlx_logging(false);

    let db = Database::connect(opts)
        .await
        .expect("Failed to connect to database");

    use migration::{Migrator, MigratorTrait};
    Migrator::up(&db, None)
        .await
        .expect("Failed to run migrations");

    tracing::info!("Database connected and migrations applied");
    db
}
