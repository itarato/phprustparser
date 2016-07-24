use tokenizer::{Token, Keyword};
use std::rc::Rc;
use std::cell::RefCell;
use std::iter::Iterator;

#[derive(Debug, Default)]
pub struct Node {
    name: String,
    token: Option<Token>,
    children: Vec<Node>,
}

impl Node {
    fn new(name: String) -> Node {
        Node {
            name: name,
            ..Default::default()
        }
    }

    fn leaf(token: Token) -> Node {
        Node {
            token: Some(token),
            ..Default::default()
        }
    }

    fn add(&mut self, node: Node) {
        self.children.push(node);
    }
}

pub struct AstBuilder<'a> {
    tlr: TokenListReader<'a>,
}

impl<'a> AstBuilder<'a> {
    pub fn new(tokens: &Vec<Token>) -> AstBuilder {
        AstBuilder {
            tlr: TokenListReader::new(Rc::new(RefCell::new(tokens))),
        }
    }

    pub fn build(&mut self) -> Node {
        // PHP start.
        self.tlr.next();
        self.build_code_block()
    }

    pub fn build_code_block(&mut self) -> Node {
        let mut n = Node::new("code block".to_string());

        while true {
            match self.tlr.peek() {
                Token::Eof | Token::BlockClose => break,
                _ => { },
            }

            let child = self.build_stmt();
            n.add(child);
        }

        n
    }

    pub fn build_stmt(&mut self) -> Node {
        let mut n = Node::new("stmt".to_string());

        let child = match self.tlr.peek() {
            Token::Keyword(Keyword::Function) => self.build_stmt_fn(),
            _ => self.build_stmt_exp(),
        };
        n.add(child);

        n
    }

    pub fn build_stmt_exp(&mut self) -> Node {
        let mut n = Node::new("exp".to_string());

        while true {
            let current_token = self.tlr.peek();
            let mut children: Vec<Node> = Vec::new();
            match current_token {
                Token::Semicolon => break,
                Token::FunctionName(s) => { self.build_fn_call(); },
                Token::VariableName(s) => { self.tlr.next(); },
                _ => { self.tlr.next(); },
            };
        }

        // Semicolon;
        self.tlr.next();

        n
    }

    pub fn build_stmt_exp_var(&mut self) -> Node {
        Node::leaf(self.tlr.next())
    }

    pub fn build_fn_call(&mut self) -> Node {
        let n = Node::new("fn call".to_string());

        self.tlr.next();

        n
    }

    pub fn build_stmt_fn(&mut self) -> Node {
        let mut n = Node::new("fn".to_string());

        n.add(Node::leaf(self.tlr.peek()));
        self.tlr.next();

        match self.tlr.peek() {
            t @ Token::FunctionName(_) => { n.add(Node::leaf(t)); },
            t @ _ => panic!("Unexpected token {:?}", t),
        }
        self.tlr.next();

        let child = self.build_arg_list();
        n.add(child);

        // Fn block open.
        self.tlr.next();

        let child = self.build_code_block();
        n.add(child);

        // Fn block close.
        self.tlr.next();

        n
    }

    pub fn build_arg_list(&mut self) -> Node {
        let mut n = Node::new("arg list".to_string());

        // Open parenthesis.
        self.tlr.next();

        // @todo introduce comma as token and use it as selector
        while true {
            match self.tlr.peek() {
                t @ Token::VariableName(_) => n.add(Node::leaf(t)),
                Token::ParenthesisClose => break,
                t @ _ => panic!("Unexpected token for arg list {:?}", t),
            };
            self.tlr.next();
        }

        // Close parenthesis.
        self.tlr.next();

        n
    }
}

struct TokenListReader<'a> {
    tokens: Rc<RefCell<&'a Vec<Token>>>,
    pos: usize,
}

impl<'a> TokenListReader<'a> {
    fn new(tokens: Rc<RefCell<&Vec<Token>>>) -> TokenListReader {
        TokenListReader {
            tokens: tokens,
            pos: Default::default(),
        }
    }

    fn peek(&self) -> Token {
        self.peek_from(0).clone()
    }

    fn peek_from(&self, offset: usize) -> Token {
        self.tokens.borrow()[self.pos + offset].clone()
    }

    fn next(&mut self) -> Token {
        let current = self.peek();
        self.pos += 1;
        current
    }
}
