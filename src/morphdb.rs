use sqlx::{FromRow, SqlitePool};

pub struct MorphDatabase(pub SqlitePool);

#[derive(Clone, FromRow, Debug)]
pub struct Morph {
    pub lemma: String,
    pub inflection: String,
    pub highest_learning_interval: i64,
}

impl MorphDatabase {
    pub async fn new(url: &str) -> Self {
        let db = SqlitePool::connect(url).await.unwrap();
        Self(db)
    }

    pub async fn list_morphs(&self) -> Vec<Morph> {
        sqlx::query_as::<_, Morph>("SELECT Morphs.* FROM Morphs")
            .fetch_all(&self.0)
            .await
            .unwrap()
    }
}
