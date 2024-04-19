mod parser;
mod tokenizer;

use std::io;
use parser::Parser;

fn main() {
    loop {
        let input = get_input();

        if input == "q\n" {
            break;
        }

        let mut computation = Parser::new(input.trim());
        println!("result: {}\n", computation.parse_computation());
    }
}

fn get_input() -> String {
    // this includes \n whenever you ENTER
    let mut input = String::new();
    println!("Enter an expression: ");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the line.");

    input
}
