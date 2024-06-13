use crate::Evaluator;
use crate::colors::*;

use std::fmt::{self, Display, Formatter};


pub struct UntypedLambdaCalculus;

impl Evaluator for UntypedLambdaCalculus {
    fn run(&self, input: &str) {
        println!("Untyped Lambda Calculus");

        println!("input: {:?}", input);
        let tokens = tokenize(input);
        if tokens.is_err() {
            print_tokenize_error(tokens.err().unwrap(), input);
            return;
        }
        let tokens = tokens.unwrap();
        let mut string = String::new();
        string.push_str("tokens: [");
        for token in tokens {
            string.push_str(&format!("{}, ", token));
        }
        string.push_str("]");
        println!("{}", string);
    }
}

fn print_tokenize_error(err: TokenizeError, input: &str) {
    println!("Error: {}", err.message);

    let (start, end) = (err.span.start, err.span.start + err.span.length);

    let mut out = String::new();
    // Add input text
    out.push_str(CYAN);
    out.push_str(&input[..start]);
    out.push_str(RED);
    out.push_str(&input[start..end]);
    out.push_str(CYAN);
    out.push_str(&input[end..]);

    // Add ^ marker
    out.push_str("\n");
    out.push_str(&" ".repeat(start));
    out.push_str(RED);
    out.push_str(&"^".repeat(err.span.length));

    println!("{}", out);
}

enum Term {
    Var(String),
    Abs(String, Box<Term>),
    App(Box<Term>, Box<Term>),
}

#[derive(Debug)]
enum Token {
    Lambda,
    Dot,
    LParen,
    RParen,
    Identifier(String),
}

#[derive(Debug)]
struct SToken {
    token: Token,
    span: Span,
}

impl SToken {
    pub fn new(token: Token, span: Span) -> SToken {
        SToken { token, span }
    }
}

impl Display for SToken {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self.token)
    }
}

pub struct InputIterator<'a> {
    chars: std::iter::Peekable<std::str::Chars<'a>>,
    offset: usize,
}
impl InputIterator<'_> {
    pub fn new(input: &str) -> InputIterator {
        InputIterator {
            chars: input.chars().peekable(),
            offset: 0,
        }
    }
    pub fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }
    pub fn next(&mut self) -> Option<char> {
        let c = self.chars.next();
        if c.is_some() {
            self.offset += 1;
        }
        c
    }
    pub fn offset(&self) -> usize {
        self.offset
    }
}

#[derive(Debug)]
pub struct Span {
    start: usize,
    length: usize,
}

impl Span {
    pub fn new(end: usize, length: usize) -> Span {
        Span { start: end-length, length }
    }
}

#[derive(Debug)]
pub struct TokenizeError {
    message: String,
    span: Span,
}

pub fn tok_err(message: &str, span: Span) -> TokenizeError {
    TokenizeError {
        message: message.to_string(),
        span,
    }
}



fn tokenize(input: &str) -> Result<Vec<SToken>, TokenizeError> {
    let mut tokens = Vec::new();
    let mut it = InputIterator::new(input);
    while let Some(&c) = it.peek() {
        match c {
            'λ' | '\\' => {
                it.next();
                let s = Span::new(it.offset(), 1);
                tokens.push(SToken::new(Token::Lambda, s));
            }
            '.' => {
                it.next();
                let s = Span::new(it.offset(), 1);
                tokens.push(SToken::new(Token::Dot, s));
            }
            '(' => {
                it.next();
                let s = Span::new(it.offset(), 1);
                tokens.push(SToken::new(Token::LParen, s));
            }
            ')' => {
                it.next();
                let s = Span::new(it.offset(), 1);
                tokens.push(SToken::new(Token::RParen, s));
            }
            c if c.is_whitespace() => {
                it.next();
            }
            c if c.is_alphabetic() => {
                let mut identifier = String::new();
                while let Some(&c) = it.peek() {
                    if c.is_alphanumeric() {
                        identifier.push(c);
                        it.next();
                    } else {
                        break;
                    }
                }
                let s = Span::new(it.offset(), identifier.len());
                tokens.push(SToken::new(Token::Identifier(identifier), s));
            }
            _ => {
                it.next();
                return Err(tok_err("Unexpected character", Span::new(it.offset(), 1)));
            }
        }
    }
    Ok(tokens)
}
