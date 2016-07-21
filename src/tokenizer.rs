use reader::Reader;

pub type TokenReader = Fn(&mut Reader, &mut ReaderStateCollection) -> Option<Token>;

#[derive(Debug)]
pub enum Keyword {
    Function,
}

impl Keyword {
    pub fn for_name(name: &str) -> Keyword {
        match name {
            "function" => Keyword::Function,
            _ => panic!("Illegal keyword"),
        }
    }

    pub fn all_names() -> Vec<&'static str> {
        vec![
            "function",
        ]
    }
}

#[derive(PartialEq)]
pub enum PhpFileState {
    Outside,
    Inside,
}

impl Default for PhpFileState { fn default() -> Self { PhpFileState::Outside } }

#[derive(Default)]
pub struct ReaderStateCollection {
    pub php_file_state: PhpFileState,
}

#[derive(Debug)]
pub enum Token {
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

pub struct Tokenizer {
    reader: Reader,
    pub tokens: Vec<Token>,
    states: ReaderStateCollection,
    token_readers: Vec<Box<TokenReader>>,
}

impl Tokenizer {
    pub fn new(source: String) -> Tokenizer {
        Tokenizer {
            reader: Reader::new(source),
            tokens: Default::default(),
            states: Default::default(),
            token_readers: Default::default(),
        }
    }

    pub fn add_token_reader(&mut self, tr: Box<TokenReader>) { self.token_readers.push(tr); }

    pub fn run(&mut self) {
        while !self.reader.is_end() {
            self.read();
        }
    }

    fn read(&mut self) {
        for tr in &self.token_readers {
            match tr(&mut self.reader, &mut self.states) {
                Some(Token::Whitespace) => { },
                Some(token) => {
                    // println!("Token: {:?}", token);
                    self.tokens.push(token);
                    break;
                },
                None => { },
            }
        }
    }
}