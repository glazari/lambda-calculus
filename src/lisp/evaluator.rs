use crate::evaluator::Evaluator;

use super::tokenizer::tokenize;

use crate::parsing_utils::tokenize_error_to_string;

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

        out.push_str("\ntokens: [");
        for token in tokens.unwrap() {
            out.push_str(&format!("\n\t{}, ", token));
        }
        out.push_str("\n]\n");

        return out;
    }

    fn __debug__(&self) -> String {
        format!("{:?}", self)
    }

    fn name(&self) -> String {
        "Lisp".to_string()
    }
}


