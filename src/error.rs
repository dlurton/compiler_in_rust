
//use std::io::Write;
use std::fmt;
use std::cmp;
use std::clone;
use source::*;

pub trait ErrorKind : fmt::Debug + cmp::PartialEq + clone::Clone {
    fn message(&self) -> String;
}

#[derive(Debug, Clone, PartialEq)]
pub struct SourceError<TErrorKind: ErrorKind> {
    kind: TErrorKind,
    span: Span,
}

impl <TErrorKind: ErrorKind> SourceError<TErrorKind> {

    pub fn new_with_span(kind: TErrorKind, span: Span) -> SourceError<TErrorKind> {
        SourceError::<TErrorKind> { kind, span }
    }

    pub fn new_with_location(kind: TErrorKind, loc: Location) -> SourceError<TErrorKind> {
        SourceError::<TErrorKind> { kind: kind, span: Span::from_location(loc) }
    }

    pub fn kind(&self) -> TErrorKind {
        //TODO Do I really need to clone here?
        //Consider removing this method and accessing the field directly.
        self.kind.clone() 
    }
    pub fn span(&self) -> Span {
        self.span
    }
    pub fn message(&self) -> String {
        self.kind.message()
    }
}

