
use source::*;
use lexer::*;
use value::*;
use ast::*;
use error::*;

// http://en.cppreference.com/w/cpp/language/operator_precedence

// https://keepcalmandlearnrust.com/2016/08/pratt-parser-in-rust/

fn get_precedence(token_kind: &TokenKind) -> u32 {
    match token_kind {
        &TokenKind::OperatorAdd | &TokenKind::OperatorSub => 10,
        &TokenKind::OperatorMul | &TokenKind::OperatorDiv => 20,
        _ => 0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseErrorKind {
    LexerError(LexerErrorKind),
    ExpectedBinaryOperator(TokenKind),
    InvalidPrefixExpressionTerm(TokenKind),
    UnexpectedEndOfInput,
}

impl ErrorKind for ParseErrorKind {
    fn message(&self) -> String {
        match self {
            &ParseErrorKind::LexerError(ref le) => le.message(),
            &ParseErrorKind::ExpectedBinaryOperator(ref tok) => format!("Expected binary operator but found: {}", tok),
            &ParseErrorKind::InvalidPrefixExpressionTerm(ref tok) => format!("Invalid prefix expression term: {}", tok),
            &ParseErrorKind::UnexpectedEndOfInput => String::from("Unexpected end of input")
        }
    }
}

pub type ParseError = SourceError<ParseErrorKind>;
pub type ParseResult = Result<Expr, ParseError>;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl <'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Parser<'a> {
        Parser { lexer: lexer }
    }
 
    pub fn parse(&mut self) -> ParseResult {
        self.parse_expr(0)
    }

    fn parse_expr(&mut self, precedence: u32) -> ParseResult {
        let prefix_result = self.parse_prefix();
        match prefix_result {
            Err(_) => prefix_result,
            Ok(mut expr) => {
                while let LexResult::Ok(next_token) = self.lexer.peek() {
                    let next_precedence = get_precedence(&next_token.kind);

                    if precedence >= next_precedence {
                        break;
                    }

                    expr = match self.parse_infix(expr, next_precedence) {
                        Err(e) => return Err(e),
                        Ok(expr) => expr
                    }
                }
                Ok(expr)
            }
        }
    }

    fn parse_prefix(&mut self) -> ParseResult {
        match self.lexer.next() {
            LexResult::Err(lex_err) => Err(
                ParseError::new_with_span(ParseErrorKind::LexerError(lex_err.kind()), lex_err.span())),

            LexResult::EndOfInput(last_location) => Err(
                ParseError::new_with_location(
                    ParseErrorKind::UnexpectedEndOfInput,
                    last_location)),

            LexResult::Ok(token) =>
                match token.kind {
                    // TODO: make Literal a token kind, introduce enum LiteralKind
                    // maybe we can get some of those cool compiler errors?
                    TokenKind::LiteralInt32(value) =>
                        Ok(Expr::new_with_span(ExprKind::Literal(Value::Int32(value)), token.span)),
                    TokenKind::Identifier(text) =>
                        Ok(Expr::new_with_span(ExprKind::VariableRef(text), token.span)),
                    _ => Err(ParseError::new_with_span(ParseErrorKind::InvalidPrefixExpressionTerm(token.kind), token.span))
                }
        }
    }

    fn parse_infix(&mut self, left: Expr, precedence: u32) -> ParseResult {
        match self.lexer.next() {
            LexResult::Err(lex_err) => Err(
                ParseError::new_with_span(ParseErrorKind::LexerError(lex_err.kind()), lex_err.span())),

            LexResult::EndOfInput(last_location) => Err(
                ParseError::new_with_location(
                    ParseErrorKind::UnexpectedEndOfInput,
                    last_location)),

            LexResult::Ok(token) => {
                // TODO: make Operator a token kind, introduce enum OperatorKind?
                // maybe we can get some of those cool compiler errors?
                let binary_op = match token.kind {
                    TokenKind::OperatorAdd => BinaryOp::Add,
                    TokenKind::OperatorSub => BinaryOp::Sub,
                    TokenKind::OperatorMul => BinaryOp::Mul,
                    TokenKind::OperatorDiv => BinaryOp::Div,
                    TokenKind::OperatorMod => BinaryOp::Mod,
                    _ => return
                        Err(
                            ParseError::new_with_span(
                                ParseErrorKind::ExpectedBinaryOperator(token.kind),
                                token.span))
                };

                match self.parse_expr(precedence) {
                    Ok(right) => {
                        let span = Span::new(left.span.start.clone(), right.span.end.clone());
                        Ok(Expr::new_with_span(ExprKind::Binary(binary_op, Box::new(left), Box::new(right)), span))
                    },
                    Err(parse_error) => Err(parse_error) //TODO:  this feels redundant is there a smoother way?
                }
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
        parser.parse().unwrap()
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
    #[test]
    pub fn parse_identifier() {
        assert_eq!(Expr::new(ExprKind::VariableRef(String::from("abc"))), parse("abc"));
    }

}
