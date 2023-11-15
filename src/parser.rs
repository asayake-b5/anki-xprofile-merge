use notmecab::{Blob, Cache, Dict, LexerToken, TokenizeError};

pub struct Parser {
    dict: Dict,
    cache: Cache,
}

impl Parser {
    pub fn new() -> Self {
        // you need to acquire a mecab dictionary and place these files here manually
        let sysdic = Blob::open("data/sys.dic").unwrap();
        let unkdic = Blob::open("data/unk.dic").unwrap();
        let matrix = Blob::open("data/matrix.bin").unwrap();
        let unkdef = Blob::open("data/char.bin").unwrap();

        let dict = Dict::load(sysdic, unkdic, matrix, unkdef).unwrap();
        Parser {
            dict,
            cache: Cache::new(),
        }
    }

    pub fn parse(&mut self, text: &str) -> Result<Vec<String>, TokenizeError> {
        let mut output: Vec<LexerToken> = Vec::with_capacity(20);
        let mut result: Vec<String> = Vec::with_capacity(20);
        self.dict
            .tokenize_with_cache(&mut self.cache, text, &mut output)?;
        // println!("{}", result);
        for token in output {
            let feature = token.get_feature(&self.dict);
            result.push(feature.split(',').nth(10).unwrap_or_default().to_string());
        }
        Ok(result)
        // Ok(output)
    }
}
