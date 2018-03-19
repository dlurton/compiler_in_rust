
use source::Location;

use std::str::Chars;
use std::fmt;
use std::collections::VecDeque;

pub struct CharsReader<'a> {
    is_at_eof: bool,
    input_chars: Chars<'a>,
    lookahead: VecDeque<char>,
    loc: Option<Location>
}

impl <'a> CharsReader<'a> {
    pub fn new(input_chars: Chars<'a>) -> CharsReader<'a> {
        CharsReader { is_at_eof: false, input_chars, lookahead: VecDeque::new(), loc: None }
    }

    pub fn loc(&self) -> Location {
        match self.loc {
            None => panic!("Must call .next() before taking the current location."),
            Some(ref l) => l.clone() 
        }
    }

    pub fn next(&mut self) -> Option<char> {
        //Note: prime() will do nothing if end of stream has been previously reached.
        self.prime(1);

        match self.lookahead.pop_front() {
            None => None,
            Some(c) => {
                self.loc = Some(match self.loc {
                    None => Location::start(),
                    Some(ref l) => if c == '\n' { l.next_line() } else { l.next() }
                });
                Some(c)
            }
        }
    }

    pub fn has_more(&self) -> bool {
        !self.is_at_eof
    }

    pub fn peek(&mut self) -> Option<char> {
        self.peek_n(0)
    }

    pub fn peek_n(&mut self, n: u32) -> Option<char> {
        self.prime(n + 1);
        if n as usize >= self.lookahead.len() { None } else { Some(self.lookahead[n as usize].clone()) }
    }

    fn prime(&mut self, min_chars: u32) {
        while self.lookahead.len() < min_chars as usize && !self.is_at_eof {
            match self.input_chars.next() {
                Some(c) => self.lookahead.push_back(c),
                None => self.is_at_eof = true
            }
        };
    }
}

impl <'a> fmt::Debug for CharsReader <'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Reader {{ loc: {:?} }}", self.loc)
    }
}

#[cfg(test)]
mod reader_tests {
    use super::*;
    #[test]
    fn reader_test() {

        let src = "ab\ncd";
        let mut reader = CharsReader::new(src.chars());

        assert_eq!('a', reader.peek_n(0).unwrap());
        assert_eq!('b', reader.peek_n(1).unwrap());
        assert_eq!('\n', reader.peek_n(2).unwrap());
        assert_eq!('c', reader.peek_n(3).unwrap());
        assert_eq!('d', reader.peek_n(4).unwrap());
        assert!(reader.peek_n(5).is_none());


        let chr = reader.next().unwrap();
        assert_eq!('a', chr);
        assert_eq!(Location::new(1, 1), reader.loc());

        let chr = reader.next().unwrap();
        assert_eq!('b', chr);
        assert_eq!(Location::new(1, 2), reader.loc());

        let chr = reader.next().unwrap();
        assert_eq!('\n', chr);
        assert_eq!(Location::new(2, 0), reader.loc());

        let chr = reader.next().unwrap();
        assert_eq!('c', chr);
        assert_eq!(Location::new(2, 1), reader.loc());

        let chr = reader.next().unwrap();
        assert_eq!('d', chr);
        assert_eq!(Location::new(2, 2), reader.loc());

        assert!(reader.next().is_none());
    }
}
