use super::Op;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Addr {
    Direct(usize),
    Indirect(usize),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Code {
    SetA(Addr, Addr),
    SetV(Addr, i32),
    PushA(Addr),
    PushV(i32),
    Pop,
    Op(Op),
    Call(usize),
    Jump(isize),
    CondJump(isize),
    Print,
    Ret,
    Exit,
}
