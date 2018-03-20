
use ast::*;
use value::*;

pub fn evaluate(expr: &Expr) -> Value {
    match expr.kind {
        ExprKind::Literal(ref v) => v.clone(),
        ExprKind::Binary(ref op, ref left, ref right) => {
            match (op, evaluate(&left), evaluate(&right)) {
               (&BinaryOp::Add, Value::Int32(l), Value::Int32(r)) => Value::Int32(l + r),
               (&BinaryOp::Sub, Value::Int32(l), Value::Int32(r)) => Value::Int32(l - r),
               (&BinaryOp::Mul, Value::Int32(l), Value::Int32(r)) => Value::Int32(l * r),
               (&BinaryOp::Div, Value::Int32(l), Value::Int32(r)) => Value::Int32(l / r),
               (&BinaryOp::Mod, Value::Int32(l), Value::Int32(r)) => Value::Int32(l % r),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lit_int32(i: i32) -> Box<Expr> {
        Box::new(Expr::new(ExprKind::Literal(Value::Int32(i))))
    }

    #[test]
    fn test_add() {
        assert_eq!(
            Value::Int32(2),
            evaluate(&Expr::new(ExprKind::Binary(BinaryOp::Add, lit_int32(1), lit_int32(1)))));
    }

    #[test]
    fn test_sub() {
        assert_eq!(
            Value::Int32(0),
            evaluate(&Expr::new(ExprKind::Binary(BinaryOp::Sub, lit_int32(1), lit_int32(1)))));
    }

    #[test]
    fn test_mul() {
        assert_eq!(
            Value::Int32(10),
            evaluate(&Expr::new(ExprKind::Binary(BinaryOp::Mul, lit_int32(2), lit_int32(5)))));
    }

    #[test]
    fn test_div() {
        assert_eq!(
            Value::Int32(5),
            evaluate(&Expr::new(ExprKind::Binary(BinaryOp::Div, lit_int32(10), lit_int32(2)))));
    }

    #[test]
    fn test_mod() {
        assert_eq!(
            Value::Int32(1),
            evaluate(&Expr::new(ExprKind::Binary(BinaryOp::Mod, lit_int32(7), lit_int32(3)))));
    }
}
