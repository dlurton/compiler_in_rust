
//use std::io::Write;
use std::fmt;
use std::cmp;
use std::clone;
use source::*;

pub trait ErrorKind : fmt::Debug + cmp::PartialEq + clone::Clone +  {
    fn message(&self) -> String;
}

#[derive(Debug, Clone, PartialEq)]
pub struct SourceError<TErrorKind: ErrorKind> {
    pub kind: TErrorKind,
    pub span: Span,
}

impl <TErrorKind: ErrorKind> SourceError<TErrorKind> {

    pub fn new_with_span(kind: TErrorKind, span: Span) -> SourceError<TErrorKind> {
        SourceError::<TErrorKind> { kind, span }
    }

    pub fn new_with_location(kind: TErrorKind, loc: Location) -> SourceError<TErrorKind> {
        SourceError::<TErrorKind> { kind: kind, span: Span::from_location(loc) }
    }
}

