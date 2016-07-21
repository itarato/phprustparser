use tokenizer::{Token};
use std::rc::Rc;
use std::cell::RefCell;
use std::iter::Iterator;

struct Node<T: AstRule> {
    token: Option<Token>,
    name: &'static str,
    children: Vec<Node<T>>,
    rule: Box<T>,
}

impl<T: AstRule> Node<T> {
    fn new(name: &'static str, rule: Box<T>) -> Node<T> {
        Node {
            name: name,
            rule: rule,
            token: Default::default(),
            children: Default::default(),
        }
    }

    fn add(&mut self, node: Node<T>) {
        self.children.push(node);
    }
}

pub struct AstBuilder;

impl AstBuilder {
    pub fn new() -> AstBuilder {
        AstBuilder
    }

    pub fn build(&mut self, tokens: &Vec<Token>) {
        let tlr = Rc::new(RefCell::new(TokenListReader::new(Rc::new(RefCell::new(tokens)))));
        let root = Node::new("code", Box::new(CodeRule));
        root.rule.run(tlr.clone());
    }
}

trait AstRule {
    fn run(&self, tlr: Rc<RefCell<TokenListReader>>);
}

struct CodeRule;

impl AstRule for CodeRule {
    fn run(&self, tlr: Rc<RefCell<TokenListReader>>) {
        // Here I expect: FN_DEF | STMT
        println!("{:?}", tlr.borrow().current());
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
