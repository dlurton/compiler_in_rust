use std::collections::*;
use value::*; 
/// A definitition of an environment.
#[derive(Debug, Clone)]
pub struct EnvDef {
    fields: HashMap<String, EnvField>
}

impl EnvDef {
    pub fn find(&self, name: &str) -> Option<&EnvField> {
        self.fields.get(name)
    }

    pub fn create_with_default_values(&self) -> Env {
        let mut fields = self.fields.values().collect::<Vec<&EnvField>>();

        fields.sort_by(|a, b| a.ordinal.cmp(&b.ordinal));
        let values = fields.iter().map(|f| f.default_value.clone()).collect();

        Env::new(self, values)
    }
}

/// A field within an environment
#[derive(Debug, Clone, PartialEq)]
pub struct EnvField {
    pub name: String,
    pub ordinal: u32,
  //  pub data_type: DataType //As yet unused?
    pub default_value: Value
}

pub struct Env<'a> {
    def: &'a EnvDef,
    values: Vec<Value>
}

impl <'a> Env<'a> {

    fn new(def: &'a EnvDef, values: Vec<Value>) -> Env<'a> {
        Env::<'a> { def, values }
    }

    pub fn get_by_index(&self, index: u32) -> Option<&Value> {
        self.values.get(index as usize)
    }

    pub fn get_by_name(&self, name: &str) -> Option<&Value> {
        let field = self.def.fields.get(name);
        match field {
            None => None,
            Some(field) => Some(self.values.get(field.ordinal as usize).unwrap_or_else(|| panic!("Index {:?} referenced by name '{:?} was invalid")))
        }
    }
}

#[derive(Debug, Clone)]
pub struct EnvDefBuilder {
    fields: HashMap<String, EnvField>
}

impl EnvDefBuilder {
    pub fn new() -> EnvDefBuilder {
        EnvDefBuilder { fields: HashMap::new() }
    }

    pub fn with_item(mut self, name: &str, default_value: Value) -> EnvDefBuilder {
        let ordinal = self.fields.len() as u32;
        self.fields.insert(String::from(name), EnvField { name: String::from(name), default_value: default_value, ordinal: ordinal });
        self
    }
    pub fn build(&self) -> EnvDef {
        EnvDef { fields: self.fields.clone() }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn env_def_bulider_test() {

        let builder = EnvDefBuilder::new();
        let env_def = builder
            .with_item("foo", Value::Int32(100))
            .with_item("bar", Value::Int32(200))
            .build();

        assert_eq!(Some(&EnvField { name:String::from("foo"), default_value: Value::Int32(100), ordinal: 0 }), env_def.find("foo"));
        assert_eq!(Some(&EnvField { name:String::from("bar"), default_value: Value::Int32(200), ordinal: 1 }), env_def.find("bar"));
    }
}
