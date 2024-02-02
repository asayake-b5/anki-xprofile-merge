pub mod ankiconnect;
pub mod ankidb;
pub mod bank_fields;
pub mod morphdb;
pub mod mydb;
pub mod parser;

use std::fmt::Display;

use anki_xprofile_merge::{clean_sentence, Match};
use ankiconnect::AnkiConnect;
use ankidb::AnkiDeck;
use chrono::DateTime;
use clap::Parser;
use inquire::MultiSelect;
use mydb::MyDeck;
use scraper::Selector;

use crate::{
    ankiconnect::Note,
    ankidb::AnkiDatabase,
    morphdb::MorphDatabase,
    mydb::{DBNote, MyDatabase},
    parser::JParser,
};
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
    let db_morphs = MorphDatabase::new(main_profile_url.as_os_str().to_str().unwrap()).await;

    let known_morphs = db_morphs.list_morphs().await;

    db.migrate().await;

    let anki_decks = db_anki.list_decks().await;
    let my_decks = db.ids().await;
    let options = gen_deck_list(&anki_decks, &my_decks);
    db.update_morphs(&known_morphs).await;

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
        .execute(&mut *tx)
        .await
        .unwrap();
    }
    tx.commit().await.unwrap();

    let ankiconnect = AnkiConnect::new("localhost", 8765);
    // let model = inquire::Select::new("Select the model to act upon:", ankiconnect.list_models())
    //     .prompt()
    //     .unwrap();

    // let decks: Vec<i64> = inquire::MultiSelect::new(
    //     "Select the decks to consider (none for all):",
    //     ankiconnect.list_decks(),
    // )
    // .prompt()
    // .unwrap()
    // .iter()
    // .map(|d| d.id)
    // .collect();
    // dbg!(&decks);

    // let notes = ankiconnect.find_notes(&model, &decks);
    // let fields = ankiconnect.list_fields(&notes);
    // let word = inquire::Select::new(
    //     "Select the field where your target is, ideally without furigana",
    //     fields.clone(),
    // )
    // .prompt()
    // .unwrap();

    // let sentence = inquire::Select::new(
    //     "Select the field where your sentence field is, if you have the choice, pick the furiganaless one",
    //     fields.clone(),
    // )
    // .prompt()
    // .unwrap();

    // let sentence_audio = inquire::Select::new(
    //     "Select the field where your sentence audio is",
    //     fields.clone(),
    // )
    // .prompt()
    // .unwrap();

    // dbg!(word_reading);
    // dbg!(sentence);
    // dbg!(sentence_audio);
    let model = "JP Mining Note";
    let decks: Vec<i64> = vec![1699524597138];
    //TODO change to word in the selector?
    let word = "Word";
    //TODO change to sentencereading, and use our own parsing?
    let sentence = "Sentence";
    let sentence_audio = "SentenceAudio";
    let notes = ankiconnect.find_notes(model, &decks);
    // dbg!(notes);
    let notes = ankiconnect.notes_redux(&notes, word, sentence, sentence_audio);
    //TODO also filter sentenceless things, later down the pipeline?
    let notes = notes
        .into_iter()
        .filter(|note| note.sentence_audio.is_empty() && !note.sentence.is_empty())
        .collect::<Vec<Note>>();
    let mut not_found = 0;
    let mut lucky_finds: Vec<Match> = Vec::with_capacity(100);
    for note in notes {
        // println!("{}", note.sentence);
        // println!("{}", note.word);
        //TODO turn word_reading into 1f1e

        if let Ok(tokenized) = parser.parse(&clean_sentence(&note.sentence)) {
            if let Some(lucky) = db.find_lucky(&tokenized).await {
                lucky_finds.push(Match {
                    id: lucky.nid,
                    sentence: lucky.sentence,
                    audio: lucky.audio,
                    image: lucky.image,
                    morphenes: lucky.morphenes,
                    og_clean_sentence: clean_sentence(&note.sentence),
                    og_sentence: note.sentence,
                    og_morphnes: tokenized,
                    og_word: note.word,
                });
                continue;
            }
            continue;
            // println!("a",);
            // let mut tokens = tokenized.split('\u{1f}');
            // let token = tokens.find(|e| e.contains(&note.word));
            let tokenized_word = parser.parse(&note.word).unwrap();
            let token = tokenized.contains(&tokenized_word);
            if !token {
                let p = parser.parse(&clean_sentence(&note.sentence));
                let p2 = parser.parse(&note.word);
                // println!("--",);
                // println!("{}", note.sentence);
                // println!("{:#?}", p);
                // println!("{:#?}", p2);
                // println!("--",);
                //TODO figure out what to do with these later
            } else {
                // println!("{:#?}", &parser.parse(&note.word));
                // TODO transaction this and find matches for all instead?
                let matches = db.find_like_morphene(&tokenized_word).await;
                if !matches.is_empty() {
                    // println!("Found {1} for {0}", tokenized_word, matches.len());
                    // println!("{:#?}", matches);
                } else {
                    println!("notFound {1} for {0}", tokenized_word, matches.len());
                    not_found += 1
                }
            }
            // println!("{token:#?}",);
            // println!("{tokenized}",);
            // for token in tokenized.split('\u{1f}') {
            //     println!("{token}",);
            // }
            //TODO handle find lucky even if rare
        }
    }
    println!("{not_found}",);
    // println!("{lucky_finds:#?}",);
    if !lucky_finds.is_empty() {
        let decks: Vec<(String, String)> = inquire::MultiSelect::new(
            "Near-perfect matches found, which ones should we update?",
            lucky_finds,
        )
        .prompt()
        .unwrap()
        .into_iter()
        .map(|d| (d.audio, d.image))
        .collect();
        dbg!(&decks);
        dbg!(decks.len());
    }

    // let tokenized_word = String::from("撤廃\u{1e}てっぱい");
    // // let tokenized_word = String::from("\u{1e}");
    // let matches = db.find_like_morphene(&tokenized_word).await;
    // println!("{:#?}", &matches);

    // ankiconnect.test_update_note_fields()
}
