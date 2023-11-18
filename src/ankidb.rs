use sqlx::{migrate::MigrateDatabase, FromRow, Sqlite, SqlitePool};

pub struct AnkiDatabase(pub SqlitePool);

#[derive(Clone, FromRow, Debug)]
pub struct AnkiDeck {
    id: i64,
    name: String,
    card_count: i64,
}

#[derive(Clone, FromRow, Debug)]
pub struct AnkiNote {
    id: i64,
    flds: String,
}

impl AnkiDatabase {
    pub async fn new(url: &str) -> Self {
        let db = SqlitePool::connect(url).await.unwrap();
        Self(db)
    }

    pub async fn list_decks(&self) -> Vec<AnkiDeck> {
        sqlx::query_as::<_, AnkiDeck>("SELECT decks.id, decks.name, COUNT(*) as card_count from decks INNER JOIN cards on cards.did = decks.id GROUP BY decks.id")
            .fetch_all(&self.0)
            .await
            .unwrap()
    }

    pub async fn list_notes(&self, deck_ids: &[i64]) -> Vec<AnkiNote> {
        let string = deck_ids
            .iter()
            .map(|e| e.to_string() + ",")
            .collect::<String>();

        if string.is_empty() {
            return Vec::new();
        }

        sqlx::query_as::<_, AnkiNote>(&format!(
            "SELECT notes.id, notes.flds FROM notes INNER JOIN cards on cards.nid = notes.id
WHERE cards.did IN ({})",
            &string[0..&string.len() - 1]
        ))
        .fetch_all(&self.0)
        .await
        .unwrap()
    }
}
