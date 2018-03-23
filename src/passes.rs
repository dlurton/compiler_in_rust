
use ast::*;
use value::*;

//pub fn get_evaluator(globals: Env) f

fn recurse_clone(expr: &Expr, node_handler: &Fn(&Expr) -> Option<Expr>) -> Expr {
    match node_handler(expr) {
        Some(e) => e,
        None => match &expr.kind {
            &ExprKind::Literal(_) | &ExprKind::VariableRef(_) | &ExprKind::VariableIndex(_) => (*expr).clone(),
            &ExprKind::Binary(ref op, ref left, ref right) => {
                let new_left = recurse_clone(left, node_handler);
                let new_right = recurse_clone(right, node_handler);
                expr.clone_with(ExprKind::Binary((*op).clone(), Box::new(new_left), Box::new(new_right)))
            }
        }
    }
}

pub fn resolve_indexes(expr: Expr, global_def: &EnvDef) -> Expr {
    recurse_clone(
        &expr,
        &|expr: &Expr| {
            match expr.kind {
                ExprKind::VariableRef(ref name) => {
                    Some(match global_def.find(&name[..]) {
                        Some(field) => expr.clone_with(ExprKind::VariableIndex(field.ordinal)),
                        None => panic!("Variable {:?} does not exist.", name)
                    })
                }
                _ => None
            }
        })
}

pub fn evaluate(expr: &Expr, env: &Vec<Value>) -> Value {
    match expr.kind {
        ExprKind::Literal(ref v) => v.clone(),
        ExprKind::VariableIndex(ref index) => match env.get(*index as usize) {
            Some(value) => (*value).clone(),
            None => panic!("Index {:?} was out of range?", index)
        },
        ExprKind::VariableRef(ref name) => panic!("Unresolved variable reference: {:?}", name),
        ExprKind::Binary(ref op, ref left, ref right) => {
            match (op, evaluate(&left, env), evaluate(&right, env)) {
                (&BinaryOp::Add, Value::Int32(l), Value::Int32(r)) => Value::Int32(l + r),
                (&BinaryOp::Sub, Value::Int32(l), Value::Int32(r)) => Value::Int32(l - r),
                (&BinaryOp::Mul, Value::Int32(l), Value::Int32(r)) => Value::Int32(l * r),
                (&BinaryOp::Div, Value::Int32(l), Value::Int32(r)) => Value::Int32(l / r),
                (&BinaryOp::Mod, Value::Int32(l), Value::Int32(r)) => Value::Int32(l % r),

                (_, Value::Tuple(_), _) | (_, _, Value::Tuple(_)) => panic!("Cannot perform binary operations on tuples"),
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

    fn eval(expr: &Expr) -> Value {
        let emptyEnv = Vec::new();
        evaluate(expr, &emptyEnv)
    }

    #[test]
    fn test_add() {
        assert_eq!(
            Value::Int32(2),
            eval(&Expr::new(ExprKind::Binary(BinaryOp::Add, lit_int32(1), lit_int32(1)))));
    }

    #[test]
    fn test_sub() {
        assert_eq!(
            Value::Int32(0),
            eval(&Expr::new(ExprKind::Binary(BinaryOp::Sub, lit_int32(1), lit_int32(1)))));
    }

    #[test]
    fn test_mul() {
        assert_eq!(
            Value::Int32(10),
            eval(&Expr::new(ExprKind::Binary(BinaryOp::Mul, lit_int32(2), lit_int32(5)))));
    }

    #[test]
    fn test_div() {
        assert_eq!(
            Value::Int32(5),
            eval(&Expr::new(ExprKind::Binary(BinaryOp::Div, lit_int32(10), lit_int32(2)))));
    }

    #[test]
    fn test_mod() {
        assert_eq!(
            Value::Int32(1),
            eval(&Expr::new(ExprKind::Binary(BinaryOp::Mod, lit_int32(7), lit_int32(3)))));
    }
}
