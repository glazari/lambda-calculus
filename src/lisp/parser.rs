use crate::parsing_utils::parse_err;
use crate::parsing_utils::ParseError;
use crate::parsing_utils::Span;

use super::ast::Expr;
use super::ast::SExpr;
use super::tokenizer::SToken;
use super::tokenizer::Token;

use std::iter::Peekable;
use std::slice::Iter;

type TokenIterator<'a> = Peekable<Iter<'a, SToken>>;

pub(super) fn parse(tokens: &Vec<SToken>) -> Result<SExpr, ParseError> {
    let mut it = tokens.iter().peekable();
    parse_expression(&mut it)
}

fn parse_expression(it: &mut TokenIterator) -> Result<SExpr, ParseError> {
    skip_whitespace(it);
    let t = it
        .peek()
        .ok_or(parse_err("Unexpected end of input", Span::new(0, 0)))?;
    let exp = match &t.item {
        Token::OpenParen => parse_list(it),
        Token::Symbol(_) => parse_symbol(it),
        Token::Number(_) => parse_number(it),
        //Token::String(_) => parse_string(it),
        //Token::Quote => parse_quote(it),
        token => {
            let msg = format!("Unexpected token: {:?}", token);
            return Err(parse_err(&msg, t.span));
        }
    };

    exp
}

fn parse_list(it: &mut TokenIterator) -> Result<SExpr, ParseError> {
    expect(it, Token::OpenParen, "Expected open paren")?;
    let mut items = Vec::new();
    loop {
        skip_whitespace(it);
        let t = it
            .peek()
            .ok_or(parse_err("Unexpected end of input", Span::new(0, 0)))?;

        if t.item == Token::CloseParen {
            break;
        }
        let exp = parse_expression(it)?;
        items.push(exp);
    }
    expect(it, Token::CloseParen, "Expected close paren")?;
    let spans: Vec<Span> = items.iter().map(|e| e.span).collect();
    let span = Span::merge(&spans);
    let exp = Expr::List(items);
    Ok(SExpr::new(exp, span))
}

fn parse_symbol(it: &mut TokenIterator) -> Result<SExpr, ParseError> {
    let t = it.next().unwrap();
    match &t.item {
        Token::Symbol(s) => {
            let exp = Expr::Symbol(s.clone());
            Ok(SExpr::new(exp, t.span))
        }
        _ => Err(parse_err("Expected symbol", t.span)),
    }
}

fn parse_number(it: &mut TokenIterator) -> Result<SExpr, ParseError> {
    let t = it.next().unwrap();
    match t.item {
        Token::Number(n) => {
            let exp = Expr::Number(n);
            Ok(SExpr::new(exp, t.span))
        }
        _ => Err(parse_err("Expected number", t.span)),
    }
}

fn skip_whitespace(it: &mut TokenIterator) {
    while let Some(t) = it.peek() {
        match t.item {
            Token::Whitespace(_) => {
                it.next();
            }
            _ => break,
        }
    }
}

fn expect_identifier(it: &mut TokenIterator) -> Result<String, ParseError> {
    match it.next() {
        None => Err(parse_err(
            "expected identifier, got end of input",
            Span::new(0, 0),
        )),
        Some(t) => match &t.item {
            Token::Symbol(s) => Ok(s.clone()),
            _ => Err(parse_err("Expected identifier", t.span)),
        },
    }
}

fn expect(it: &mut TokenIterator, expected: Token, msg: &str) -> Result<(), ParseError> {
    let t = it.next().ok_or(parse_err(msg, Span::new(0, 0)))?;
    if t.item != expected {
        return Err(parse_err(msg, t.span));
    }
    Ok(())
}
