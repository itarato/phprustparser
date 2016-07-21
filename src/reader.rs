#[derive(Default)]
pub struct Reader {
    chars: Vec<char>,
    position: usize,
}

impl Reader {
    pub fn new(source: String) -> Reader {
        let chars = source.chars().collect::<Vec<char>>();
        Reader {
            chars: chars,
            ..Default::default()
        }
    }

    pub fn peek_char(&self) -> char {
        self.chars[self.position]
    }

    pub fn peek_char_n(&self, n: usize) -> String {
        self.chars.iter().skip(self.position).take(n).map(|ch| *ch).collect()
    }

    pub fn peek_until<P>(&self, pred: P) -> String where for <'r> P: FnMut(&'r &char) -> bool {
        self.chars.iter().skip(self.position).take_while(pred).map(|ch| *ch).collect()
    }

    pub fn forward(&mut self) {
        self.position += 1
    }

    pub fn forward_n(&mut self, n: usize) {
        self.position += n
    }

    pub fn is_end(&self) -> bool {
        self.position >= self.chars.len()
    }
}