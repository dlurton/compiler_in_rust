
/*
  This module contains types related to source code.
*/

/// A location within a source file (line & column).
#[derive(Debug, Clone, PartialEq)]
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

    pub fn unknown() -> Location {
        Location::new(0, 0)
    }

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

/// A span within a source file indicated by a starting Location and ending Location.
#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub start: Location,
    pub end: Location
}

impl Span {
    pub fn new(start: Location, end: Location) -> Span {
        Span { start, end }
    }

    pub fn unknown() -> Span {
        Span::new(Location::unknown(), Location::unknown())
    }
}
