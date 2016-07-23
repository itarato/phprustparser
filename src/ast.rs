use tokenizer::{Token, Keyword};
use std::rc::Rc;
use std::cell::RefCell;
use std::iter::Iterator;
use std::fmt;

struct Node {
    name: &'static str,
    children: Vec<Node>,
}

impl Node {
    fn new(name: &'static str) -> Node {
        Node {
            name: name,
            children: Default::default(),
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

    pub fn build(&mut self) {
        let root = self.build_code_block();
    }

    pub fn build_code_block(&mut self) -> Node {
        self.tlr.next();
        let mut n = Node::new("root");

        let child = self.build_stmt();
        n.add(child);

        n
    }

    pub fn build_stmt(&mut self) -> Node {
        let mut n = Node::new("stmt");

        let token = self.tlr.current();
        println!("{:?}", token);

        n
    }

    pub fn build_stmt_fn(&mut self) -> Node {
        let mut n = Node::new("fn");

        self.tlr.next();
        n.add(Node::new("T> keyword function"));

        match *self.tlr.current() {
            Token::FunctionName(ref s) => {
                let name = format!("T> function name: {}", s.as_str());
                // let name = "few";
                n.add(Node::new(&name.clone()));
            },
            ref t @ _ => panic!("Unexpected token {:?}", t),
        }

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

    fn current(&self) -> &Token {
        &self.tokens.borrow()[self.pos]
    }

    fn next(&mut self) {
        self.pos += 1;
    }
}
