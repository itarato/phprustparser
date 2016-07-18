fn main() {
    let source = load_source();
    let mut reader = Reader::new(source);

    reader.addTokenReader(Box::new(|| {
        println!("one tr");
        TokenReaderResult
    }));

    reader.run();
}

fn load_source() -> String {
    return "<?php
$foo = 'Hello world';
echo($foo);
".to_string();
}

struct TokenReaderResult;

type TokenReader = Fn() -> TokenReaderResult;

enum Token {
    PhpStart,
    VariableName(String),
    OpEqual,
    StringValue(String),
    Semicolon,
    FunctionName(String),
    ParenthesisOpen,
    ParenthesisClose,
}

#[derive(PartialEq)]
enum ReaderState {
    Read,
    Finish,
}

impl Default for ReaderState { fn default() -> Self { ReaderState::Read } }

#[derive(PartialEq)]
enum PhpFileState {
    Outside,
    Inside,
}

impl Default for PhpFileState { fn default() -> Self { PhpFileState::Outside } }

#[derive(Default)]
struct Reader {
    source: String,
    chars: Vec<char>,
    position: usize,
    tokens: Vec<Token>,
    readerState: ReaderState,
    phpFileState: PhpFileState,
    tokenReaders: Vec<Box<TokenReader>>,
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

    fn addTokenReader(&mut self, tr: Box<TokenReader>) { self.tokenReaders.push(tr); }

    fn run(&mut self) {
        while self.readerState == ReaderState::Read {
            self.read();
        }
    }

    fn read(&mut self) {
        let c = self.chars[self.position];
        println!("{:#?}", c);

        for tr in &self.tokenReaders {
            tr();
        }

        self.position += 1;
        if self.position >= self.chars.len() {
            self.readerState = ReaderState::Finish;
        }
    }
}