use crate::Evaluator;

pub struct UntypedLambdaCalculus;

impl Evaluator for UntypedLambdaCalculus {
    fn run(&self, input: &str) {
        println!("Untyped Lambda Calculus");
        println!("input: {:?}", input);
    }
}

enum Term {
    Var(String),
    Abs(String, Box<Term>),
    App(Box<Term>, Box<Term>),
}

enum Token {
    Lambda,
    Dot,
    LParen,
    RParen,
    Identifier(String),
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

pub struct Span {
    start: usize,
    length: usize,
}

impl Span {
    pub fn new(start: usize, length: usize) -> Span {
        Span { start, length }
    }
}

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

fn tokenize(input: &str) -> Result<Vec<Token>, TokenizeError> {
    let mut tokens = Vec::new();
    let mut it = InputIterator::new(input);
    while let Some(&c) = it.peek() {
        match c {
            'λ' => {
                tokens.push(Token::Lambda);
                it.next();
            }
            '.' => {
                tokens.push(Token::Dot);
                it.next();
            }
            '(' => {
                tokens.push(Token::LParen);
                it.next();
            }
            ')' => {
                tokens.push(Token::RParen);
                it.next();
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
                tokens.push(Token::Identifier(identifier));
            }
            _ => {
                return tok_err("Unexpected character", Span::new(it.offset(), 1));
            }
        }
    }
    Ok(tokens)
}
