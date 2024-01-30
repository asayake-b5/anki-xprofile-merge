use notmecab::{Blob, Cache, Dict, LexerToken, TokenizeError};
use wana_kana::ConvertJapanese;

pub struct JParser {
    dict: Dict,
    cache: Cache,
}

impl JParser {
    pub fn new() -> Self {
        // you need to acquire a mecab dictionary and place these files here manually
        let sysdic = Blob::open("data/sys.dic").unwrap();
        let unkdic = Blob::open("data/unk.dic").unwrap();
        let matrix = Blob::open("data/matrix.bin").unwrap();
        let unkdef = Blob::open("data/char.bin").unwrap();

        let dict = Dict::load(sysdic, unkdic, matrix, unkdef).unwrap();
        JParser {
            dict,
            cache: Cache::new(),
        }
    }

    pub fn parse(&mut self, text: &str) -> Result<String, TokenizeError> {
        let mut output: Vec<LexerToken> = Vec::with_capacity(20);
        let mut result: Vec<String> = Vec::with_capacity(20);
        self.dict
            .tokenize_with_cache(&mut self.cache, text, &mut output)?;
        for token in output {
            let feature = token.get_feature(&self.dict);
            let mut splits = feature.split(',');
            if splits.nth(0).unwrap() == "補助記号" {
                continue;
            }
            let kanji = splits.nth(9).unwrap_or_default().to_string();
            let ruby = splits.next().unwrap_or_default().to_string().to_hiragana();
            let s = if kanji == ruby {
                kanji
            } else {
                format!("{kanji}\u{1e}{ruby}")
            };

            result.push(s);
        }
        Ok(result.join("\u{1f}"))
    }
}
