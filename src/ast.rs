
use source::Span;
use value::Value;

// https://github.com/rust-unofficial/patterns/blob/master/patterns/visitor.md


#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod
}

#[derive(Debug)]
pub enum ExprKind {
    Literal(Value),
    Binary(BinaryOp, Box<Expr>, Box<Expr>),
}

#[derive(Debug)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span
}

impl Expr {
    pub fn new(kind: ExprKind) -> Expr {
        Expr { kind, span: Span::unknown() }
    }
}

/*
pub struct BinaryExpr {
    left: Box<Expr>
}
*/
