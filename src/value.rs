
// https://github.com/rust-unofficial/patterns/blob/master/patterns/visitor.md
use std::vec::Vec;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Int32(i32),
    Tuple(Vec<Value>)
}

