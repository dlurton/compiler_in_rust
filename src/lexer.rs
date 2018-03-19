
use std::str::Chars;

use input::CharsReader;
use source::Span;
use source::Location;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    LiteralInt32(i32),
    Identifier(String),
    OperatorAdd,
    OperatorSub,
    OperatorMul,
    OperatorDiv,
    OperatorMod,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    kind: TokenKind,
    span: Span
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
    reader: CharsReader<'a>
}

impl <'a> Lexer<'a> {
    pub fn new(chars: Chars) -> Lexer {
        return Lexer { reader: CharsReader::new(chars) }
    }

    pub fn next(&mut self) -> Option<Token> {
        if !self.reader.has_more() {
            None
        } else {
            self.eat_white();
            if let Some(token) = self.read_single_char_token() {
                Some(token)
            } else if let Some(token) = self.read_literal_number() {
                Some(token)
            } else if let Some(token) = self.read_identifier() {
                Some(token)
            } else {
                panic!("Invalid character: {:?}!", self.reader.peek())
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
            Some('+') => Some(TokenKind::OperatorAdd),
            Some('-') => Some(TokenKind::OperatorSub),
            Some('*') => Some(TokenKind::OperatorMul),
            Some('/') => Some(TokenKind::OperatorDiv),
            Some('%') => Some(TokenKind::OperatorMod),
            _ => None
        };
        match kind {
            Some(kind) => {
                self.reader.next();
                Some(Token::new(kind, Span::new(self.reader.loc(), self.reader.loc())))
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
                    Some((buf, Span::new(start, self.reader.loc())))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    fn tok(kind: TokenKind, start_line: u32, start_col: u32, end_line: u32, end_col: u32) -> Token {
        Token::new(kind, Span::new(Location::new(start_line, start_col), Location::new(end_line, end_col)))
    }
    use super::*;
    #[test]
    fn lexer_test() {
        let mut l = Lexer::new("  123  \n 456 \nabc\na123 \n+\n-\n*\n/\n%".chars());

        assert_eq!(tok(TokenKind::LiteralInt32(123), 1, 3, 1, 5), l.next().unwrap());
        assert_eq!(tok(TokenKind::LiteralInt32(456), 2, 2, 2, 4), l.next().unwrap());
        assert_eq!(tok(TokenKind::Identifier(String::from("abc")), 3, 1, 3, 3), l.next().unwrap());
        assert_eq!(tok(TokenKind::Identifier(String::from("a123")), 4, 1, 4, 4), l.next().unwrap());

        assert_eq!(tok(TokenKind::OperatorAdd, 5, 1, 5, 1), l.next().unwrap());
        assert_eq!(tok(TokenKind::OperatorSub, 6, 1, 6, 1), l.next().unwrap());
        assert_eq!(tok(TokenKind::OperatorMul, 7, 1, 7, 1), l.next().unwrap());
        assert_eq!(tok(TokenKind::OperatorDiv, 8, 1, 8, 1), l.next().unwrap());
        assert_eq!(tok(TokenKind::OperatorMod, 9, 1, 9, 1), l.next().unwrap());
    }
}


  
