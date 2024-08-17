mod app;
mod colors;
mod errors;
mod tui;
mod untyped_arithmetic;
mod untyped_lambda_calculus;
mod parsing_utils;
mod lisp;
mod evaluator;

use colors::*;
use std::io::{self, Write};

use color_eyre::Result;
use evaluator::Evaluator;





fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "tui" {
        app::app_main()?;
    } else {
        main2();
    }
    Ok(())
}

fn main2() {
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
        let out = evaluator.run(input);
        println!("{}", out);
    }
}

fn pick_evaluator() -> Box<dyn Evaluator> {
    let evaluators = evaluator::available_evaluators();
    println!("{}Pick an evaluator:{}", CYAN, RESET);
    for (i, evaluator) in evaluators.iter().enumerate() {
        println!("{}{}.{} {}", GREEN, i + 1, RESET, evaluator.name());
    }
    loop {
        print_prompt();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        let index = input.parse::<usize>();
        if let Ok(index) = index {
            let eval = evaluator::pick(index);
            if let Ok(eval) = eval {
                return eval;
            }
        }
        println!("Invalid evaluator. Try again.");
    }
}

fn print_prompt() {
    print!("{}{}{}", GREEN, PROMPT, RESET);
    io::stdout().flush().unwrap();
}
