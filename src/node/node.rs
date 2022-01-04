use crate::token::{Token, Type, Value};
use std::cell::RefCell;

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
    pub id: RefCell<usize>,
}

impl VarNd {
    pub fn new(ty: Option<Type>, name: String) -> Self {
        VarNd {
            ty: ty,
            name: name,
            id: RefCell::new(0),
        }
    }

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
pub enum ItemNd {
    Stmt(StmtNd),
    Block(BlockNd),
}

#[derive(Debug, Clone)]
pub struct BlockNd {
    pub items: Vec<ItemNd>,
}



// #[derive(Debug, Clone)]
// pub struct FuncHeadNd {

// }

// #[derive(Debug, Clone)]
// pub struct FuncNd {
//     pub head: Box<FuncHeadNd>,
//     pub block: Box<BlockNd>,
// }

// #[derive(Debug, Clone)]
// pub enum GItemNd {
//     Stmt(StmtNd),
//     Func(FuncNd),
// }

#[derive(Debug, Clone)]
pub struct RootNd {
    pub items: Vec<ItemNd>,
}
