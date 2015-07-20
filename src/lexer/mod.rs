pub mod token;

use super::scanner::{Scanner, Character};
pub use self::token::{Token, TokenData, get_token};

pub struct Lexer<I> where I: Iterator<Item = String> {
  scanner: Scanner<I>,
  buf: Vec<Character>,
}

impl<I> Lexer<I> where I: Iterator<Item = String> {
    pub fn new(inner: I) -> Self {
        Lexer {
            scanner: Scanner::new(inner),
            buf: Vec::new(),
        }
    }

    pub fn get_ref(&self) -> &I {
        self.scanner.get_ref()
    }

    pub fn get_mut(&mut self) -> &mut I {
        self.scanner.get_mut()
    }

    fn drain_buf(&mut self) -> Option<Token> {
        if self.buf.len() == 0 {
            None
        } else {
            match get_token(self.buf[0]) {
                None => {
                    let id: String = self.buf.iter().map(|c| c.get_char()).collect();
                    let mut loc = self.buf[0].get_loc();
                    loc.set_size(self.buf.len());
                    self.buf.clear();
                    Some(Token::new(TokenData::Identifier(id), loc))
                }
                tok  => {
                    self.buf.clear();
                    tok
                }
            }
        }
    }
}

impl<I> Iterator for Lexer<I> where I: Iterator<Item = String> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        self.drain_buf().or_else(|| {
            loop {
                match self.scanner.next() {
                    None    => return self.drain_buf(),
                    Some(c) => {
                        if c.get_char().is_whitespace() {
                            if self.buf.len() > 0 { return self.drain_buf(); }
                            continue;
                        }
                        match get_token(c) {
                            None => { self.buf.push(c); },
                            tok  => {
                                let buf = self.drain_buf();
                                return buf.map_or(tok, |buf| {
                                    self.buf.push(c);
                                    Some(buf)
                                })
                            }
                        }
                    },
                };
            }
        })
    }
}