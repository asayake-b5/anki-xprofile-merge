pub mod ankidb;
pub mod bank_fields;
pub mod mydb;
pub mod parser;

use std::fmt::Display;

use ankidb::AnkiDeck;
use chrono::DateTime;
use clap::Parser;
use mydb::MyDeck;

use crate::{ankidb::AnkiDatabase, mydb::MyDatabase, parser::JParser};
const DB_URL: &str = "sqlite://sqlite.db";

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

#[derive(Parser, Debug)]
// #[command(author, version, about, long_about = None)]
struct Cli {
    /// The profile you normally use, with all your morphs calculated etc, where your mining deck is
    main_profile_name: String,
    /// Your sentence bank profile
    sentence_bank_profile_name: String,
}

//TODO n+1ing sentences, aka pull all the known words

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    dbg!(&cli);

    let mut main_profile_url = dirs::data_dir().unwrap();
    main_profile_url.push("Anki2");
    main_profile_url.push(&cli.main_profile_name);
    main_profile_url.push("ankimorphs.db");
    if !main_profile_url.as_path().exists() {
        eprintln!("Path {} does not exist, did you provide the right profile name, and are you using anki-morphs?", main_profile_url.display());
        return;
    }

    let mut sentence_bank_url = dirs::data_dir().unwrap();
    sentence_bank_url.push("Anki2");
    sentence_bank_url.push(&cli.sentence_bank_profile_name);
    sentence_bank_url.push("collection.anki2");
    if !sentence_bank_url.as_path().exists() {
        eprintln!(
            "Path {} does not exist, did you provide the right profile name?",
            sentence_bank_url.display()
        );
        return;
    }

    // dbg!(sentence_bank_url);

    // dbg!(machine_kind);

    let mut parser = JParser::new();

    let db = MyDatabase::new(DB_URL).await;
    let db_anki = AnkiDatabase::new(sentence_bank_url.as_os_str().to_str().unwrap()).await;
    db.migrate().await;

    let anki_decks = db_anki.list_decks().await;
    let my_decks = db.ids().await;
    let options = gen_deck_list(&anki_decks, &my_decks);

    let decks: Vec<i64> = inquire::MultiSelect::new("Select the decks to add:", options)
        .prompt()
        .unwrap()
        .iter()
        .map(|d| d.id)
        .collect();
    db.add_ids(&decks).await;

    let notes = db_anki.list_notes(&decks).await;
    // let bank_fields = bank_fields::extract_slice(&notes);
    // dbg!(parser.parse(&bank_fields[0].sentence));

    let mut tx = db.0.begin().await.unwrap();
    //TODO rayon?
    for note in notes {
        let bank_fields = bank_fields::extract_note(&note);
        let parsed = parser.parse(&bank_fields.sentence).unwrap_or_default();

        let _ = sqlx::query(
            "insert or ignore into notes (nid, audio, image, sentence, morphenes) values (?, ?, ?, ?, ?);",
        )
        .bind(note.id)
        .bind(bank_fields.audio)
        .bind(bank_fields.image)
        .bind(bank_fields.sentence)
        .bind(
            parsed
        )
        // .bind(note.id)
        .execute(&mut tx)
        .await
        .unwrap();
    }
    tx.commit().await.unwrap();
}
