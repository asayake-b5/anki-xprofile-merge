use sqlx::{migrate::MigrateDatabase, FromRow, Sqlite, SqlitePool};

pub struct AnkiDatabase(pub SqlitePool);

impl AnkiDatabase {
    pub async fn new(url: &str) -> Self {
        let db = SqlitePool::connect(url).await.unwrap();
        Self(db)
    }
}
