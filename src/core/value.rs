#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Value {
    Int(i32),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Void,
    Int,
    Func(Vec<Type>),
}

pub fn get_value_type(v: Value) -> Type {
    match v {
        Value::Int(_) => Type::Int,
    }
}

pub fn get_type_size(_ty: Type) -> usize {
    1
}
