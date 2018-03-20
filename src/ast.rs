
use source::Span;
use value::Value;

#[derive(Debug, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod
}

#[derive(Debug, PartialEq)]
pub enum ExprKind {
    Literal(Value),
    Binary(BinaryOp, Box<Expr>, Box<Expr>),
}

#[derive(Debug)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span
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

impl Expr {
    pub fn new(kind: ExprKind) -> Expr {
        Expr { kind, span: Span::unknown() }
    }

    pub fn new_with_span(kind: ExprKind, span: Span) -> Expr {
        Expr { kind, span }
    }
}

