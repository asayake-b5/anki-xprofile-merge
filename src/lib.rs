use regex::Regex;
use scraper::Selector;

pub fn clean_sentence(input: &str) -> String {
    let fragment = scraper::Html::parse_fragment(input);
    //TODO not ruby somehow in the selector, to pick everything butruby?
    let selector = Selector::parse("*").unwrap();
    let contents = fragment.select(&selector).next().unwrap();
    contents.text().collect::<String>()
}
