use super::super::Location;

pub enum SymbolData {
  Whitespace,
  LParens,
  RParens,
  Lambda,
  Dot,
  Variable(String),
}

pub struct Symbol {
  data: SymbolData,
  loc: Location,
}
