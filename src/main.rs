
mod lexer;
mod input;
use lexer::Lexer;

fn main() {
    println!("Hello, this is just here because I suspect it needs to be.");

    let mut lexer = Lexer::new("  123  ".chars());
    loop {
        match lexer.next() {
            Some(t) => println!("Got token {:?}", t),
            None => {
                println!("End of input");
                break;
            }
        }
    }
}
 
