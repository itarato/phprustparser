use tokenizer::{Token, Keyword};
use std::rc::Rc;
use std::cell::RefCell;
use std::iter::Iterator;
use std::fmt;

#[derive(Debug)]
struct Node {
    name: String,
    children: Vec<Node>,
}

impl Node {
    fn new(name: String) -> Node {
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
        println!("{:#?}", root);
    }

    pub fn build_code_block(&mut self) -> Node {
        self.tlr.next();
        let mut n = Node::new("code block".to_string());

        while true {
            {
                let currentToken = self.tlr.current();
                match currentToken {
                    &Token::Eof | &Token::BlockClose => break,
                    _ => { },
                }
            }

            let child = self.build_stmt();
            n.add(child);
        }

        n
    }

    pub fn build_stmt(&mut self) -> Node {
        let mut n = Node::new("stmt".to_string());

        let child = match *self.tlr.current() {
            Token::Keyword(Keyword::Function) => self.build_stmt_fn(),
            _ => self.build_stmt_exp(),
        };
        n.add(child);

        n
    }

    pub fn build_stmt_exp(&mut self) -> Node {
        let n = Node::new("exp".to_string());

        while true {
            {
                let currentToken = self.tlr.current();
                match currentToken {
                    &Token::Semicolon => break,
                    _ => {
                    },
                }
            }

            self.tlr.next();
        }
        self.tlr.next();

        n
    }

    pub fn build_stmt_fn(&mut self) -> Node {
        let mut n = Node::new("fn".to_string());

        self.tlr.next();
        n.add(Node::new("T> keyword function".to_string()));

        match *self.tlr.current() {
            Token::FunctionName(ref s) => {
                n.add(Node::new(format!("T> function name: {}", s)));
            },
            ref t @ _ => panic!("Unexpected token {:?}", t),
        }
        self.tlr.next();

        let child = self.build_arg_list();
        n.add(child);

        // Fn block open.
        self.tlr.next();

        let child = self.build_code_block(); // ---> NOW ADD EXP TO STMT AS SECOND OPTION
        n.add(child);

        // Fn block close.
        self.tlr.next();

        n
    }

    pub fn build_arg_list(&mut self) -> Node {
        let mut n = Node::new("arg list".to_string());

        // Open parenthesis.
        self.tlr.next();

        while true {
            {
                match self.tlr.current() {
                    &Token::VariableName(ref v) => n.add(Node::new(format!("T> variable name: {}", v))),
                    _ => break,
                };
            }
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

    fn current(&self) -> &Token {
        &self.tokens.borrow()[self.pos]
    }

    fn next(&mut self) {
        self.pos += 1;
    }
}
