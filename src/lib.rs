use scraper::Selector;
use std::fmt::Display;

pub fn clean_sentence(input: &str) -> String {
    let fragment = scraper::Html::parse_fragment(input);
    //TODO not ruby somehow in the selector, to pick everything butruby?
    let selector = Selector::parse("*").unwrap();
    let contents = fragment.select(&selector).next().unwrap();
    contents.text().collect::<String>()
}

#[derive(Debug)]
pub struct Match {
    pub id: i64, //TODO remove?
    pub sentence: String,
    pub audio: String,
    pub image: String,
    pub morphenes: String,   //TODO remove?
    pub og_sentence: String, //TODO remove?
    pub og_clean_sentence: String,
    pub og_word: String,
    pub og_morphnes: String,
    // pub origin // TODO do this?
}

impl Match {
    pub fn similar_sentences(&self) -> bool {
        self.og_clean_sentence == self.sentence
            || self.og_clean_sentence.contains(&self.sentence)
            || self.sentence.contains(&self.og_clean_sentence)
    }
}

impl Display for Match {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sentence_part = if self.similar_sentences() {
            self.sentence.clone()
        } else {
            format!("{} -> {}", self.og_clean_sentence, self.sentence)
        };
        write!(f, "[{}]: {}", self.og_word, sentence_part)
    }
}
