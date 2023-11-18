pub mod ankidb;
pub mod mydb;
pub mod parser;

use sqlx::prelude::FromRow;

use crate::{ankidb::AnkiDatabase, mydb::MyDatabase, parser::Parser};
const DB_URL: &str = "sqlite://sqlite.db";
//TODO [cfg(windows) linux etc, parameter/inquire about the profile]
const DB_URL2: &str = "sqlite:///home/bv/.local/share/Anki2/SentenceBank/collection.anki2";

#[tokio::main]
async fn main() {
    let mut parser = Parser::new();

    let mut db = MyDatabase::new(DB_URL).await;
    let db_anki = AnkiDatabase::new(DB_URL2).await;
    db.migrate().await;

    // let mut tx = db.0.begin().await.unwrap();
    // //TODO rayon?
    // for note in notes_results {
    //     let thing = note
    //         .flds
    //         .split(0x1f as char)
    //         .map(|e| e.to_string())
    //         .collect::<Vec<String>>();

    //     let _ = sqlx::query(
    //         "INSERT OR IGNORE INTO notes (nid, audio, image, sentence, morphenes) VALUES (?, ?, ?, ?, ?);",
    //     )
    //     .bind(note.id)
    //     .bind(thing[0].clone())
    //     .bind(thing[1].clone())
    //     .bind(thing[2].clone())
    //     .bind(
    //         parser
    //             .parse(&thing[2])
    //             .unwrap_or_default()
    //             .join(",")
    //             .to_string(),
    //     )
    //     // .bind(note.id)
    //     .execute(&mut tx)
    //     .await
    //     .unwrap();
    // }
    // tx.commit().await.unwrap();
}
