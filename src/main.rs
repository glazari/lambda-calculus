mod colors;
mod untyped_arithmetic;

use colors::*;
use std::io::{self, Write};
use untyped_arithmetic::UntypedArithmetic;

trait Evaluator {
    fn run(&self, input: &str);
}

fn main() {
    let mut evaluator = pick_evaluator();
    println!("Lambda REPL");
    loop {
        print_prompt();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input == ":q" {
            break;
        }
        if input == "change" {
            evaluator = pick_evaluator();
            continue;
        }
        evaluator.run(input);
    }
}

fn pick_evaluator() -> Box<dyn Evaluator> {
    println!("{}Pick an evaluator:{}", CYAN, RESET);
    println!("{}1.{} Untyped Arithmetic", GREEN, RESET);
    loop {
        print_prompt();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        match input {
            "1" => return Box::new(UntypedArithmetic),
            _ => println!("Invalid evaluator. Try again."),
        }
    }
}

fn print_prompt() {
    print!("{}{}{}", GREEN, PROMPT, RESET);
    io::stdout().flush().unwrap();
}
