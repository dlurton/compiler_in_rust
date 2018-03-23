
use std::io::Write;

extern crate compiler_in_rust_lib;
use compiler_in_rust_lib::*;

fn main() {
    println!("Hello, enter an expression and I will evaluate it for you.  To view ast, type '?' as the first character of your expression.  To exit, type 'exit'.");

    while {
        print!(">");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(bytes_read) => {
                if bytes_read == 1 { // Only read newline
                    true
                } else {
                    if input.starts_with("?") {
                        println!("{:#?}", parse(&input[1..]));
                        true
                    } else {
                        match input[..].trim() {
                            "exit" => false,
                            input => {
                                let result = execute(&input);
                                println!("{:?}", result);
                                true
                            }
                        }
                    }
                }
            },
            Err(err) => {
                println!("error reading from standard input: {}", err);
                false
            }
        }

    } { }
} 
