#[derive(Debug, PartialEq, Clone)]
pub enum ErrKind {
    LexErr,
    ParseErr,
    
    ReDeclare,
    NoDeclare,
    ReImpl,
    TypeErr,

    StackOverFlow,
    DivideZero,
}