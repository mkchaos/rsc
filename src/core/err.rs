#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SemanticErr {
    DoubleDeclare,
    NoDeclareUse,
    MismatchType,
}