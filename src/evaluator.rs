use std::fmt::{self, Debug, Formatter};

use crate::untyped_arithmetic::UntypedArithmetic;
use crate::untyped_lambda_calculus::UntypedLambdaCalculus;
use crate::lisp::Lisp;
use color_eyre::eyre::Result;

pub trait Evaluator {
    fn run(&self, input: &str) -> String;
    fn __debug__(&self) -> String;
    fn name(&self) -> String;
}

impl Debug for dyn Evaluator {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Evaluator")
    }
}

pub fn available_evaluators() -> Vec<Box<dyn Evaluator>> {
    vec![Box::new(UntypedArithmetic), Box::new(UntypedLambdaCalculus), Box::new(Lisp)]
}

pub fn pick(index: usize) -> Result<Box<dyn Evaluator>> {
    match index {
        1 => return Ok(Box::new(UntypedArithmetic)),
        2 => return Ok(Box::new(UntypedLambdaCalculus)),
        3 => return Ok(Box::new(Lisp)),
        _ => return Err(color_eyre::eyre::eyre!("Invalid evaluator index")),
    };
}
