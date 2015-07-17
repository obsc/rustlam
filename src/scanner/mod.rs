pub mod standard;
pub mod file;

pub use self::standard::StdScanner;
pub use self::file::FileScanner;

pub struct Scanner<I> where I: Iterator<Item = String> {
    iter: I,
    src: usize,
    line: usize,
    col: usize,
    text: Vec<String>,
    buf: Vec<char>,
}

#[derive(Debug)]
pub struct Character {
    cargo: char,
    src: usize,
    line: usize,
    col: usize,
}

impl<I> Scanner<I> where I: Iterator<Item = String> {
    pub fn new(inner: I) -> Self {
        Scanner{
            iter: inner,
            src: 0,
            line: 0,
            col: 0,
            text: Vec::new(),
            buf: Vec::new(),
        }
    }

    pub fn get_ref(&self) -> &I {
        &self.iter
    }

    pub fn get_mut(&mut self) -> &mut I {
        &mut self.iter
    }
}

impl<I> Iterator for Scanner<I> where I: Iterator<Item = String> {
    type Item = Character;

    fn next(&mut self) -> Option<Character> {
        let c = match self.buf.get(self.col) {
            Some(&c) => Some(c),
            None     => {
                self.iter.next().map(|line| {
                    self.buf = line.trim_right().chars().collect();
                    self.buf.push('\n');
                    self.text.push(line);
                    self.buf[self.col]
                })
            },
        };

        c.map(|c| {
            let out = Character{
                cargo: c,
                src: self.src,
                line: self.line,
                col: self.col,
            };

            self.src += 1;
            if c == '\n' {
                self.buf = Vec::new();
                self.col = 0;
                self.line += 1;
            } else {
                self.col += 1;
            }

            out
        })
    }
}
