
// https://github.com/rust-unofficial/patterns/blob/master/patterns/visitor.md

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Int32(i32)
}
