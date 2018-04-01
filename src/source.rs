
/*
  This module contains types related to source code.
*/
use std::fmt::*;

/// A location within a source file (line & column).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Location {
    /// The line of the location, starting at 1.
    pub line: u32,
    /// The byte offset from the start of the line, starting at 1.
    pub col_offset: u32
}

impl Location {
    pub fn start() -> Location {
        Location { line: 1, col_offset: 1,  }
    }

    #[cfg(test)]
    pub fn unknown() -> Location {
        Location::new(0, 0)
    }

    #[cfg(test)]
    pub fn new(line: u32, col_offset: u32) -> Location {
        Location { line, col_offset }
    }

    pub fn next(&self) -> Location {
        Location { line: self.line, col_offset: self.col_offset + 1 }
    }

    pub fn next_line(&self) -> Location {
        Location { line: self.line + 1, col_offset: 0 }
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}, {}", self.line, self.col_offset)
    }
}

/// A span within a source file indicated by a starting Location and ending Location.
//TODO: I don't like Copy so much because the compiler will silently use it in some
//circumstances in lieu of a move--am I being overconservative about preventing copies?
#[derive(Clone, Copy, PartialEq)]
pub struct Span {
    pub start: Location,
    pub end: Location
}

impl Span {
    pub fn new(start: Location, end: Location) -> Span {
        Span { start, end }
    }

    pub fn from_location(loc: Location) -> Span {
        Span { start: loc.clone(), end: loc }
    }

    #[cfg(test)]
    pub fn unknown() -> Span {
        Span::new(Location::unknown(), Location::unknown())
    }
}

impl Debug for Span {
    fn fmt(&self, f: &mut Formatter) -> Result {
        f.write_fmt(format_args!("({}, {}, {}, {})", self.start.line, self.start.col_offset, self.end.line, self.end.col_offset))
    }
}

