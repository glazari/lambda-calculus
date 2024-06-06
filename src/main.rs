use std::io::{self, Write};
use std::collections::HashSet;

fn main() {
    run();
}

fn tokenize(input: &str) -> Vec<&str> {
    input.split_whitespace().collect()
}

fn consts(term: &Term) -> HashSet<&str> { 
    let set = match term {
        Term::True => set("true"), 
        Term::False => set("false"), 
        Term::Zero => set("0"), 
        Term::If(t1, t2, t3) => {
            let mut s = consts(t1);
            s.extend(consts(t2).iter());
            s.extend(consts(t3).iter());
            s
        }
        Term::Succ(t1) => consts(t1),
        Term::Pred(t1) => consts(t1),
        Term::IsZero(t1) => consts(t1),
    };
    set
}

fn size(term: &Term) -> usize {
    match term {
        Term::True => 1,
        Term::False => 1,
        Term::Zero => 1,
        Term::If(t1, t2, t3) => 1 + size(t1) + size(t2) + size(t3),
        Term::Succ(t1) => 1 + size(t1),
        Term::Pred(t1) => 1 + size(t1),
        Term::IsZero(t1) => 1 + size(t1),
    }
}

fn depth(term: &Term) -> usize {
    match term {
        Term::True => 1,
        Term::False => 1,
        Term::Zero => 1,
        Term::If(t1, t2, t3) => 1 + depth(t1).max(depth(t2)).max(depth(t3)),
        Term::Succ(t1) => 1 + depth(t1),
        Term::Pred(t1) => 1 + depth(t1),
        Term::IsZero(t1) => 1 + depth(t1),
    }
}

fn set(t: &str) -> HashSet<&str> {
    let mut set = HashSet::new();
    set.insert(t);
    set
}

fn parser(tokens: &Vec<&str>, i: usize) -> Result<(Term, usize), ParseError> {
    let (term, i) = match tokens[i] {
        "true" => (Term::True, i + 1),
        "false" => (Term::False, i + 1),
        "if" => {
            let (t1, i) = parser(tokens, i + 1)?;
            let i = expect(tokens, i, "then")?;
            let (t2, i) = parser(tokens, i)?;
            let i = expect(tokens, i, "else")?;
            let (t3, i) = parser(tokens, i)?;
            (if_(t1, t2, t3), i)
        }
        "0" => (Term::Zero, i + 1),
        "succ" => {
            let (t1, word) = parser(tokens, i + 1)?;
            (succ(t1), word)
        }
        "pred" => {
            let (t1, word) = parser(tokens, i + 1)?;
            (pred(t1), word)
        }
        "iszero" => {
            let (t1, word) = parser(tokens, i + 1)?;
            (iszero(t1), word)
        }
        _ => return Err(err("Unexpected token", i)),
    };
    Ok((term, i))
}

fn expect(tokens: &Vec<&str>, i: usize, expected: &str) -> Result<usize, ParseError> {
    if tokens[i] == expected {
        Ok(i + 1)
    } else {
        let msg = format!("Expected '{}'", expected);
        Err(err(&msg, i))
    }
}

// AST
#[derive(Debug)]
enum Term {
    True,
    False,
    If(Box<Term>, Box<Term>, Box<Term>),
    Zero,
    Succ(Box<Term>),
    Pred(Box<Term>),
    IsZero(Box<Term>),
}

fn if_(t1: Term, t2: Term, t3: Term) -> Term {
    Term::If(Box::new(t1), Box::new(t2), Box::new(t3))
}
fn succ(t1: Term) -> Term {
    Term::Succ(Box::new(t1))
}
fn pred(t1: Term) -> Term {
    Term::Pred(Box::new(t1))
}
fn iszero(t1: Term) -> Term {
    Term::IsZero(Box::new(t1))
}

#[derive(Debug)]
struct ParseError {
    message: String,
    offset: usize,
}

fn print_parse_error(e: ParseError, input: &str) {
    println!("Error: {}", e.message);
    let tokens = tokenize(input);
    print!("{}{}", CYAN, tokens[..e.offset].join(" "));
    if e.offset > 0 {
        print!(" ");
    }
    println!("{}{}{}", RED, tokens[e.offset], RESET);
    for i in 0..e.offset {
        let spaces = tokens[i].len();
        print!("{:width$} ", "", width = spaces);
    }
    // ^^^ for the length of the token
    println!("{}{}{}", RED, "^".repeat(tokens[e.offset].len()), RESET);
}

fn err(message: &str, offset: usize) -> ParseError {
    ParseError {
        message: message.to_string(),
        offset,
    }
}

// Repl
fn run() {
    println!("Lambda REPL");
    loop {
        print_prompt();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input == ":q" {
            break;
        }
        let tokens = tokenize(input);
        let term = parser(&tokens, 0);
        if term.is_err() {
            print_parse_error(term.err().unwrap(), input);
            continue;
        }

        let (t, _) = term.unwrap();
        println!("{:?}", t);
        
        println!("consts: {:?}", consts(&t));
        println!("size: {:?}", size(&t));
        println!("depth: {:?}", depth(&t));
    }
}

fn print_prompt() {
    print!("{}{}{}", GREEN, PROMPT, RESET);
    io::stdout().flush().unwrap();
}

const PROMPT: &str = "> ";
const CYAN: &str = "\x1b[36m";
const GREEN: &str = "\x1b[32m";
const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";
