
use std::str::Chars;
use std::collections::vec_deque::VecDeque;
use std::fmt;
use input::CharsReader;
use source::*;
use error::*;
use common::*;

#[derive(Debug, Clone, PartialEq)]
pub enum LexerErrorKind {
    InvalidCharacter(char),
    InvalidInteger(String)
}

impl ErrorKind for LexerErrorKind {
    fn message(&self) -> String {
        match self {
            &LexerErrorKind::InvalidInteger(ref text) => format!("Invalid integer: '{}'", text),
            &LexerErrorKind::InvalidCharacter(ref chr) => format!("Invalid character: '{}'", chr),
        }
    }
}

pub type LexerError = SourceError<LexerErrorKind>;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    LiteralInt32(i32),
    Identifier(String),
    // The existence of BinaryOp here would make it difficult to support
    // operators that may be either unary or n-ary depending on context.  Good thing I don't like
    // context sensitivity.
    BinaryOperator(BinaryOp),
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &TokenKind::LiteralInt32(ref n) => write!(f, "literal integer {}", n),
            &TokenKind::Identifier(ref text) => write!(f, "identifier \"{}\"", text),
            &TokenKind::BinaryOperator(ref op) => match op {
                &BinaryOp::Add => write!(f, "operator +"),
                &BinaryOp::Sub => write!(f, "operator -"),
                &BinaryOp::Mul => write!(f, "operator *"),
                &BinaryOp::Div => write!(f, "operator /"),
                &BinaryOp::Mod => write!(f, "operator %"),
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LexResult {
    Ok(Token),
    EndOfInput(Location),
    Err(LexerError),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Token {
        Token { kind, span }
    }
}


fn is_white(chr: char) -> bool {
    match chr {
        ' ' | '\t' | '\n' | '\r' => true,
        _ => false
    }
}

fn is_digit(chr: char) -> bool {
    chr >= '0' && chr <= '9'
}

fn is_letter(chr: char) -> bool {
    (chr >= 'a' && chr <= 'z') || (chr >= 'A' && chr <= 'Z')
}

pub struct Lexer<'a> {
    reader: CharsReader<'a>,
    lookahead: VecDeque<LexResult>,
}

impl <'a> Lexer<'a> {
    pub fn new(chars: Chars<'a>) -> Lexer<'a> {
        return Lexer {
            reader: CharsReader::new(chars),
            lookahead: VecDeque::new(),
        }
    }

    pub fn has_more(&mut self) -> bool {

        return if !self.lookahead.is_empty() {
            if let Some(&LexResult::EndOfInput(_)) = self.lookahead.front() {
                false
            } else {
                true
            }
        } else {
            self.eat_white();
            self.reader.has_more()
        }
    }

    pub fn next(&mut self) -> LexResult {
        self.prime(1);

        //Note: pop_front() shouldn't ever return None after call to self.prime(1)
        self.lookahead.pop_front().unwrap()
    }

    pub fn peek(&mut self) -> LexResult {
        self.peek_n(0)
    }

    pub fn peek_n(&mut self, n: u32) -> LexResult {
        self.prime(n + 1);
        self.lookahead[n as usize].clone() 
    }

    fn prime(&mut self, num_tokens: u32) {
        while self.lookahead.len() < num_tokens as usize {
            let token = self.read_next_token();
            self.lookahead.push_back(token)
        }
    }

    /// Stuffs a LexResult into the lookahead.
    /// If the end of input has been reached, stuffs a LexResult::EndOfInput.
    fn read_next_token(&mut self) -> LexResult {
        //Note:  eat_white() gracefully handles EOF
        self.eat_white();
        if !self.reader.has_more() {
            LexResult::EndOfInput(self.reader.loc())
        } else {
            if let Some(token) = self.read_single_char_token() {
                LexResult::Ok(token)
            } else if let Some(token) = self.read_literal_number() {
                LexResult::Ok(token)
            } else if let Some(token) = self.read_identifier() {
                LexResult::Ok(token)
            } else {
                //Note:  if self.reader.has_more() then self.reader.next() shouldn't ever return None.
                LexResult::Err(
                    LexerError::new_with_location(
                        LexerErrorKind::InvalidCharacter(self.reader.next().unwrap()),
                        self.reader.loc())) 
            }
        }
    }

    fn eat_white(&mut self) {
        while let Some(c) = self.reader.peek() {
            if !is_white(c) {
                break;
            }
            self.reader.next();
        }
    }

    fn read_single_char_token(&mut self) -> Option<Token> {
        let kind = match self.reader.peek() {
            Some('+') => Some(TokenKind::BinaryOperator(BinaryOp::Add)),
            Some('-') => Some(TokenKind::BinaryOperator(BinaryOp::Sub)),
            Some('*') => Some(TokenKind::BinaryOperator(BinaryOp::Mul)),
            Some('/') => Some(TokenKind::BinaryOperator(BinaryOp::Div)),
            Some('%') => Some(TokenKind::BinaryOperator(BinaryOp::Mod)),
            _ => None
        };
        match kind {
            Some(kind) => {
                self.reader.next();
                Some(Token::new(kind, Span::from_locations(self.reader.loc(), self.reader.loc())))
            }
            None => None
        }
    }

    fn read_literal_number(&mut self) -> Option<Token> {
        if let Some((text, span)) = self.read_token(|c| is_digit(c), |c| is_digit(c)) {
            // For now we don't care too much about error handling.
            let maybe_int = text.parse::<i32>();
            match maybe_int {
                Ok(i) => Some(Token::new(TokenKind::LiteralInt32(i), span)),
                Err(_e) => panic!("Crap, it wasn't an integer!")
            }
        } else {
            None
        }
    }

    fn read_identifier(&mut self) -> Option<Token> {
        if let Some((text, span)) = self.read_token(|c| is_letter(c), |c| is_letter(c) || is_digit(c)) {
            Some(Token::new(TokenKind::Identifier(text), span))
        } else {
            None
        }
    }

    fn read_token(&mut self, start_cond: fn(char)->bool, continue_cond: fn(char)->bool) -> Option<(String, Span)> {
        match self.reader.peek() {
            None => None,
            Some(c) => {
                if !start_cond(c) {
                    None
                } else {
                    let mut buf = String::new();
                    buf.push(c);
                    self.reader.next();
                    let start = self.reader.loc();
                    while let Some(c) = self.reader.peek() {
                        if !continue_cond(c) {
                            break;
                        }
                        buf.push(c);

                        self.reader.next();
                    }
                    Some((buf, Span::from_locations(start, self.reader.loc())))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tok(kind: TokenKind, start_line: u32, start_col: u32, end_line: u32, end_col: u32) -> LexResult {
        LexResult::Ok(Token::new(kind, Span::from_locations(Location::new(start_line, start_col), Location::new(end_line, end_col))))
    }

    #[test]
    fn lexer_test() {
        let mut l = Lexer::new("  123  \n 456 \nabc\na123 \n+\n-\n*\n/\n%".chars());

        assert_eq!(tok(TokenKind::LiteralInt32(123), 1, 3, 1, 5), l.next());
        assert_eq!(tok(TokenKind::LiteralInt32(456), 2, 2, 2, 4), l.next());
        assert_eq!(tok(TokenKind::Identifier(String::from("abc")), 3, 1, 3, 3), l.next());
        assert_eq!(tok(TokenKind::Identifier(String::from("a123")), 4, 1, 4, 4), l.next());

        assert_eq!(tok(TokenKind::BinaryOperator(BinaryOp::Add), 5, 1, 5, 1), l.next());
        assert_eq!(tok(TokenKind::BinaryOperator(BinaryOp::Sub), 6, 1, 6, 1), l.next());
        assert_eq!(tok(TokenKind::BinaryOperator(BinaryOp::Mul), 7, 1, 7, 1), l.next());
        assert_eq!(tok(TokenKind::BinaryOperator(BinaryOp::Div), 8, 1, 8, 1), l.next());
        assert_eq!(tok(TokenKind::BinaryOperator(BinaryOp::Mod), 9, 1, 9, 1), l.next());
    }
    #[test]
    fn lexer_parse_single_identifier() {
        let mut l = Lexer::new("abc".chars());
        assert_eq!(tok(TokenKind::Identifier(String::from("abc")), 1, 1, 1, 3), l.next());
    }
}


  
