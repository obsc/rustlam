pub mod token;

use std::iter::Peekable;

use super::scanner::{Scanner, Character};
pub use self::token::{Token, TokenData, get_symbol};

pub struct Lexer<I> where I: Iterator<Item = String> {
  scanner: Peekable<Scanner<I>>,
}

impl<I> Lexer<I> where I: Iterator<Item = String> {
    pub fn new(inner: Scanner<I>) -> Self {
        Lexer {
            scanner: inner.peekable(),
        }
    }
}

impl<I> Iterator for Lexer<I> where I: Iterator<Item = String> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        while self.scanner.peek()
            .map_or(false, |c| c.get_char().is_whitespace()) {
                self.scanner.next();
            }

        let mut id: Vec<Character> = Vec::new();

        while self.scanner.peek()
            .map_or(false, |c| {
                !c.get_char().is_whitespace() &&
                get_symbol(c.get_char()).is_none()
            }) {
                id.push(self.scanner.next().unwrap());
            }

        if id.len() == 0 {
            self.scanner.next().map(|c| {
                let sym = get_symbol(c.get_char()).unwrap();
                Token::new(TokenData::Symbol(sym), c.get_loc())
            })
        } else {
            let mut loc = id[0].get_loc();
            loc.set_size(id.len());
            let id = id.iter().map(|c| c.get_char()).collect();
            Some(Token::new(TokenData::Identifier(id), loc))
        }
    }
}