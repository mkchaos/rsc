use crate::node::node::*;
// use crate::node::semantic_analyzer::Context;
use crate::token::Token;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum V {
    Direct(usize),
    Indirect(usize),
    Value(i32),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Instrument {
    Mov(V, V),
    Op(Token, V, V),
    BinOp(Token, V, V, V),
    Print,
}

pub struct Program {
    inss: Vec<Instrument>,
}

impl Program {
    fn new() -> Self {
        Program {
            inss: Vec::new(),
        }
    }

    fn get_suggest_store_pos(&self) -> V {
        V::Value(0)
    }
}

pub trait Compiler {
    fn compile(&self, prog: &mut Program);
}

impl Compiler for FactorNd {
    fn compile(&self, prog: &mut Program) {
        let p = prog.get_suggest_store_pos();
        match self {
            FactorNd::Var(n) => {}
            FactorNd::Value(n) => {}
            FactorNd::Expr(n) => {}
        }
    }
}

impl Compiler for TermNd {
    fn compile(&self, prog: &mut Program) {}
}

impl Compiler for ExprNd {
    fn compile(&self, prog: &mut Program) {}
}

impl Compiler for VarNd {
    fn compile(&self, prog: &mut Program) {}
}
