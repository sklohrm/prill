use std::{
    env,
    io::{self, Write},
    process,
};

use crate::calc::Engine;

mod calc;

fn main() {
    let mut engine = Engine::new();
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // cli
        print!("CLI Coming Soon!");
        return;
    }
    // repl
    loop {
        print!("prill> ");
        io::stdout().flush().unwrap();
        execute(&mut engine);
    }
}

fn execute(engine: &mut Engine) {
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        println!("Error: {input}")
    }

    let input = input.trim();
    match parse_input(input) {
        Action::EvalLine(line) => match engine.eval_line(&line) {
            Ok(Some(result)) => {
                println!("=> {result}");
                println!()
            }
            Ok(None) => {}
            Err(e) => println!("Error: {e}"),
        },
        Action::Print => {
            let stack = engine.stack();
            if stack.is_empty() {
                println!("Memory is empty.")
            } else {
                print!("Memory:");
                for i in stack.iter().rev() {
                    print!("{i} ")
                }
                println!("\n")
            }
        }
        Action::Quit => process::exit(0),
    }
}

fn parse_input(input: &str) -> Action {
    match input.trim() {
        "p" => Action::Print,
        "q" => Action::Quit,
        other => Action::EvalLine(other.to_string()),
    }
}

enum Action {
    EvalLine(String),
    Print,
    Quit,
}
