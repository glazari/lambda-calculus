use crate::evaluator::Evaluator;

use crate::parsing_utils::tok_err;
use crate::parsing_utils::InputIterator;
use crate::parsing_utils::SToken;
use crate::parsing_utils::TokenizeError;
use crate::parsing_utils::tokenize_error_to_string;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct Lisp;

impl Evaluator for Lisp {
    fn run(&self, input: &str) -> String {
        let mut out = String::new();
        out.push_str("input: ");
        out.push_str(input);
        let tokens = tokenize(input).map_err(|e| tokenize_error_to_string(e, input));
        if tokens.is_err() {
            return tokens.unwrap_err();
        }

        out.push_str("\ntokens: [");
        for token in tokens.unwrap() {
            out.push_str(&format!("{}, ", token));
        }
        out.push_str("]\n");

        return out;
    }

    fn __debug__(&self) -> String {
        format!("{:?}", self)
    }

    fn name(&self) -> String {
        "Lisp".to_string()
    }
}

#[derive(Debug, PartialEq)]
enum Token {
    OpenParen,
    CloseParen,
    Symbol(String),
    Number(i64),
    String(String),
    Quote,
    Backquote,
    Comma,
    True,
    False,
    Nil,
    Comment(String),
    Plus,
    Minus,
    Multiply,
    Divide,
    Equal,
    LessThan,
    GreaterThan,
    Whitespace(String),
}

fn tokenize(input: &str) -> Result<Vec<SToken<Token>>, TokenizeError> {
    let mut it = InputIterator::new(input);
    while let Some(c) = it.peek() {
        match c {
            '(' => it.next_and_push(Token::OpenParen, 1),
            ')' => it.next_and_push(Token::CloseParen, 1),
            '\'' => it.next_and_push(Token::Quote, 1),
            '`' => it.next_and_push(Token::Backquote, 1),
            ',' => it.next_and_push(Token::Comma, 1),
            '+' => it.next_and_push(Token::Plus, 1),
            '-' => it.next_and_push(Token::Minus, 1),
            '*' => it.next_and_push(Token::Multiply, 1),
            '/' => it.next_and_push(Token::Divide, 1),
            '=' => it.next_and_push(Token::Equal, 1),
            '<' => it.next_and_push(Token::LessThan, 1),
            '>' => it.next_and_push(Token::GreaterThan, 1),
            '#' => {
                match it.peek() {
                    Some('t') => it.next_and_push(Token::True, 2),
                    Some('f') => it.next_and_push(Token::False, 2),
                    Some('n') => it.next_and_push(Token::Nil, 2),
                    _ => {
                        return Err(tok_err("Invalid character", it.span(1)));
                    }
                }
            }
            ';' => {
                let mut comment = String::new();
                while let Some(&c) = it.peek() {
                    if c == '\n' {
                        break;
                    }
                    comment.push(c);
                    it.next();
                }
                let len = comment.len();
                it.push(Token::Comment(comment), len);
            }
            c if c.is_digit(10) => {
                let mut num = String::new();
                num.push(*c);
                while let Some(&c) = it.peek() {
                    match c {
                        c if c.is_digit(10) => {
                            num.push(c);
                            it.next();
                        }
                        c if c.is_whitespace() => break,
                        _ => return Err(tok_err("Invalid number", it.span(1))),
                    }
                }
                it.push(Token::Number(num.parse().unwrap()), num.len());
            }
            c if c.is_whitespace() => {
                let mut ws = String::new();
                while let Some(&c) = it.peek() {
                    if c.is_whitespace() {
                        ws.push(c);
                        it.next();
                    } else {
                        break;
                    }
                }
                let len = ws.len();
                it.push(Token::Whitespace(ws), len);
            }
            _ => {
                it.next();
                return Err(tok_err("Invalid character", it.span(1)));
            }
        }
    }
    return Ok(it.tokens);
}
