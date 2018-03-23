
use source::Span;
use value::Value;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExprKind {
    Literal(Value),
    Binary(BinaryOp, Box<Expr>, Box<Expr>),
    VariableRef(String),
    VariableIndex(u32)
}

#[derive(Debug, Clone)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span
}

impl Expr {
    #[cfg(test)]
    pub fn new(kind: ExprKind) -> Expr {
        Expr { kind, span: Span::unknown() }
    }

    pub fn new_with_span(kind: ExprKind, span: Span) -> Expr {
        Expr { kind, span }
    }

    pub fn clone_with(&self, kind: ExprKind) -> Expr {
        Expr { kind: kind, span: self.span.clone() }
    }
}

/// The main reason for implementing this manually is to prevent the span
/// from being part of the equality comparison...  Specifying the span in
/// unit tests is a pain.
impl PartialEq for Expr {
    fn eq(&self, other: &Expr) -> bool {
        self.kind == other.kind
    }
    fn ne(&self, other: &Expr) -> bool {
        self.kind != other.kind
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    Int32
}

#[derive(Debug)]
pub struct EnvDef {
    indexes: HashMap<String, EnvField>
}

impl EnvDef {
    pub fn find(&self, name: &str) -> Option<&EnvField> {
        self.indexes.get(name)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnvField {
    pub name: String,
    pub ordinal: u32,
    pub data_type: DataType
}

#[derive(Debug, Clone)]
pub struct EnvDefBuilder {
    indexes: HashMap<String, EnvField>
}

impl EnvDefBuilder {
    pub fn new() -> EnvDefBuilder {
        EnvDefBuilder { indexes: HashMap::new() }
    }
    pub fn with_item(mut self, name: &str, data_type: DataType) -> EnvDefBuilder {
        let ordinal = self.indexes.len() as u32;
        self.indexes.insert(String::from(name), EnvField { name: String::from(name), data_type: data_type, ordinal: ordinal });
        self
    }
    pub fn build(&self) -> EnvDef {
        EnvDef { indexes: self.indexes.clone() }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn env_def_bulider_test() {

        let builder = EnvDefBuilder::new();
        let env_def = builder
            .with_item("foo", DataType::Int32)
            .with_item("bar", DataType::Int32)
            .build();

        assert_eq!(Some(&EnvField { name:String::from("foo"), data_type: DataType::Int32, ordinal: 0 }), env_def.find("foo"));
        assert_eq!(Some(&EnvField { name:String::from("bar"), data_type: DataType::Int32, ordinal: 1 }), env_def.find("bar"));

    }
}
