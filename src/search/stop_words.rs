// https://gist.github.com/sebleier/554280 source of English stop words
// https://github.com/solariz/german_stopwords/blob/master/german_stopwords_plain.txt source of German stop words

use memoize::memoize;
use std::collections::HashSet;
use tantivy::tokenizer::{Token, TokenFilter, TokenStream, Tokenizer};


#[memoize]
fn get_default_args() -> HashSet<String> {
    let mut result: HashSet<String> = HashSet::new();

    {
        let data = include_bytes!("stop_words_english.txt");
        let content = std::str::from_utf8(data).expect("Could not convert English");
        for x in content.split("\n") {
            result.insert(x.to_string());
        }
    }
    {
        let data = include_bytes!("stop_words_german.txt");
        let content = std::str::from_utf8(data).expect("Could not convert English");
        for x in content.split("\n") {
            result.insert(x.to_string());
        }
    }

    return result;
}

#[derive(Clone)]
pub struct StopFilter;

impl TokenFilter for StopFilter {
    type Tokenizer<T: Tokenizer> = StopFilterWrapper<T>;

    fn transform<T: Tokenizer>(self, tokenizer: T) -> Self::Tokenizer<T> {
        StopFilterWrapper {
            inner: tokenizer,
            buffer: String::new(),
        }
    }
}

impl<T: Tokenizer> Tokenizer for StopFilterWrapper<T> {
    type TokenStream<'a> = StopFilterStream<'a, T::TokenStream<'a>>;

    fn token_stream<'a>(&'a mut self, text: &'a str) -> Self::TokenStream<'a> {
        StopFilterStream {
            tail: self.inner.token_stream(text),
            buffer: &mut self.buffer,
        }
    }
}


impl<T: TokenStream> TokenStream for StopFilterStream<'_, T> {
    fn advance(&mut self) -> bool {
        if !self.tail.advance() {
            return false;
        }
        if let Some(res) = get_default_args().get(&self.token().text.clone()) {
            return false;
        }
        true
    }

    fn token(&self) -> &Token {
        self.tail.token()
    }

    fn token_mut(&mut self) -> &mut Token {
        self.tail.token_mut()
    }
}


#[derive(Clone)]
pub struct StopFilterWrapper<T> {
    inner: T,
    buffer: String,

}


pub struct StopFilterStream<'a, T> {
    tail: T,
    buffer: &'a mut String,

}