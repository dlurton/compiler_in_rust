
pub mod source;
pub mod input;
pub mod lexer;
pub mod value;
pub mod ast;
pub mod parser;
pub mod evaluator;


use lexer::*;
use parser::*;
use evaluator::*;
use value::*;

pub fn execute(source: &str) -> Value {
    let lexer = Lexer::new(source.chars());
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();
    let result = evaluate(&ast);
    return result;
}



