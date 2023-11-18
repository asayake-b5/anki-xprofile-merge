use sqlx::{migrate::MigrateDatabase, FromRow, Sqlite, SqlitePool};

pub struct MyDatabase(pub SqlitePool);

#[derive(Clone, FromRow, Debug)]
pub struct MyDeck {
    pub id: i64,
    pub timestamp: i64,
}

impl MyDatabase {
    pub async fn new(url: &str) -> Self {
        if !Sqlite::database_exists(url).await.unwrap_or(false) {
            println!("Creating database {}", url);
            match Sqlite::create_database(url).await {
                Ok(_) => println!("Create db success"),
                Err(error) => panic!("error: {}", error),
            }
        } else {
            println!("Database already exists");
        }
        let db = SqlitePool::connect(url).await.unwrap();
        Self(db)
    }

    pub async fn add_ids(&mut self, deck_ids: &[i64]) {
        let mut tx = self.0.begin().await.unwrap();
        for id in deck_ids {
            sqlx::query(
                "INSERT INTO decks (id) VALUES (?) ON CONFLICT(id) DO UPDATE SET timestamp = CURRENT_TIMESTAMP"
            )
            .bind(id)
            .execute(&mut tx)
            .await
            .unwrap();
        }
        tx.commit().await.unwrap();
    }

    pub async fn ids(&self) -> Vec<MyDeck> {
        sqlx::query_as::<_, MyDeck>(
            "SELECT id, CAST(strftime(\"%s\", timestamp) as INTEGER) as timestamp from decks",
        )
        .fetch_all(&self.0)
        .await
        .unwrap()
    }

    pub async fn migrate(&self) {
        //TODO migrate from insert_str!() ?
        let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let migrations = std::path::Path::new(&crate_dir).join("./migrations");
        let migration_results = sqlx::migrate::Migrator::new(migrations)
            .await
            .unwrap()
            .run(&self.0)
            .await;
        match migration_results {
            Ok(_) => println!("Migration success"),
            Err(error) => {
                panic!("error: {}", error);
            }
        }
        println!("migration: {:?}", migration_results);
    }
}
