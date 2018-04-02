
pub mod resolve_variables;
pub mod evaluate;

use ast::*;
use value::*;
use error::*;
use common::*;
use env::*;

/// This is where all rewrite pass error kinds should go.
/// If there is more than 2(ish) different errors from a given pass, please
/// create a `ErrorKind` enum for that pass and implement `ErrorKind`
/// to reduce coupling.
#[derive(Debug, Clone, PartialEq)]
pub enum PassErrorKind {
    VariableDoesNotExist { variable_name: String }
}

impl ErrorKind for PassErrorKind {
    fn message(&self) -> String {
        match self {
            &PassErrorKind::VariableDoesNotExist{ref variable_name} => format!("Variable '{}' does not exist", variable_name)
        }
    }
}
type PassError = SourceError<PassErrorKind>;
pub type PassResult = Result<Expr, PassError>;


/// Recurses over `expr`, invoking `node_handler` for each node, which can return Some(Expr) if the node is to be rewritten.
/// If `node_handler` returns None, returns a deep clone of the current node instead.
fn recurse_clone<TErrorKind: ErrorKind>(expr: &Expr, node_handler: &Fn(&Expr) -> Option<Result<Expr, SourceError<TErrorKind>>>) -> Result<Expr, SourceError<TErrorKind>> {
    match node_handler(expr) {
        Some(e) => e,
        None => match &expr.kind {
            &ExprKind::Literal{ value: _ } |
            &ExprKind::VariableRef { name: _ } |
            &ExprKind::VariableIndex { index: _ }
            => Ok((*expr).clone()),

            &ExprKind::Binary { ref op, ref left, ref right } => {
                let result = recurse_clone(left, node_handler);
                let new_left = match result {
                    Err(e) => return Err(e),
                    Ok(expr) => expr
                };

                let result = recurse_clone(right, node_handler);
                let new_right = match result {
                    Err(e) => return Err(e),
                    Ok(expr) => expr
                };

                Ok(expr.clone_with(ExprKind::Binary {
                    op: (*op).clone(),
                    left: Box::new(new_left),
                    right: Box::new(new_right)
                }))
            }
        }
    }
}

