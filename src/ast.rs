use tokenizer::{Token, Keyword};
use std::rc::Rc;
use std::cell::RefCell;
use std::iter::Iterator;

#[derive(Default, Debug)]
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
        self.tlr.next(); // PHP start.
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
            _ => {
                let exp_child = self.build_stmt_exp();
                self.tlr.next(); // Semicolon;
                exp_child
            },
            // @todo here comes the control structures
        };
        n.add(child);

        n
    }

    pub fn build_stmt_exp(&mut self) -> Node {
        let mut n = Node::new("exp".to_string());

        let mut children: Vec<Node> = Vec::new();

        while true {
            let current_token = self.tlr.peek();
            match current_token {
                Token::Semicolon | Token::Comma | Token::ParenthesisClose => break,
                Token::Assignment => {
                    self.tlr.next(); // OP =.
                    let mut assignment = Node::new("assignment".to_string());
                    assignment.add(children.pop().unwrap());
                    assignment.add(self.build_stmt_exp());
                    n.add(assignment);
                },
                Token::FunctionName(_) => {
                    children.push(self.build_fn_call());
                },
                Token::VariableName(_) | Token::StringValue(_) | Token::NumericValue(_) | Token::Op(_) => {
                    children.push(self.build_single_token());
                },
                Token::ParenthesisOpen => {
                    self.tlr.next(); // Parenthesis open.
                    children.push(self.build_stmt_exp());
                    self.tlr.next(); // Parenthesis close.
                }
                t @ _ => panic!("Unknown token {:?}", t),
            };

            if children.len() == 3 {
                let mut op_child = Node::new("op".to_string());
                let right = children.pop().unwrap();
                let op = children.pop().unwrap();
                let left = children.pop().unwrap();
                op_child.add(left);
                op_child.add(op);
                op_child.add(right);
                children.push(op_child);
            }
        }

        while children.len() > 0 {
            n.add(children.pop().unwrap());
        }

        n
    }

    pub fn build_arg_exp_list(&mut self) -> Node {
        let mut n = Node::new("fn call exp list".to_string());

        // Open parenthesis.
        self.tlr.next();

        // @todo introduce comma as token and use it as selector
        while true {
            match self.tlr.peek() {
                Token::Comma => { self.tlr.next(); },
                Token::ParenthesisClose => break,
                _ => n.add(self.build_stmt_exp()),
            };
        }

        // Close parenthesis.
        self.tlr.next();

        n
    }

    pub fn build_single_token(&mut self) -> Node {
        Node::leaf(self.tlr.next())
    }

    pub fn build_fn_call(&mut self) -> Node {
        let mut n = Node::new("fn call".to_string());

        n.add(Node::leaf(self.tlr.next()));
        n.add(self.build_arg_exp_list());

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
                Token::VariableName(_) => n.add(Node::leaf(self.tlr.next())),
                Token::Comma => { self.tlr.next(); },
                Token::ParenthesisClose => break,
                t @ _ => panic!("Unexpected token for arg list {:?}", t),
            };
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
