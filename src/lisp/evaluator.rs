use crate::evaluator::Evaluator;

use super::tokenizer::tokenize;
use super::parser::parse;

use crate::parsing_utils::tokenize_error_to_string;
use crate::parsing_utils::parse_error_to_string;

#[derive(Debug)]
pub struct Lisp;

impl Evaluator for Lisp {
    fn run(&self, input: &str) -> String {
        let mut out = String::new();
        out.push_str("input: ");
        out.push_str(input);
        let tokens = tokenize(input).map_err(|e| tokenize_error_to_string(e, input));
        if tokens.is_err() {
            return tokens.unwrap_err();
        }
        let tokens = tokens.unwrap();

        out.push_str("\ntokens: [");
        for token in tokens.iter() {
            out.push_str(&format!("\n\t{}, ", token));
        }
        out.push_str("\n]\n");

        let expr = parse(&tokens);
        if expr.is_err() {
            let err = parse_error_to_string(expr.err().unwrap(), input);
            out.push_str(err.as_str());
            return out;
        }

        let expr = expr.unwrap();

        out.push_str("\nparsed: ");
        out.push_str(&format!("{:#?}", expr));


        return out;
    }

    fn __debug__(&self) -> String {
        format!("{:?}", self)
    }

    fn name(&self) -> String {
        "Lisp".to_string()
    }
}


