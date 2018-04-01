
pub mod source;
pub mod input;
pub mod lexer;
pub mod value;
pub mod ast;
pub mod parser;
pub mod passes;
pub mod error;
pub mod env;
pub mod common;

use lexer::*;
use parser::*;
use passes::*;
use value::*;
use error::*;
use env::*;

pub fn execute(source: &str) -> ExecuteResult {
    let empty_env = EnvDefBuilder::new().build();
    execute_with_globals(source, &empty_env)
}

pub fn parse(source: &str) -> ParseResult {
    let lexer = Lexer::new(source.chars());
    let mut parser = Parser::new(lexer);
    parser.parse()
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExecuteErrorKind {
    Parse(ParseErrorKind),
    Pass(PassErrorKind),
    Evaluate(EvaluateErrorKind)
}

impl ErrorKind for ExecuteErrorKind {
    fn message(&self) -> String {
        match self {
            &ExecuteErrorKind::Parse(ref kind) => kind.message(),
            &ExecuteErrorKind::Evaluate(ref kind) => kind.message(),
            &ExecuteErrorKind::Pass(ref kind) => kind.message()
        }
    }
}

pub type ExecuteError = SourceError<ExecuteErrorKind>;
pub type ExecuteResult = Result<Value, ExecuteError>;

pub fn execute_with_globals(source: &str, global_env_def: &EnvDef) -> ExecuteResult {
    match parse(source) {
        Ok(ast) =>
            match resolve_variables(ast, &global_env_def) {
                Ok(ast) => {
                    let global_env = global_env_def.create_with_default_values();
                    match evaluate(&ast, &global_env) {
                        Ok(value) => Ok(value),
                        Err(err) => Err(ExecuteError::new_with_span(ExecuteErrorKind::Evaluate(err.kind), err.span))
                    }
                },
                Err(pass_err) => {
                    Err(ExecuteError::new_with_span(ExecuteErrorKind::Pass(pass_err.kind), pass_err.span))
                }
            },
        Err(parse_error) => {
            Err(ExecuteError::new_with_span(ExecuteErrorKind::Parse(parse_error.kind), parse_error.span))
        }
   }

}

