
pub mod source;
pub mod input;
pub mod lexer;
pub mod value;
pub mod ast;
pub mod parser;
pub mod passes;

use ast::*;
use lexer::*;
use parser::*;
use passes::*;
use value::*;
use std::vec::*;

pub fn execute(source: &str) -> Value {
    let empty_env = Vec::new();
    execute_with_globals(source, empty_env)
}

pub fn parse(source: &str) -> Expr {
    let lexer = Lexer::new(source.chars());
    let mut parser = Parser::new(lexer);
    parser.parse()
}

pub fn execute_with_globals(source: &str, global_env: Vec<Value>) -> Value {
    let ast = parse(source);
    let result = evaluate(&ast, &global_env);
    return result;
}

