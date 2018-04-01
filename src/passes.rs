
use ast::*;
use value::*;
use error::*;
use common::*; 
//pub fn get_evaluator(globals: Env)

// TODO:  don't put all possible transformation pass error codes here.
// Would be nicer to keep error codes from each pass distinct.

#[derive(Debug, Clone, PartialEq)]
pub enum PassErrorKind {
    VariableDoesNotExist(String)
}

impl ErrorKind for PassErrorKind {
    fn message(&self) -> String {
        match self {
            &PassErrorKind::VariableDoesNotExist(ref name) => format!("Variable '{}' does not exist", name)
        }
    }
}

type PassError = SourceError<PassErrorKind>;
pub type PassResult = Result<Expr, PassError>;

fn recurse_clone(expr: &Expr, node_handler: &Fn(&Expr) -> Option<PassResult>) -> PassResult {
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

pub fn resolve_variables(expr: Expr, global_def: &EnvDef) -> PassResult {
    recurse_clone(
        &expr,
        &|expr: &Expr| {
            let kind = &expr.kind;
            match kind {
                &ExprKind::VariableRef { ref name } =>
                    Some(match global_def.find(&name[..]) {
                        Some(field) => Ok(Expr::new_variable_index_with_span(field.ordinal, expr.span)),
                        None => Err(
                            PassError::new_with_span(
                                PassErrorKind::VariableDoesNotExist(name.clone()),
                                expr.span.clone()))
                    }),
                _ => None
            }
        })
}

#[derive(Debug, Clone, PartialEq)]
pub enum EvaluateErrorKind {
    IndexOutOfRange(u32)
}

impl ErrorKind for EvaluateErrorKind {
    fn message(&self) -> String {
        match self {
            &EvaluateErrorKind::IndexOutOfRange(index) => format!("Index {} was out of range.", index)
        }
    }
}

pub type EvaluateError = SourceError<EvaluateErrorKind>;
pub type EvaluateResult = Result<Value, EvaluateError>;


pub fn evaluate(expr: &Expr, env: &Env) -> EvaluateResult {
    match expr.kind {
        ExprKind::Literal { ref value } => Ok(value.clone()),
        ExprKind::VariableIndex{ ref index } => match env.get_by_index(*index) {
            Some(value) => Ok((*value).clone()),
            None => Err(EvaluateError::new_with_span(
                EvaluateErrorKind::IndexOutOfRange(*index),
                expr.span)
            )
        },
        //This case indicates that the `resolve_variables` pass was not executed against `expr`
        ExprKind::VariableRef { ref name } => panic!("Unresolved variable reference: {:?}", name),
        ExprKind::Binary{ ref op, ref left, ref right } => {
            let left_value = match evaluate(&left, env) {
                Err(e) => return Err(e),
                Ok(value) => value
            };
            let right_value = match evaluate(&right, env) {
                Err(e) => return Err(e),
                Ok(value) => value
            };

            Ok(match (op, left_value, right_value) {
                (&BinaryOp::Add, Value::Int32(l), Value::Int32(r)) => Value::Int32(l + r),
                (&BinaryOp::Sub, Value::Int32(l), Value::Int32(r)) => Value::Int32(l - r),
                (&BinaryOp::Mul, Value::Int32(l), Value::Int32(r)) => Value::Int32(l * r),
                (&BinaryOp::Div, Value::Int32(l), Value::Int32(r)) => Value::Int32(l / r),
                (&BinaryOp::Mod, Value::Int32(l), Value::Int32(r)) => Value::Int32(l % r),

                (_, Value::Tuple(_), _) | (_, _, Value::Tuple(_)) => panic!("Cannot perform binary operations on tuples"),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lit_int32(value: i32) -> Expr {
        Expr::new_literal(Value::Int32(value))
    }

    fn eval(expr: &Expr) -> Value {
        let env = EnvDefBuilder::new().build();
        let empty = env.create_with_default_values();
        evaluate(expr, &empty).unwrap()
    }

    #[test]
    fn test_add() {
        assert_eq!(Value::Int32(2), eval(&Expr::new_binary(BinaryOp::Add, lit_int32(1), lit_int32(1))));
    }

    #[test]
    fn test_sub() {
        assert_eq!(Value::Int32(0), eval(&Expr::new_binary(BinaryOp::Sub, lit_int32(1), lit_int32(1))));
    }

    #[test]
    fn test_mul() {
        assert_eq!(Value::Int32(10), eval(&Expr::new_binary(BinaryOp::Mul, lit_int32(2), lit_int32(5))));
    }

    #[test]
    fn test_div() {
        assert_eq!(Value::Int32(5), eval(&Expr::new_binary(BinaryOp::Div, lit_int32(10), lit_int32(2))));
    }

    #[test]
    fn test_mod() {
        assert_eq!(Value::Int32(1), eval(&Expr::new_binary(BinaryOp::Mod, lit_int32(7), lit_int32(3))));
    }
}
