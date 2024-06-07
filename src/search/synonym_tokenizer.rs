use std::collections::HashMap;
use std::mem;

use tantivy::tokenizer::{Token, TokenFilter, Tokenizer, TokenStream};

fn get_default_args() -> HashMap<String, String> {
    let res: HashMap<String, String> = HashMap::from(
        [
            ("joghurt".to_string(), "yoghurt".to_string()),
            ("yogurt".to_string(), "yoghurt".to_string()),
            ("zucchini".to_string(), "courgette".to_string()),
            ("filo".to_string(), "phyllo".to_string()),
            ("swede".to_string(), "rutabagaba".to_string()),
            ("chilli".to_string(), "chili".to_string()),
            ("steinpilz".to_string(), "porcini".to_string()),
            ("cep".to_string(), "porcini".to_string()),
            ("houmus".to_string(), "hummus".to_string()),
            ("eggplant".to_string(), "aubergine".to_string()),
            ("za'atar".to_string(), "zaatar".to_string()),
            ("zatar".to_string(), "zaatar".to_string()),
            ("soya".to_string(), "soy".to_string()),
            ("cornflour".to_string(), "cornstarch".to_string()),
            ("ladyfingers".to_string(), "savoiardi".to_string()),
            ("verjus".to_string(), "verjuice".to_string()),
            ("pumpkin".to_string(), "squash".to_string()),
            ("kaffir".to_string(), "makrut".to_string()),
            ("gherkin".to_string(), "cornichon".to_string()),
        ]
    );
    return res;
}


#[derive(Clone)]
pub struct SynonymFilter;

impl TokenFilter for SynonymFilter {
    type Tokenizer<T: Tokenizer> = SynonymFilterWrapper<T>;

    fn transform<T: Tokenizer>(self, tokenizer: T) -> Self::Tokenizer<T> {
        SynonymFilterWrapper {
            inner: tokenizer,
            buffer: String::new()
        }
    }
}

#[derive(Clone)]
pub struct SynonymFilterWrapper<T> {
    inner: T,
    buffer: String,

}

impl<T: Tokenizer> Tokenizer for SynonymFilterWrapper<T> {
    type TokenStream<'a> = SynonymFilterStream<'a, T::TokenStream<'a>>;

    fn token_stream<'a>(&'a mut self, text: &'a str) -> Self::TokenStream<'a> {
        SynonymFilterStream {
            tail: self.inner.token_stream(text),
            buffer: &mut self.buffer,
        }
    }
}

pub struct SynonymFilterStream<'a, T> {
    tail: T,
    buffer: &'a mut String,

}

impl<T: TokenStream> TokenStream for SynonymFilterStream<'_, T> {
    fn advance(&mut self) -> bool {
        if !self.tail.advance() {
            return false;
        }
        if let Some(res) = get_default_args().get(&self.token().text.clone()) {
            // ignore its already ascii
            mem::swap(&mut self.tail.token_mut().text, &mut res.clone());
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


impl<T> SynonymFilterStream<'_, T> {
    fn predicate(&self, token: &Token) -> bool {
        if get_default_args().contains_key(&token.text) {
            return false
        }
        return true
    }
}


