use super::op::Op;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MemAddr {
    Direct(usize),
    Indirect(usize),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CodeAddr {
    Direct(usize),
    NameStart(u32), // scope
    NameEnd(u32),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Code {
    Push(MemAddr),
    PushValue(i32),
    Pop(usize),
    PopMov(MemAddr),
    Op(Op),
    Call(CodeAddr, usize),
    Jump(CodeAddr),
    CondJump(CodeAddr),
    Print,
    Ret(usize),
}