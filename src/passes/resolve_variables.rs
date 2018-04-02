use ast::*;
use env::*;

use super::*; 

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
                                PassErrorKind::VariableDoesNotExist { variable_name: name.clone() },
                                expr.span.clone()))
                    }),
                _ => None
            }
        })
}
