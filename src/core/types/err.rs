#[derive(Debug, PartialEq, Clone)]
pub enum ErrKind {
    LexErr,
    ParseErr,
    
    ReDeclare,
    NoDeclare,
    ReImpl,
    FuncNoImpl,
    NoMainFunc,
    TypeErr,
    FormatErr,
    GlobalNeedConst,

    StackOverFlow,
    DivideZero,
}