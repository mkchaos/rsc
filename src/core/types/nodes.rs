use super::op::{calc_op_1, calc_op_2, get_op_param_num, CalcItem};
use super::token::{Type, Value};
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

    pub fn try_retrieve_const(&self) -> Option<i32> {
        let mut st = Vec::new();
        for it in self.stack.iter() {
            match it {
                CalcItem::Op(op) => {
                    let num = get_op_param_num(op.clone());
                    match num {
                        1 => {
                            let a = st.pop().unwrap();
                            st.push(calc_op_1(op.clone(), a));
                        }
                        2 => {
                            let b = st.pop().unwrap();
                            let a = st.pop().unwrap();
                            st.push(calc_op_2(op.clone(), a, b));
                        }
                        _ => {
                            panic!("not support yet");
                        }
                    }
                }
                CalcItem::Factor(FactorNd::Value(Value::Int(num))) => {
                    st.push(*num);
                }
                _ => {
                    return None;
                }
            }
        }
        if st.len() != 1 {
            None
        } else {
            Some(st.pop().unwrap())
        }
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

    pub fn try_retrieve_const(&self) -> Option<i32> {
        if self.expr.is_none() {
            Some(0)
        } else {
            self.expr.as_ref().unwrap().try_retrieve_const()
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

    pub fn func_ty(&self) -> Type {
        let mut ty_vec = Vec::new();
        for (t, _) in self.params.iter() {
            ty_vec.push(t.clone());
        }
        ty_vec.push(self.ret_ty.clone());
        Type::Func(ty_vec)
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
