fn main() {
    let source = load_source();
    let mut tokenizer = Tokenizer::new(source);

    // PHP start token.
    tokenizer.add_token_reader(Box::new(|reader, states| {
        if states.php_file_state == PhpFileState::Outside {
            if reader.peek_char_n(5) == "<?php" {
                states.php_file_state = PhpFileState::Inside;
                reader.forward_n(5);
                return Some(Token::PhpStart);
            }
            return Some(Token::Whitespace)
        }
        None
    }));

    // Variable name.
    tokenizer.add_token_reader(Box::new(|reader, _| {
        if reader.peek_char() == '$' {
            reader.forward();
            let s = reader.peek_until(|ch| {
                **ch >= 'a' && **ch <= 'z'
            });
            reader.forward_n(s.len());
            return Some(Token::VariableName(s))
        }
        None
    }));

    // Otherwise / whitespace.
    tokenizer.add_token_reader(Box::new(|reader, _| {
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
    php_file_state: PhpFileState,
}

#[derive(Default)]
struct Reader {
    chars: Vec<char>,
    position: usize,
}

impl Reader {
    fn new(source: String) -> Reader {
        let chars = source.chars().collect::<Vec<char>>();
        Reader {
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

    fn peek_until<P>(&self, pred: P) -> String where for <'r> P: FnMut(&'r &char) -> bool {
        self.chars.iter().skip(self.position).take_while(pred).map(|ch| *ch).collect()
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
    token_readers: Vec<Box<TokenReader>>,
}

impl Tokenizer {
    fn new(source: String) -> Tokenizer {
        Tokenizer {
            reader: Reader::new(source),
            tokens: Default::default(),
            states: Default::default(),
            token_readers: Default::default(),
        }
    }

    fn add_token_reader(&mut self, tr: Box<TokenReader>) { self.token_readers.push(tr); }

    fn run(&mut self) {
        while !self.reader.is_end() {
            self.read();
        }
    }

    fn read(&mut self) {
        for tr in &self.token_readers {
            match tr(&mut self.reader, &mut self.states) {
                 Some(token) => {
                    println!("Token: {:?}", token);
                    self.tokens.push(token);
                    break;
                 },
                 None => { },
             }
        }
    }
}