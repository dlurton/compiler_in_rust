
pub mod source;
pub mod input;
pub mod lexer;
pub mod value;
pub mod ast;
pub mod parser;
pub mod passes;
pub mod error;

use ast::*;
use lexer::*;
use parser::*;
use passes::*;
use value::*;
use error::*;

pub fn execute(source: &str) -> Value {
    let empty_env = EnvDefBuilder::new().build();
    execute_with_globals(source, &empty_env)
}

pub fn parse(source: &str) -> Expr {
    let lexer = Lexer::new(source.chars());
    let err_stream = ErrorStream::new(&mut std::io::stdout());
    let mut parser = Parser::new(lexer, err_stream);
    parser.parse()
}

pub fn execute_with_globals(source: &str, global_env_def: &EnvDef) -> Value {
    let ast = parse(source);
    let ast = resolve_variables(ast, &global_env_def);

    let global_env = global_env_def.create_with_default_values();
    let result = evaluate(&ast, &global_env);

    return result;
}

