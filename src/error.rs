
use std::io::Write;

use source::*;

pub struct ErrorStream<'a> {
    writable: &'a mut Write,
    pub error_count: u32
}

impl <'a> ErrorStream<'a> {
    pub fn new(writable: &mut Write) -> ErrorStream {
        ErrorStream { writable, error_count: 0 }
    }

    pub fn error_with_span(&mut self, span: Span, message: String) {
        self.error_count += 1;
        self.writable.write_fmt(format_args!("{}: {}", span.start, message))
            .expect("Writing to error stream failed?");
    }
}

