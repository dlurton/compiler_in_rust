
use std::io::Write;

extern crate compiler_in_rust_lib;
use compiler_in_rust_lib::*;
use error::*; 

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

                        match parse(&input[1..]) {
                            Ok(expr) => println!("{:#?}", expr),
                            Err(e) => println!("Error: {}", e.kind().message())
                        }
                        true
                    } else {
                        match input[..].trim() {
                            "exit" => false,
                            input => {
                                let result = execute(&input);
                                match result {
                                    Err(e) => println!("Error: {}", e.kind().message()),
                                    Ok(v) => println!("Result: {:?}", v)
                                }
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
