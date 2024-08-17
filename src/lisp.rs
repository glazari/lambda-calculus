use crate::evaluator::Evaluator;

use std::fmt::{self, Display, Formatter};
use crate::parsing_utils::SToken;
use crate::parsing_utils::TokenizeError;


#[derive(Debug)]
pub struct Lisp;

impl Evaluator for Lisp {
    fn run(&self, input: &str) -> String {
        let mut out = String::new();
        out.push_str("input: ");
        out.push_str(input);


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
enum Token {}



fn tokenize (input: &str) -> Result<Vec<SToken<Token>>, TokenizeError> {
    return Ok(vec![]);
}


