#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Value {
    Int(i32),
    Bool(bool),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Type {
    Void,
    Int,
    Bool,
}

pub fn get_value_type(v: Value) -> Type {
    match v {
        Value::Int(_) => Type::Int,
        Value::Bool(_) => Type::Bool,
    }
}

pub fn get_type_size(_ty: Type) -> usize {
    1
}