use crate::colors::*;
use std::collections::HashSet;

use crate::evaluator::Evaluator;

#[derive(Debug)]
pub struct UntypedArithmetic;

impl Evaluator for UntypedArithmetic {
    fn run(&self, input: &str) -> String {
        let tokens = tokenize(input);
        let term = parser(&tokens, 0);
        let mut output = String::new();
        if term.is_err() {
            let err = parse_error_to_string(term.err().unwrap(), input);
            output.push_str(err.as_str());
            return output;
        }

        let (t, _) = term.unwrap();
        output.push_str(format!("{:?}\n", t).as_str());
        output.push_str(format!("consts: {:?}\n", consts(&t)).as_str());
        output.push_str(format!("size: {:?}\n", size(&t)).as_str());
        output.push_str(format!("depth: {:?}\n", depth(&t)).as_str());

        let result = eval(&t);
        if result.is_err() {
            let err = eval_error_to_string(result.err().unwrap());
            output.push_str(err.as_str());
            return output;
        }

        output.push_str("evaluation result: ");
        output.push_str(format!("{:?}\n", result.unwrap()).as_str());
        output
    }
    fn __debug__(&self) -> String {
        format!("{:?}", self)
    }
    fn name(&self) -> String {
        "Untyped Arithmetic".to_string()
    }
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

fn eval(term: &Term) -> Result<Term, EvalError> {
    let mut t = term.clone();
    let mut prev = term.clone();
    loop {
        if isval(&t) {
            return Ok(t.clone());
        }
        t = little_step_eval(&t)?;
        if prev == t {
            return Err(eval_err("No rule applies", &t));
        }
        prev = t.clone();
    }
}

fn little_step_eval(term: &Term) -> Result<Term, EvalError> {
    let t = match term {
        Term::If(t1, t2, t3) => match **t1 {
            Term::True => *t2.clone(),
            Term::False => *t3.clone(),
            _ => if_(little_step_eval(t1)?, *t2.clone(), *t3.clone()),
        },
        Term::Succ(t1) => succ(little_step_eval(t1)?),
        Term::Pred(t1) => {
            let t = *t1.clone();
            match t {
                Term::Zero => Term::Zero,
                Term::Succ(t2) if isnumericval(&t2) => *t2.clone(),
                _ => pred(little_step_eval(t1)?),
            }
        }
        Term::IsZero(t1) => {
            let t = *t1.clone();
            match t {
                Term::Zero => Term::True,
                Term::Succ(t2) if isnumericval(&t2) => Term::False,
                _ => iszero(little_step_eval(t1)?),
            }
        }
        Term::True | Term::False | Term::Zero => term.clone(),
    };
    Ok(t)
}

fn isval(term: &Term) -> bool {
    match term {
        Term::True | Term::False | Term::Zero => true,
        t if isnumericval(t) => true,
        _ => false,
    }
}

fn isnumericval(term: &Term) -> bool {
    match term {
        Term::Zero => true,
        Term::Succ(t1) => isnumericval(t1),
        _ => false,
    }
}

#[derive(Debug)]
struct EvalError {
    message: String,
    term: Term,
}

fn eval_err(message: &str, term: &Term) -> EvalError {
    EvalError {
        message: message.to_string(),
        term: term.clone(),
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
#[derive(Debug, Clone, PartialEq)]
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



fn parse_error_to_string(e: ParseError, input: &str) -> String {
    let mut out = String::new();
    out.push_str(format!("Error: {}\n", e.message).as_str());
    let tokens = tokenize(input);
    out.push_str(format!("{}{}", CYAN, tokens[..e.offset].join(" ")).as_str());
    if e.offset > 0 {
        out.push_str(" ");
    }
    out.push_str(format!("{}{}{}\n", RED, tokens[e.offset], RESET).as_str());
    for i in 0..e.offset {
        let spaces = tokens[i].len();
        out.push_str(format!("{:width$} ", "", width = spaces).as_str());
    }
    // ^^^ for the length of the token
    out.push_str(format!("{}{}", RED, "^".repeat(tokens[e.offset].len())).as_str());
    out.push_str(RESET);
    out.push_str("\n");
    out
}

fn err(message: &str, offset: usize) -> ParseError {
    ParseError {
        message: message.to_string(),
        offset,
    }
}

fn eval_error_to_string(e: EvalError) -> String {
    let mut out = String::new();
    out.push_str(format!("Error: {}\n", e.message).as_str());
    out.push_str(format!("{:?}\n", e.term).as_str());
    out
}
