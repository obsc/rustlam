pub mod symbol;

use super::scanner::Scanner;

pub struct Lexer<I> where I: Iterator<Item = String> {
  scanner: Scanner<I>,
}
