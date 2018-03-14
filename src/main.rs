


#[derive(Debug)]
#[derive(Clone)]
struct Location {
    line: u32,
    col_offset: u32
}

impl Location {
    pub fn new() -> Location {
        Location { line: 1, col_offset: 0,  }
    }

    pub fn next(&self) -> Location {
        Location { line: self.line, col_offset: self.col_offset + 1 }
    }

    pub fn next_line(&self) -> Location {
        Location { line: self.line + 1, col_offset: 0 }
    }
}

#[derive(Debug)]
#[derive(Clone)]
struct Terminal {
    chr: char,
    loc: Location
}

impl Terminal {
    pub fn new(chr: char, loc: Location) -> Terminal {
        return Terminal { chr, loc };
    }
}

use std::fmt;
use std::collections::VecDeque;

struct Reader<'a> {
    is_at_eof: bool,
    input_chars: std::str::Chars<'a>,
    lookahead: VecDeque<Terminal>,
    loc: Location
}

impl <'a> Reader<'a> {
    pub fn new(input_chars: std::str::Chars<'a>) -> Reader<'a> {
        Reader { is_at_eof: false, input_chars, loc: Location::new(), lookahead: VecDeque::new() }
    }

    fn next(&mut self) -> Option<Terminal> {
        self.prime(1);
        return if self.lookahead.is_empty() { None } else { self.lookahead.pop_front() }
    }

    fn peek(&mut self) -> Option<Terminal> {
        return self.peek_n(1);
    }

    fn peek_n(&mut self, n: u32) -> Option<Terminal> {
        self.prime(n + 1);
        return if n as usize >= self.lookahead.len() { None } else { Some(self.lookahead[n as usize].clone()) };
    }

    fn prime(&mut self, min_lookahead_terminals: u32) {
        while !self.is_at_eof && self.lookahead.len() < min_lookahead_terminals as usize {
            self.read_and_prime()
        }
    }

    fn read_and_prime(&mut self) {
        let next_c = self.input_chars.next();
        if next_c.is_some() {
            let c = next_c.unwrap();
            self.loc = if c == '\n' { self.loc.next_line() } else { self.loc.next() };
            self.lookahead.push_back(Terminal::new(c, self.loc.clone()));
        } else {
            self.is_at_eof = true;
        }
    }
}

impl <'a> fmt::Debug for Reader <'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Reader {{ loc: {:?} }}", self.loc)
    }
}

struct Lexer<'a> {
    reader: Reader<'a>
}


fn main() {
    let src = "  \t \n 1234 \n";
    let mut reader = Reader::new(src.chars());
    loop { 
        let curr = reader.next();
        match curr {
            Some(c) => println!("{}", c.chr),
            None => {
                println!("end of input");
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reader_test() {

        let src = "ab\ncd";
        let mut reader = Reader::new(src.chars());

        assert_eq!('a', reader.peek_n(0).unwrap().chr);
        assert_eq!('b', reader.peek_n(1).unwrap().chr);
        assert_eq!('\n', reader.peek_n(2).unwrap().chr);
        assert_eq!('c', reader.peek_n(3).unwrap().chr);
        assert_eq!('d', reader.peek_n(4).unwrap().chr);
        assert!(reader.peek_n(5).is_none());
        
        let t = reader.next().unwrap();
        assert_eq!('a', t.chr);
        assert_eq!(1, t.loc.line);
        assert_eq!(1, t.loc.col_offset);

        let t = reader.next().unwrap();
        assert_eq!('b', t.chr);
        assert_eq!(1, t.loc.line);
        assert_eq!(2, t.loc.col_offset);

        let t = reader.next().unwrap();
        assert_eq!('\n', t.chr);
        assert_eq!(2, t.loc.line);
        assert_eq!(0, t.loc.col_offset);

        let t = reader.next().unwrap();
        assert_eq!('c', t.chr);
        assert_eq!(2, t.loc.line);
        assert_eq!(1, t.loc.col_offset);

        let t = reader.next().unwrap();
        assert_eq!('d', t.chr);
        assert_eq!(2, t.loc.line);
        assert_eq!(2, t.loc.col_offset);

        assert!(reader.next().is_none());
    }

    #[test]
    fn lexer_test() {
        
    }
}

