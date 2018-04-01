
use source::Span;
use value::Value;
use common::*;

//TODO experiment with struct enum variants here
#[derive(Debug, PartialEq, Clone)]
pub enum ExprKind {
    Literal{ value: Value },
    Binary{ op: BinaryOp, left: Box<Expr>, right: Box<Expr> },
    VariableRef { name: String },
    VariableIndex { index: u32 }
}

#[derive(Debug, Clone)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span
}

impl Expr {
    #[cfg(test)]
    pub fn new(kind: ExprKind) -> Expr {
        Expr { kind, span: Span::unknown() }
    }

    #[cfg(test)]
    pub fn new_literal(value: Value) -> Expr {
        Expr::new_literal_with_span(value, Span::unknown())
    }
    pub fn new_literal_with_span(value: Value, span: Span) -> Expr {
        Expr::new_with_span(ExprKind::Literal { value }, span)
    }

    #[cfg(test)]
    pub fn new_binary(op: BinaryOp, left: Expr, right: Expr) -> Expr {
        Expr::new_binary_with_span(op, left, right, Span::unknown())
    }
    pub fn new_binary_with_span(op: BinaryOp, left: Expr, right: Expr, span: Span) -> Expr {
        Expr::new_with_span(ExprKind::Binary {
            op: op,
            left: Box::new(left),
            right: Box::new(right)
        }, span)
    }

    #[cfg(test)]
    pub fn new_variable_ref(name: String, span: Span) -> Expr {
        Expr::new_variable_ref_with_span(name, span)
    }
    pub fn new_variable_ref_with_span(name: String, span: Span) -> Expr {
        Expr::new_with_span(ExprKind::VariableRef { name }, span)
    }

    #[cfg(test)]
    pub fn new_variable_index(index: u32) -> Expr {
        Expr::new_variable_index_with_span(index, Span::unknown())
    }
    pub fn new_variable_index_with_span(index: u32 , span: Span) -> Expr {
        Expr::new_with_span(ExprKind::VariableIndex{ index }, span)
    }

    pub fn new_with_span(kind: ExprKind, span: Span) -> Expr {
        Expr { kind, span }
    }

    pub fn clone_with(&self, kind: ExprKind) -> Expr {
        Expr { kind: kind, span: self.span.clone() }
    }
}

/// The main reason for implementing this manually is to prevent the span
/// from being part of the equality comparison...  Specifying the span in
/// unit tests is a pain.
impl PartialEq for Expr {
    fn eq(&self, other: &Expr) -> bool {
        self.kind == other.kind
    }
    fn ne(&self, other: &Expr) -> bool {
        self.kind != other.kind
    }
}

