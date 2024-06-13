use crate::colors::*;
use crate::Evaluator;

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
        for token in &tokens {
            string.push_str(&format!("{}, ", token));
        }
        string.push_str("]");
        println!("{}", string);

        let term = parse(&tokens);
        if term.is_err() {
            print_parse_error(term.err().unwrap(), input);
            return;
        }
        let term = term.unwrap();
        println!("parsed: {:?}", term);
        println!("{}", term);

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

#[derive(Debug)]
enum Term {
    Var(String),
    Abs(String, Box<Term>),
    App(Box<Term>, Box<Term>),
}

impl Display for Term {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Term::Var(id) => write!(f, "{}", id),
            Term::Abs(id, exp) => write!(f, "λ{}.{}", id, exp),
            Term::App(t1, t2) => write!(f, "({} {})", t1, t2),
        }
    }
}

fn  app(term: Term, param: Term) -> Term {
    Term::App(Box::new(term), Box::new(param))
}

fn abs(id: String, exp: Term) -> Term {
    Term::Abs(id, Box::new(exp))
}

#[derive(Debug, PartialEq)]
enum Token {
    Lambda,
    Dot,
    LParen,
    RParen,
    Identifier(String),
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Token::Lambda => write!(f, "λ"),
            Token::Dot => write!(f, "."),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::Identifier(id) => write!(f, "{}", id),
        }
    }
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

    pub fn span(&self, length: usize) -> Span {
        Span {
            start: self.offset - length,
            length,
        }
    }

    pub fn stoken(&self, token: Token, length: usize) -> SToken {
        SToken {
            token,
            span: self.span(length),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Span {
    start: usize,
    length: usize,
}

impl Span {
    pub fn new(end: usize, length: usize) -> Span {
        Span {
            start: end - length,
            length,
        }
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

fn stoken(token: Token, span: Span) -> SToken {
    SToken { token, span }
}

fn tokenize(input: &str) -> Result<Vec<SToken>, TokenizeError> {
    let mut tokens = Vec::new();
    let mut it = InputIterator::new(input);
    while let Some(&c) = it.peek() {
        match c {
            'λ' | '\\' => {
                it.next();
                tokens.push(it.stoken(Token::Lambda, 1));
            }
            '.' => {
                it.next();
                tokens.push(it.stoken(Token::Dot, 1));
            }
            '(' => {
                it.next();
                tokens.push(it.stoken(Token::LParen, 1));
            }
            ')' => {
                it.next();
                tokens.push(it.stoken(Token::RParen, 1));
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

#[derive(Debug)]
struct ParseError {
    message: String,
    span: Span,
}

fn parse_err(message: &str, span: Span) -> ParseError {
    ParseError {
        message: message.to_string(),
        span,
    }
}

fn print_parse_error(err: ParseError, input: &str) {
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

fn parse(tokens: &Vec<SToken>) -> Result<Term, ParseError> {
    let mut it = tokens.iter().peekable();
    parse_term(&mut it)
}

enum Precedence {
    Lowest,
    Application,
    Lambda,
}

fn parse_term(it: &mut std::iter::Peekable<std::slice::Iter<SToken>>) -> Result<Term, ParseError> {
    match it.peek() {
        None => Err(parse_err("Unexpected end of input", Span::new(0, 0))),
        Some(t) => match t.token {
            Token::Identifier(_) => parse_application(it),
            Token::Lambda => parse_abstraction(it),
            _ => Err(parse_err("Unexpected token", t.span)),
        },
    }
}

fn parse_abstraction(
    it: &mut std::iter::Peekable<std::slice::Iter<SToken>>,
) -> Result<Term, ParseError> {
    expect_token(it, Token::Lambda)?;
    let id = expect_identifier(it)?;
    expect_token(it, Token::Dot)?;
    let term = parse_term(it)?;
    Ok(abs(id, term))
}

fn parse_application(
    it: &mut std::iter::Peekable<std::slice::Iter<SToken>>,
) -> Result<Term, ParseError> {
    let id = expect_identifier(it)?;
    let mut exp = Term::Var(id);
    while let Some(t) = it.peek() {
        match t.token {
            Token::Identifier(_) => {
                let id = expect_identifier(it)?;
                let term = Term::Var(id);
                exp = app(exp, term);
            }
            Token::Lambda => {
                let term = parse_abstraction(it)?;
                exp = app(exp, term);
            }

            _ => break,
        }
    }
    Ok(exp)
}

fn expect_identifier(
    it: &mut std::iter::Peekable<std::slice::Iter<SToken>>,
) -> Result<String, ParseError> {
    match it.next() {
        None => Err(parse_err("Unexpected end of input", Span::new(0, 0))),
        Some(t) => match &t.token {
            Token::Identifier(id) => Ok(id.clone()),
            _ => Err(parse_err("Expected identifier", t.span)),
        },
    }
}

fn expect_token(
    it: &mut std::iter::Peekable<std::slice::Iter<SToken>>,
    expected: Token,
) -> Result<(), ParseError> {
    match it.next() {
        None => Err(parse_err("Unexpected end of input", Span::new(0, 0))),
        Some(t) => {
            if t.token == expected {
                Ok(())
            } else {
                let msg = format!("Unexpected token '{}', expected '{}'", t.token, expected);
                Err(parse_err(&msg, t.span))
            }
        }
    }
}
