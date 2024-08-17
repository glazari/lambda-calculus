use crate::parsing_utils::tok_err;
use crate::parsing_utils::InputIterator;
use crate::parsing_utils::SToken;
use crate::parsing_utils::TokenizeError;

#[derive(Debug, PartialEq)]
pub(super) enum Token {
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

pub(super) fn tokenize(input: &str) -> Result<Vec<SToken<Token>>, TokenizeError> {
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
            '#' => tokenize_constants(&mut it)?,
            ';' => tokenize_comment(&mut it),
            '"' => tokenize_string(&mut it)?,
            c if c.is_digit(10) => tokenize_number(&mut it)?,
            c if c.is_alphabetic() => tokenize_symbol(&mut it),
            c if c.is_whitespace() => tokenize_whitespace(&mut it),
            c => {
                let msg = format!("Invalid character: {}", c);
                it.next();
                return Err(it.tok_err(&msg, 1));
            }
        }
    }
    return Ok(it.tokens);
}

fn tokenize_constants(it: &mut InputIterator<Token>) -> Result<(), TokenizeError> {
    it.next();
    match it.peek() {
        Some('t') => it.next_and_push(Token::True, 2),
        Some('f') => it.next_and_push(Token::False, 2),
        Some('n') => it.next_and_push(Token::Nil, 2),
        Some(c) => {
            let msg = format!("Invalid character after #: #{}", c);
            it.next();
            return Err(it.tok_err(&msg, 2));
        }
        None => return Err(it.tok_err("Unexpected end of input", 1)),
    }
    Ok(())
}

fn tokenize_string(it: &mut InputIterator<Token>) -> Result<(), TokenizeError> {
    let mut s = String::new();
    it.next();
    while let Some(&c) = it.peek() {
        match c {
            '"' => {
                it.next();
                let len = s.len();
                it.push(Token::String(s), len + 2);
                return Ok(());
            }
            _ => {
                s.push(c);
                it.next();
            }
        }
    }
    Err(it.tok_err("Unterminated string", s.len()))
}

fn tokenize_symbol(it: &mut InputIterator<Token>) {
    let mut symbol = String::new();
    while let Some(&c) = it.peek() {
        match c {
            c if c.is_alphabetic() || c.is_digit(10) || c == '_' => {
                symbol.push(c);
                it.next();
            }
            _ => break,
        }
    }
    let len = symbol.len();
    it.push(Token::Symbol(symbol), len);
}

fn tokenize_whitespace(it: &mut InputIterator<Token>) {
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

fn tokenize_number(it: &mut InputIterator<Token>) -> Result<(), TokenizeError> {
    let mut num = String::new();
    while let Some(&c) = it.peek() {
        match c {
            c if c.is_digit(10) => {
                num.push(c);
                it.next();
            }
            c if c.is_alphabetic() => return Err(tok_err("Invalid number", it.span(1))),
            _ => break,
        }
    }
    it.push(Token::Number(num.parse().unwrap()), num.len());
    Ok(())
}

fn tokenize_comment(it: &mut InputIterator<Token>) {
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

fn s(string: &str) -> String {
    string.to_string()
}

mod test {
    use super::*;
    use crate::parsing_utils::strip_spans;

    #[test]
    fn test_tokenize() {
        let cases = vec![
            (
                "(\'`,+-*/=<>)",
                vec![
                    Token::OpenParen,
                    Token::Quote,
                    Token::Backquote,
                    Token::Comma,
                    Token::Plus,
                    Token::Minus,
                    Token::Multiply,
                    Token::Divide,
                    Token::Equal,
                    Token::LessThan,
                    Token::GreaterThan,
                    Token::CloseParen,
                ],
            ),
            (
                "(#t #f #n)",
                vec![
                    Token::OpenParen,
                    Token::True,
                    Token::Whitespace(s(" ")),
                    Token::False,
                    Token::Whitespace(s(" ")),
                    Token::Nil,
                    Token::CloseParen,
                ],
            ),
            (
                "\"hello world\" \"rock\" \"paper\"",
                vec![
                    Token::String(s("hello world")),
                    Token::Whitespace(s(" ")),
                    Token::String(s("rock")),
                    Token::Whitespace(s(" ")),
                    Token::String(s("paper")),
                ],
            ),
            (
                "123,456,4,789456123",
                vec![
                    Token::Number(123),
                    Token::Comma,
                    Token::Number(456),
                    Token::Comma,
                    Token::Number(4),
                    Token::Comma,
                    Token::Number(789456123),
                ],
            ),
            (
                "hello world",
                vec![
                    Token::Symbol(s("hello")),
                    Token::Whitespace(s(" ")),
                    Token::Symbol(s("world")),
                ],
            ),
        ];

        for case in cases {
            let (input, expected) = case;
            let tokens = tokenize(input).unwrap();
            assert_eq!(strip_spans(tokens), expected);
        }
    }
}
