pub mod scanner;
pub mod lexer;

#[derive(Debug, Copy, Clone)]
pub struct Location {
  src: usize,
  line: usize,
  col: usize,
  size: usize,
}

impl Location {
  pub fn new() -> Self {
    Location {
      src: 0,
      line: 0,
      col: 0,
      size: 1,
    }
  }

  pub fn next_col(&mut self) {
    self.src += 1;
    self.col += 1;
  }

  pub fn next_row(&mut self) {
    self.src += 1;
    self.line += 1;
    self.col = 0;
  }

  pub fn get_src(&self) -> usize { self.src }
  pub fn get_line(&self) -> usize { self.line }
  pub fn get_col(&self) -> usize { self.col }
  pub fn get_size(&self) -> usize { self.size }
  pub fn set_size(&mut self, size: usize) { self.size = size; }
}
