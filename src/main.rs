#[macro_use]
mod macros;
mod reader;
mod tokenizer;
mod ast;

use tokenizer::{Tokenizer, Token, PhpFileState, Keyword};
use ast::{AstBuilder};

fn main() {
    let source = load_source();
    println!("{}", source);
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
    let tokens = &tokenizer.tokens;
    println!("Tokens: {:?}", &tokens);

    let mut ast_builder = AstBuilder::new();
    ast_builder.build(tokens);
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
