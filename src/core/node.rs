use crate::core::{CalcItem, Type, Value};
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub enum FactorNd {
    Var(VarNd),
    Value(Value),
    Func(FuncCallNd),
}

#[derive(Debug, Clone)]
pub struct ExprNd {
    pub stack: Vec<CalcItem>,
}

impl ExprNd {
    pub fn new(stack: Vec<CalcItem>) -> Self {
        ExprNd { stack: stack }
    }
}

#[derive(Debug, Clone)]
pub struct VarNd {
    pub name: String,
    id: RefCell<u32>,
}

impl VarNd {
    pub fn new(name: String) -> Self {
        VarNd {
            name: name,
            id: RefCell::new(0),
        }
    }

    pub fn get_id(&self) -> u32 {
        *self.id.borrow()
    }

    pub fn set_id(&self, id: u32) {
        *self.id.borrow_mut() = id;
    }
}

#[derive(Debug, Clone)]
pub struct AssignNd {
    pub var: VarNd,
    pub expr: ExprNd,
}

impl AssignNd {
    pub fn new(v: VarNd, ex: ExprNd) -> Self {
        AssignNd { var: v, expr: ex }
    }
}

#[derive(Debug, Clone)]
pub struct DeclareNd {
    pub ty: Type,
    pub var: VarNd,
    pub expr: Option<ExprNd>,
}

impl DeclareNd {
    pub fn new(ty: Type, v: VarNd, ex: Option<ExprNd>) -> Self {
        DeclareNd {
            ty: ty,
            var: v,
            expr: ex,
        }
    }
}

#[derive(Debug, Clone)]
pub enum StmtNd {
    Assign(AssignNd),
    Declare(DeclareNd),
    Expr(ExprNd),
    Print(VarNd),
    Empty,
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

impl BlockNd {
    pub fn new(items: Vec<ItemNd>) -> Self {
        BlockNd { items: items }
    }
}

#[derive(Debug, Clone)]
pub struct FuncNd {
    pub ret_ty: Type,
    pub var: VarNd,
    pub params: Vec<(Type, Option<VarNd>)>,
    pub block: Option<BlockNd>,
}

impl FuncNd {
    pub fn new(
        ty: Type,
        var: VarNd,
        params: Vec<(Type, Option<VarNd>)>,
        block: Option<BlockNd>,
    ) -> Self {
        FuncNd {
            ret_ty: ty,
            var: var,
            params: params,
            block: block,
        }
    }

    pub fn is_impl(&self) -> bool {
        self.block.is_some()
    }

    pub fn check(&self) -> bool {
        if self.is_impl() {
            for p in self.params.iter() {
                if p.1.is_none() {
                    return false;
                }
            }
        }
        return true;
    }
}

#[derive(Debug, Clone)]
pub struct FuncCallNd {
    pub var: VarNd,
    pub params: Vec<ExprNd>,
}

impl FuncCallNd {
    pub fn new(var: VarNd, params: Vec<ExprNd>) -> Self {
        FuncCallNd {
            var: var,
            params: params,
        }
    }
}

#[derive(Debug, Clone)]
pub enum GItemNd {
    Declare(DeclareNd),
    Func(FuncNd),
}

#[derive(Debug, Clone)]
pub struct RootNd {
    pub items: Vec<GItemNd>,
}

impl RootNd {
    pub fn new(items: Vec<GItemNd>) -> Self {
        RootNd { items: items }
    }
}
