use super::super::Location;

pub enum Symbol {
    LParens,
    RParens,
    Lambda,
    Dot,
}

pub enum TokenData {
    Symbol(Symbol),
    Identifier(String),
}

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

pub fn get_symbol(c: char) -> Option<Symbol> {
    match c {
        '('  => Some(Symbol::LParens),
        ')'  => Some(Symbol::RParens),
        '\\' => Some(Symbol::Lambda),
        '.'  => Some(Symbol::Dot),
        _    => None
    }
}