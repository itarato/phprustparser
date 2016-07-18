fn main() {
    let source = load_source();
    let mut tokenizer = Tokenizer::new(source);

    tokenizer.addTokenReader(Box::new(|reader, states| {
        if states.phpFileState == PhpFileState::Outside {
            if reader.peek_char_n(5) == "<?php" {
                states.phpFileState = PhpFileState::Inside;
                reader.forward_n(5);
                return Some(Token::PhpStart);
            }
        }
        None
    }));

    tokenizer.addTokenReader(Box::new(|reader, states| {
        reader.forward();
        Some(Token::Whitespace)
    }));

    tokenizer.run();
}

fn load_source() -> String {
    return "<?php
$foo = 'Hello world';
echo($foo);
".to_string();
}

type TokenReader = Fn(&mut Reader, &mut ReaderStateCollection) -> Option<Token>;

#[derive(Debug)]
enum Token {
    PhpStart,
    VariableName(String),
    OpEqual,
    StringValue(String),
    Semicolon,
    FunctionName(String),
    ParenthesisOpen,
    ParenthesisClose,
    Whitespace,
}

#[derive(PartialEq)]
enum PhpFileState {
    Outside,
    Inside,
}

impl Default for PhpFileState { fn default() -> Self { PhpFileState::Outside } }

#[derive(Default)]
struct ReaderStateCollection {
    phpFileState: PhpFileState,
}

#[derive(Default)]
struct Reader {
    source: String,
    chars: Vec<char>,
    position: usize,
}

impl Reader {
    fn new(source: String) -> Reader {
        let chars = source.chars().collect::<Vec<char>>();
        Reader {
            source: source,
            chars: chars,
            ..Default::default()
        }
    }

    fn peek_char(&self) -> char {
        self.chars[self.position]
    }

    fn peek_char_n(&self, n: usize) -> String {
        self.chars.iter().skip(self.position).take(n).map(|ch| *ch).collect()
    }

    fn forward(&mut self) {
        self.position += 1
    }

    fn forward_n(&mut self, n: usize) {
        self.position += n
    }

    fn is_end(&self) -> bool {
        self.position >= self.chars.len()
    }

}

struct Tokenizer {
    reader: Reader,
    tokens: Vec<Token>,
    states: ReaderStateCollection,
    tokenReaders: Vec<Box<TokenReader>>,
}

impl Tokenizer {
    fn new(source: String) -> Tokenizer {
        Tokenizer {
            reader: Reader::new(source),
            tokens: Default::default(),
            states: Default::default(),
            tokenReaders: Default::default(),
        }
    }

    fn addTokenReader(&mut self, tr: Box<TokenReader>) { self.tokenReaders.push(tr); }

    fn run(&mut self) {
        while !self.reader.is_end() {
            self.read();
        }
    }

    fn read(&mut self) {
        for tr in &self.tokenReaders {
            match tr(&mut self.reader, &mut self.states) {
                 Some(token) => {
                    println!("Token: {:#?}", token);
                    self.tokens.push(token);
                    break;
                 },
                 None => { },
             }
        }
    }
}