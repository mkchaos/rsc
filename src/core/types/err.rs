use strum_macros::Display;

#[derive(Debug, PartialEq, Clone, Display)]
pub enum ErrKind {
    LexErr,
    ParseErr,
    
    ReDeclare,
    NoDeclare,
    ReImpl,
    FuncNoImpl,
    NoMainFunc,
    TypeErr,
    GlobalNeedConst,

    StackOverFlow,
    DivideZero,
}