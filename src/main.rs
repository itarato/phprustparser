macro_rules! in_char_ranges_callback {
    ( $([$f:expr, $t: expr]),* ) => {
        |ch| {
            $( (**ch >= $f && **ch <= $t) ||)* false
        }
    }
}

macro_rules! in_chars_callback {
    ( $($c:expr),* ) => {
        |ch| {
            $( (**ch == $c) ||)* false
        }
    }
}

macro_rules! not_in_chars_callback {
    ( $($c:expr),* ) => {
        |ch| {
            $( (**ch != $c) &&)* true
        }
    }
}

macro_rules! is_valid_fn {
    (VARNAME) => (
        in_char_ranges_callback!{['a', 'z'], ['A', 'Z'], ['0', '9'], ['_', '_'], ['$', '$']}
    );
    (NUMBER) => (
        in_char_ranges_callback!{['0', '9'], ['-', '-'], ['.', '.']}
    );
    (OP) => (
        in_chars_callback!{'+', '-', '*', '/', '%', '^', '=', '!', '.'}
    );
    (NOT, $ch:expr) => (
        not_in_chars_callback!($ch)
    );
}

macro_rules! is_valid {
    (VARNAME_START, $c:expr) => ({
        ($c >= 'a' && $c <= 'z') || ($c >= 'A' && $c <= 'Z')
    });
}

macro_rules! add_one_char_token_reader {
    ($tokenizer:expr, $ch:expr, $out:expr) => {{
        $tokenizer.add_token_reader(Box::new(|reader, _| {
            if reader.peek_char() == $ch {
                reader.forward();
                return Some($out);
            }
            None
        }));
    }}
}

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

    tokenizer.add_token_reader(Box::new(|reader, _| {
        for keyword_name in Keyword::all_names() {
            if reader.peek_char_n(keyword_name.len()) == keyword_name {
                reader.forward_n(keyword_name.len());
                return Some(Token::Keyword(Keyword::for_name(keyword_name)));
            }
        }
        None
    }));

    // String.
    tokenizer.add_token_reader(Box::new(|reader, _| {
        let c = reader.peek_char();
        if c == '"' || c == '\'' {
            reader.forward();
            let s = reader.peek_until(is_valid_fn!(NOT, c));
            reader.forward_n(s.len() + 1);
            return Some(Token::StringValue(s));
        }
        None
    }));

    // Variable name.
    tokenizer.add_token_reader(Box::new(|reader, _| {
        if reader.peek_char() == '$' {
            reader.forward();
            let s = reader.peek_until(is_valid_fn!(VARNAME));
            reader.forward_n(s.len());
            return Some(Token::VariableName(s))
        }
        None
    }));

    // Operation.
    tokenizer.add_token_reader(Box::new(|reader, _| {
        let ops = reader.peek_until(is_valid_fn!(OP));
        if ops.len() > 0 {
            reader.forward_n(ops.len());
            return Some(Token::Op(ops));
        }
        None
    }));

    // Semicolon.
    add_one_char_token_reader!(tokenizer, ';', Token::Semicolon);

    // Brackets.
    add_one_char_token_reader!(tokenizer, '(', Token::ParenthesisOpen);
    add_one_char_token_reader!(tokenizer, ')', Token::ParenthesisClose);
    add_one_char_token_reader!(tokenizer, '{', Token::BlockOpen);
    add_one_char_token_reader!(tokenizer, '}', Token::BlockClose);

    // Function call.
    tokenizer.add_token_reader(Box::new(|reader, _| {
        let ch = reader.peek_char();
        if is_valid!(VARNAME_START, ch) {
            let name = reader.peek_until(is_valid_fn!(VARNAME));
            reader.forward_n(name.len());
            return Some(Token::FunctionName(name));
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

function say($text) {
    echo('Hello ' . $text);
}

$foo = 'Hello world';
say($foo);
".to_string();
}

type TokenReader = Fn(&mut Reader, &mut ReaderStateCollection) -> Option<Token>;

#[derive(Debug)]
enum Keyword {
    Function,
}

impl Keyword {
    fn for_name(name: &str) -> Keyword {
        match name {
            "function" => Keyword::Function,
            _ => panic!("Illegal keyword"),
        }
    }

    fn all_names() -> Vec<&'static str> {
        vec![
            "function",
        ]
    }
}

#[derive(Debug)]
enum Token {
    PhpStart,
    Keyword(Keyword),
    VariableName(String),
    Op(String),
    StringValue(String),
    Semicolon,
    FunctionName(String),
    ParenthesisOpen,
    ParenthesisClose,
    BlockOpen,
    BlockClose,
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
        println!("{:?}", self.tokens);
    }

    fn read(&mut self) {
        for tr in &self.token_readers {
            match tr(&mut self.reader, &mut self.states) {
                Some(Token::Whitespace) => println!("."),
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