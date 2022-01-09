use strum_macros::Display;

#[derive(Debug, PartialEq, Clone, Display)]
pub enum ErrKind {
    LexErr,
    ParseErr,

    JumpNoLoop,
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
