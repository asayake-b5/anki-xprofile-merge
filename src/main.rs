pub mod parser;

use sqlx::{migrate::MigrateDatabase, prelude::FromRow, Sqlite, SqlitePool};

use crate::parser::Parser;
const DB_URL: &str = "sqlite://sqlite.db";
const DB_URL2: &str = "sqlite:///home/bv/.local/share/Anki2/SentenceBank/collection.anki2";

#[derive(Clone, FromRow, Debug)]
struct AnkiNote {
    id: i64,
    flds: String,
}

#[tokio::main]
async fn main() {
    let mut parser = Parser::new();

    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }
    let db = SqlitePool::connect(DB_URL).await.unwrap();
    let db_anki = SqlitePool::connect(DB_URL2).await.unwrap();
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let migrations = std::path::Path::new(&crate_dir).join("./migrations");
    let migration_results = sqlx::migrate::Migrator::new(migrations)
        .await
        .unwrap()
        .run(&db)
        .await;
    match migration_results {
        Ok(_) => println!("Migration success"),
        Err(error) => {
            panic!("error: {}", error);
        }
    }
    println!("migration: {:?}", migration_results);
    let notes_results = sqlx::query_as::<_, AnkiNote>("SELECT id, flds FROM notes")
        .fetch_all(&db_anki)
        .await
        .unwrap();

    let mut tx = db.begin().await.unwrap();
    //TODO rayon?
    for note in notes_results {
        let thing = note
            .flds
            .split(0x1f as char)
            .map(|e| e.to_string())
            .collect::<Vec<String>>();

        let _ = sqlx::query(
            "INSERT OR IGNORE INTO notes (nid, audio, image, sentence, morphenes) VALUES (?, ?, ?, ?, ?);",
        )
        .bind(note.id)
        .bind(thing[0].clone())
        .bind(thing[1].clone())
        .bind(thing[2].clone())
        .bind(
            parser
                .parse(&thing[2])
                .unwrap_or_default()
                .join(",")
                .to_string(),
        )
        // .bind(note.id)
        .execute(&mut tx)
        .await
        .unwrap();
    }
    tx.commit().await.unwrap();
}
