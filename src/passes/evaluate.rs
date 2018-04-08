
use ast::*;
use super::*;


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
        //This would be a bug.
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
            })
        }
        ExprKind::CompoundExpr { ref exprs } => {
            //Iterate over all expressions except the last, discarding the result.
            for expr in &exprs[0..exprs.len() - 2] {
                if let Err(e) = evaluate(expr, env) {
                    return Err(e)
                }
            }
            //Evalute final expression, which is the result of the CompoundExpr
            evaluate(&exprs[exprs.len() - 1], env)
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
