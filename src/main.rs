mod brainfuck;

use std::env;
use std::fs::read_to_string;
use std::io::{stdin, stdout, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut input: String = String::new();

    if args.len() > 1 {
        input = read_to_string(&args[1]).unwrap();
    } else {
        print!("Input brainfuck: ");
        stdout().flush().unwrap();
        if stdin().read_line(&mut input).is_err() {
            panic!("Something went wrong!");
        }
    }

    brainfuck::evaluate(input);
}
