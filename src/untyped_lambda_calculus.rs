use crate::evaluator::Evaluator;
use crate::parsing_utils::tok_err;
use crate::parsing_utils::tokenize_error_to_string;
use crate::parsing_utils::InputIterator;
use crate::parsing_utils::Spanned;
use crate::parsing_utils::Span;
use crate::parsing_utils::TokenizeError;
use crate::parsing_utils::ParseError;
use crate::parsing_utils::parse_err;
use crate::parsing_utils::parse_error_to_string;


use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct UntypedLambdaCalculus;

impl Evaluator for UntypedLambdaCalculus {
    fn run(&self, input: &str) -> String {
        let mut out = String::new();
        out.push_str("input: ");
        out.push_str(input);
        out.push_str("\n");
        let tokens = tokenize(input);
        if tokens.is_err() {
            let err = tokenize_error_to_string(tokens.err().unwrap(), input);
            out.push_str(err.as_str());
            return out;
        }
        let tokens = tokens.unwrap();
        out.push_str("tokens: [");
        for token in &tokens {
            out.push_str(&format!("{}, ", token));
        }
        out.push_str("]\n");

        let term = parse(&tokens);
        if term.is_err() {
            let err = parse_error_to_string(term.err().unwrap(), input);
            out.push_str(err.as_str());
            return out;
        }
        let term = term.unwrap();
        out.push_str("parsed: ");
        out.push_str(&format!("{}\n", term));
        out
    }
    fn __debug__(&self) -> String {
        format!("{:?}", self)
    }
    fn name(&self) -> String {
        "Untyped Lambda Calculus".to_string()
    }
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

fn app(term: Term, param: Term) -> Term {
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



fn tokenize(input: &str) -> Result<Vec<Spanned<Token>>, TokenizeError> {
    let mut it = InputIterator::new(input);
    while let Some(&c) = it.peek() {
        match c {
            'λ' | '\\' => it.next_and_push(Token::Lambda, 1),
            '.' => it.next_and_push(Token::Dot, 1),
            '(' => it.next_and_push(Token::LParen, 1),
            ')' => it.next_and_push(Token::RParen, 1),
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
                let len = identifier.len();
                it.push(Token::Identifier(identifier), len);
            }
            _ => {
                it.next();
                return Err(tok_err("Unexpected character", Span::new(it.offset(), 1)));
            }
        }
    }
    Ok(it.tokens)
}


fn parse(tokens: &Vec<Spanned<Token>>) -> Result<Term, ParseError> {
    let mut it = tokens.iter().peekable();
    parse_term(&mut it)
}

enum Precedence {
    Lowest,
    Application,
    Lambda,
}

fn parse_term(it: &mut std::iter::Peekable<std::slice::Iter<Spanned<Token>>>) -> Result<Term, ParseError> {
    match it.peek() {
        None => Err(parse_err("Unexpected end of input", Span::new(0, 0))),
        Some(t) => match t.item {
            Token::Identifier(_) => parse_application(it),
            Token::Lambda => parse_abstraction(it),
            _ => Err(parse_err("Unexpected token", t.span)),
        },
    }
}

fn parse_abstraction(
    it: &mut std::iter::Peekable<std::slice::Iter<Spanned<Token>>>,
) -> Result<Term, ParseError> {
    expect_token(it, Token::Lambda)?;
    let id = expect_identifier(it)?;
    expect_token(it, Token::Dot)?;
    let term = parse_term(it)?;
    Ok(abs(id, term))
}

fn parse_application(
    it: &mut std::iter::Peekable<std::slice::Iter<Spanned<Token>>>,
) -> Result<Term, ParseError> {
    let id = expect_identifier(it)?;
    let mut exp = Term::Var(id);
    while let Some(t) = it.peek() {
        match t.item {
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
    it: &mut std::iter::Peekable<std::slice::Iter<Spanned<Token>>>,
) -> Result<String, ParseError> {
    match it.next() {
        None => Err(parse_err("Unexpected end of input", Span::new(0, 0))),
        Some(t) => match &t.item {
            Token::Identifier(id) => Ok(id.clone()),
            _ => Err(parse_err("Expected identifier", t.span)),
        },
    }
}

fn expect_token(
    it: &mut std::iter::Peekable<std::slice::Iter<Spanned<Token>>>,
    expected: Token,
) -> Result<(), ParseError> {
    match it.next() {
        None => Err(parse_err("Unexpected end of input", Span::new(0, 0))),
        Some(t) => {
            if t.item == expected {
                Ok(())
            } else {
                let msg = format!("Unexpected token '{}', expected '{}'", t.item, expected);
                Err(parse_err(&msg, t.span))
            }
        }
    }
}
