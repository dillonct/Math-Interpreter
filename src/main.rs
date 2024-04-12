use std::io;
use std::iter::Peekable;
use std::str::Chars;

fn main() {
    loop {
        let input = get_input();

        if input == "q\n" {
            break;
        }

        let mut input = input.trim().chars().peekable();

        while input.peek().is_some() {
            let result = parse_result(&mut input);
            println!("result: {}", result);
        }

        println!();
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

fn parse_result(iterator: &mut Peekable<Chars>) -> i32 {
    let val = parse_expression(iterator);
    match_char(iterator, '.');

    val
}
fn parse_expression(iterator: &mut Peekable<Chars>) -> i32 {
    let mut val = parse_term(iterator);
    loop {
        let c = get_char(iterator.peek());
        match c {
            '+' => {
                iterator.next();
                val += parse_term(iterator);
                // next_skip(iterator);
            }
            '-' => {
                iterator.next();
                val -= parse_term(iterator);
                // next_skip(iterator);
            }
            ' ' => {
                next_skip(iterator);
            }
            _ => {
                break;
            }
        }
    }

    val
}

fn parse_term(iterator: &mut Peekable<Chars>) -> i32 {
    let mut val = parse_factor(iterator);
    loop {
        let c = get_char(iterator.peek());
        match c {
            '*' => {
                iterator.next();
                val *= parse_factor(iterator);
                // next_skip(iterator);
            }
            '/' => {
                iterator.next();
                val /= parse_factor(iterator);
                // next_skip(iterator);
            }
            ' ' => {
                next_skip(iterator);
            }
            _ => {
                break;
            }
        }
    }

    val
}

fn parse_factor(iterator: &mut Peekable<Chars>) -> i32 {
    let val;

    next_skip(iterator);

    let c = get_char(iterator.peek());

    if c.is_digit(10) {
        val = parse_number(iterator);
        // iterator.next();
    } else if c == '(' {
        iterator.next();
        val = parse_expression(iterator);
        match_char(iterator, ')');
    } else {
        panic!("this shit dont work"); // fix this later
    }

    val
}

fn parse_number(iterator: &mut Peekable<Chars>) -> i32 {
    let mut val = 0;
    loop {
        let c = get_char(iterator.peek());
        if !c.is_digit(10) {
            break;
        }

        val *= 10;
        val += c.to_digit(10).expect("Cannot convert character to digit") as i32;
        iterator.next();
    }

    val
}

fn match_char(iterator: &mut Peekable<Chars>, char_to_match: char) -> bool {
    if get_char(iterator.peek()) == char_to_match {
        iterator.next();
        return true;
    }

    panic!("Does not close {}", char_to_match);
}

fn get_char(option_value: Option<&char>) -> char {
    match option_value {
        Some(&char_value) => char_value,
        None => '\0',
    }
}

fn next_skip(iterator: &mut Peekable<Chars>) {
    loop {
        let c = get_char(iterator.peek());
        if c == ' ' {
            iterator.next();
        } else {
            break;
        }
    }
}
