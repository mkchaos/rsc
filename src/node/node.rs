use crate::token::{Token, Type, Value};

#[derive(Debug, Clone)]
pub enum FactorNd {
    Var(VarNd),
    Value(Value),
    Expr(Box<ExprNd>),
}

#[derive(Debug, Clone)]
pub struct TermNd {
    pub a: Box<FactorNd>,
    pub b: Option<(Box<TermNd>, Token)>,
}

#[derive(Debug, Clone)]
pub struct ExprNd {
    pub a: Box<TermNd>,
    pub b: Option<(Box<ExprNd>, Token)>,
}

#[derive(Debug, Clone)]
pub struct VarNd {
    pub ty: Option<Type>,
    pub name: String,
}

impl VarNd {
    pub fn declared(&self) -> bool {
        self.ty.is_some()
    }
}

#[derive(Debug, Clone)]
pub struct StmtNd {
    pub var: Option<VarNd>,
    pub expr: Option<ExprNd>,
}

#[derive(Debug, Clone)]
pub struct RootNd {
    pub stmts: Vec<StmtNd>,
}
