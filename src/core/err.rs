#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SemanticErr {
    DoubleDeclare,
    NoDeclareUse,
    MismatchType,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LexErr(pub String);