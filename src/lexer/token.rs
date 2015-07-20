use super::super::Location;
use super::super::scanner::Character;

#[derive(Debug)]
pub enum Symbol {
    LParens,
    RParens,
    Lambda,
    Dot,
}

#[derive(Debug)]
pub enum TokenData {
    Symbol(Symbol),
    Identifier(String),
}

#[derive(Debug)]
pub struct Token {
  data: TokenData,
  loc: Location,
}

impl Token {
    pub fn new(data: TokenData, loc: Location) -> Token {
        Token{
            data: data,
            loc: loc,
        }
    }
}

pub fn get_token(c: Character) -> Option<Token> {
    let loc = c.get_loc();
    match c.get_char() {
        '('  => Some(Token::new(TokenData::Symbol(Symbol::LParens), loc)),
        ')'  => Some(Token::new(TokenData::Symbol(Symbol::RParens), loc)),
        '\\' => Some(Token::new(TokenData::Symbol(Symbol::Lambda), loc)),
        '.'  => Some(Token::new(TokenData::Symbol(Symbol::Dot), loc)),
        _    => None
    }
}