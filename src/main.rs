pub mod ankidb;
pub mod mydb;
pub mod parser;

use std::fmt::Display;

use ankidb::AnkiDeck;
use chrono::DateTime;
use mydb::MyDeck;
use sqlx::prelude::FromRow;

use crate::{ankidb::AnkiDatabase, mydb::MyDatabase, parser::Parser};
const DB_URL: &str = "sqlite://sqlite.db";
//TODO [cfg(windows) linux etc, parameter/inquire about the profile]
const DB_URL2: &str = "sqlite:///home/bv/.local/share/Anki2/SentenceBank/collection.anki2";

#[derive(Debug)]
pub struct DeckList {
    id: i64,
    name: String,
    card_count: i64,
    timestamp: Option<i64>,
}

impl Display for DeckList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //TODO trim the deck name here, keep only the subdeck name
        let timestamp = if let Some(timestamp) = self.timestamp {
            format!(
                "- Inserted {}",
                DateTime::from_timestamp(timestamp, 0).unwrap()
            )
        } else {
            String::from("")
        };
        write!(f, "{}({} cards) {}", self.name, self.card_count, timestamp)
    }
}

pub fn gen_deck_list(anki: &[AnkiDeck], mine: &[MyDeck]) -> Vec<DeckList> {
    let mut r = Vec::with_capacity(1000);
    for deck in anki {
        let mut timestamp = None;
        if let Some(card) = mine.iter().find(|e| e.id == deck.id) {
            timestamp = Some(card.timestamp);
        }
        r.push(DeckList {
            id: deck.id,
            name: deck.name.clone(),
            card_count: deck.card_count,
            timestamp,
        })
    }
    r
}

//TODO n+1ing sentences, aka pull all the known words

#[tokio::main]
async fn main() {
    let mut parser = Parser::new();

    let mut db = MyDatabase::new(DB_URL).await;
    let db_anki = AnkiDatabase::new(DB_URL2).await;
    db.migrate().await;

    let anki_decks = db_anki.list_decks().await;
    let my_decks = db.ids().await;
    let options = gen_deck_list(&anki_decks, &my_decks);

    //TODO add deckid to mydb, then do some formatting if the deck was already added
    // TODO create new struct, to put our timestamp in
    // mydeck table: deckid, timestamp
    inquire::MultiSelect::new("Select the decks to add:", options)
        .prompt()
        .unwrap();

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
