use crate::colors::*;
use std::fmt::{self, Display, Formatter};
use std::fmt::Debug;

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

pub fn tokenize_error_to_string(err: TokenizeError, input: &str) -> String {
    let mut out = String::new();
    out.push_str(format!("Error: {}\n", err.message).as_str());

    let (start, end) = (err.span.start, err.span.start + err.span.length);

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

    out
}

pub fn tok_err(message: &str, span: Span) -> TokenizeError {
    TokenizeError {
        message: message.to_string(),
        span,
    }
}

#[derive(Debug)]
pub struct SToken<T> {
    pub token: T,
    pub span: Span,
}

pub struct InputIterator<'a, T> {
    chars: std::iter::Peekable<std::str::Chars<'a>>,
    offset: usize,
    pub tokens: Vec<SToken<T>>
}
impl<T> InputIterator<'_, T> {
    pub fn new(input: &str) -> InputIterator<T> {
        InputIterator {
            chars: input.chars().peekable(),
            offset: 0,
            tokens: Vec::new()
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

    pub fn stoken(&self, token: T, length: usize) -> SToken<T> {
        SToken {
            token,
            span: self.span(length),
        }
    }

    pub fn next_and_push(&mut self, token: T, length: usize) {
        self.next();
        self.tokens.push(self.stoken(token, length));
    }
    pub fn push(&mut self, token: T, length: usize) {
        self.tokens.push(self.stoken(token, length));
    }
}

impl<T> SToken<T> {
    pub fn new(token: T, span: Span) -> SToken<T> {
        SToken { token, span }
    }
}

impl<T: Debug> Display for SToken<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self.token)
    }
}


pub fn stoken<T>(token: T, span: Span) -> SToken<T> {
    SToken { token, span }
}


#[derive(Debug)]
pub struct ParseError {
    message: String,
    span: Span,
}

pub fn parse_err(message: &str, span: Span) -> ParseError {
    ParseError {
        message: message.to_string(),
        span,
    }
}

pub fn parse_error_to_string(err: ParseError, input: &str) -> String {
    let mut out = String::new();
    out.push_str(format!("Error: {}\n", err.message).as_str());

    let (start, end) = (err.span.start, err.span.start + err.span.length);

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

    out
}
