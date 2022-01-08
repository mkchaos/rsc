use super::op::Op;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MemAddr {
    Direct(usize),
    Indirect(usize),
    Value(i32),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CodeAddr {
    Direct(usize),
    Offset(isize),
    NameStart(u32), // scope
    NameEnd(u32),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Code {
    Push(MemAddr),
    Pop(MemAddr),
    Op(Op),
    Call(CodeAddr),
    Jump(CodeAddr),
    CondJump(CodeAddr),
    Print,
    Ret,
    Exit,
}

pub fn only_pop_code() -> Code {
    Code::Pop(MemAddr::Value(0))
}
