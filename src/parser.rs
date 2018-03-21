
use source::*;
use lexer::*;
use value::*;
use ast::*;

// http://en.cppreference.com/w/cpp/language/operator_precedence

// https://keepcalmandlearnrust.com/2016/08/pratt-parser-in-rust/

fn get_precedence(token_kind: &TokenKind) -> u32 {
    match token_kind {
        &TokenKind::OperatorAdd | &TokenKind::OperatorSub => 10,
        &TokenKind::OperatorMul | &TokenKind::OperatorDiv => 20,
        _ => 0
    }
}
pub struct Parser<'a> {
    lexer: Lexer<'a>
}

impl <'a> Parser<'a> {
    pub fn new(lexer: Lexer) -> Parser {
        Parser { lexer }
    }

 
    pub fn parse(&mut self) -> Expr {
        self.parse_expr(0)
    }

    fn parse_expr(&mut self, precedence: u32) -> Expr {

        let mut expr = self.parse_prefix();

        while let Some(next_token) = self.lexer.peek() {
            let next_precedence = get_precedence(&next_token.kind);

            if precedence >= next_precedence {
                break;
            }

            expr = self.parse_infix(expr, next_precedence);
        }
        expr
    }

    fn parse_prefix(&mut self) -> Expr {
        match self.lexer.next() {
            None => panic!("Unexpected end of tokens!"),
            Some(token) => match token.kind {
                // TODO: make Literal a token kind, introduce enum LiteralKind
                // maybe we can get some of those cool compiler errors?
                TokenKind::LiteralInt32(value) =>
                    Expr::new_with_span(ExprKind::Literal(Value::Int32(value)), token.span),
                _ => panic!("Invalid prefix expression term!")
            }
        }
    }

    fn parse_infix(&mut self, left: Expr, precedence: u32) -> Expr {
        match self.lexer.next() {
            None => panic!("Unexpected end of tokens!"),
            Some(token) => {
                // TODO: make Operator a token kind, introduce enum OperatorKind?
                // maybe we can get some of those cool compiler errors?
                let binary_op = match token.kind {
                    TokenKind::OperatorAdd => BinaryOp::Add,
                    TokenKind::OperatorSub => BinaryOp::Sub,
                    TokenKind::OperatorMul => BinaryOp::Mul,
                    TokenKind::OperatorDiv => BinaryOp::Div,
                    TokenKind::OperatorMod => BinaryOp::Mod,
                    _ => panic!("Invalid binary operator!")
                };
                let right = self.parse_expr(precedence);
                let span = Span::new(left.span.start.clone(), right.span.end.clone());
                Expr::new_with_span(ExprKind::Binary(binary_op, Box::new(left), Box::new(right)), span)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(src: &str) -> Expr {
        let lexer = Lexer::new(src.chars());
        let mut parser = Parser::new(lexer);
        parser.parse()
    }

    #[test]
    pub fn parse_literals() {
        assert_eq!(Expr::new(ExprKind::Literal(Value::Int32(1))), parse("1"));
        assert_eq!(Expr::new(ExprKind::Literal(Value::Int32(123))), parse("  123  "));
    }

    #[test]
    pub fn parse_literal_binary_add() {
        assert_eq!(
            Expr::new(
                ExprKind::Binary(
                    BinaryOp::Add,
                    Box::new(Expr::new(ExprKind::Literal(Value::Int32(1)))),
                    Box::new(Expr::new(ExprKind::Literal(Value::Int32(1)))))),
            parse("1+1")
        )
    }

    #[test]
    pub fn parse_literal_binary_mul() {
        assert_eq!(
            Expr::new(
                ExprKind::Binary(
                    BinaryOp::Mul,
                    Box::new(Expr::new(ExprKind::Literal(Value::Int32(1)))),
                    Box::new(Expr::new(ExprKind::Literal(Value::Int32(1)))))),
            parse("1*1")
        )
    }

    #[test]
    pub fn parse_literal_binary_add_mul() {
        assert_eq!(
            Expr::new(
                ExprKind::Binary(
                    BinaryOp::Add,
                    Box::new(Expr::new(ExprKind::Literal(Value::Int32(1)))),
                    Box::new(Expr::new(
                        ExprKind::Binary(
                            BinaryOp::Mul,
                            Box::new(Expr::new(ExprKind::Literal(Value::Int32(2)))),
                            Box::new(Expr::new(ExprKind::Literal(Value::Int32(3))))))))),
            parse("1+2*3")
        )
    }
}
