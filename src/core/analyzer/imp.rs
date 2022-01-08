use super::context::Context;
use super::Analyzer;
use crate::core::types::nodes::*;
use crate::core::types::{get_op_param_num, get_value_type, CalcItem, ErrKind, Type};

impl Analyzer for FactorNd {
    fn analyze(&self, cxt: &mut Context) -> Result<Type, ErrKind> {
        match self {
            FactorNd::Var(n) => n.analyze(cxt),
            FactorNd::Value(n) => Ok(get_value_type(n.clone())),
            FactorNd::Func(n) => n.analyze(cxt),
        }
    }
}

impl Analyzer for ExprNd {
    fn analyze(&self, cxt: &mut Context) -> Result<Type, ErrKind> {
        let mut st = Vec::new();
        for it in self.stack.iter() {
            match it {
                CalcItem::Op(op) => {
                    let num = get_op_param_num(op.clone());
                    for _ in 0..num {
                        if Type::Int != st.pop().unwrap() {
                            return Err(ErrKind::TypeErr);
                        }
                    }
                    st.push(Type::Int);
                }
                CalcItem::Factor(f) => {
                    st.push(f.analyze(cxt)?);
                }
            }
        }
        if st.len() != 1 {
            Err(ErrKind::TypeErr)
        } else {
            Ok(st.last().unwrap().clone())
        }
    }
}

impl Analyzer for VarNd {
    fn analyze(&self, cxt: &mut Context) -> Result<Type, ErrKind> {
        let id = cxt.fetch(&self.name)?;
        self.set_id(id);
        Ok(cxt.get_type_by_id(id)?)
    }
}

impl Analyzer for AssignNd {
    fn analyze(&self, cxt: &mut Context) -> Result<Type, ErrKind> {
        let ty = self.expr.analyze(cxt)?;
        if ty != self.var.analyze(cxt)? {
            Err(ErrKind::TypeErr)
        } else {
            Ok(Type::Void)
        }
    }
}

impl Analyzer for DeclareNd {
    fn analyze(&self, cxt: &mut Context) -> Result<Type, ErrKind> {
        if self.expr.is_some() {
            if self.ty != self.expr.as_ref().unwrap().analyze(cxt)? {
                return Err(ErrKind::TypeErr);
            }
        }
        let id = cxt.declare_var(&self.var.name, &self.ty)?;
        self.var.set_id(id);
        Ok(Type::Void)
    }
}

impl Analyzer for StmtNd {
    fn analyze(&self, cxt: &mut Context) -> Result<Type, ErrKind> {
        match self {
            StmtNd::Assign(n) => n.analyze(cxt),
            StmtNd::Declare(n) => n.analyze(cxt),
            StmtNd::Expr(n) => n.analyze(cxt),
            StmtNd::Print(n) => n.analyze(cxt),
            _ => Ok(Type::Void),
        }
    }
}

impl Analyzer for ItemNd {
    fn analyze(&self, cxt: &mut Context) -> Result<Type, ErrKind> {
        match self {
            ItemNd::Block(n) => n.analyze(cxt),
            ItemNd::Stmt(n) => n.analyze(cxt),
            _ => Ok(Type::Void)
        }
    }
}

impl Analyzer for BlockNd {
    fn analyze(&self, cxt: &mut Context) -> Result<Type, ErrKind> {
        cxt.enter_scope();
        for item in self.items.iter() {
            item.analyze(cxt)?;
        }
        cxt.exit_scope();
        Ok(Type::Void)
    }
}

impl Analyzer for FuncNd {
    fn analyze(&self, cxt: &mut Context) -> Result<Type, ErrKind> {
        let name = &self.var.name;
        let id = if self.is_impl() {
            cxt.impl_fn(name, &self.func_ty())?
        } else {
            cxt.declare_fn(name, &self.func_ty())?
        };
        self.var.set_id(id);
        if self.is_impl() {
            cxt.enter_scope();
            for (t, v) in self.params.iter() {
                if v.is_none() {
                    return Err(ErrKind::FormatErr);
                }
                let v = v.as_ref().unwrap();
                let id = cxt.declare_var(&v.name, t)?;
                v.set_id(id);
            }
            self.block.as_ref().unwrap().analyze(cxt)?;
            cxt.exit_scope();
        }
        Ok(Type::Void)
    }
}

impl Analyzer for FuncCallNd {
    fn analyze(&self, cxt: &mut Context) -> Result<Type, ErrKind> {
        let name = &self.var.name;
        let id = cxt.fetch(name)?;
        self.var.set_id(id);
        let ty = cxt.get_type_by_id(id)?;
        match ty {
            Type::Func(v) => {
                if self.params.len() + 1 != v.len() {
                    Err(ErrKind::TypeErr)
                } else {
                    for (idx, p) in self.params.iter().enumerate() {
                        if p.analyze(cxt)? != v[idx] {
                            return Err(ErrKind::TypeErr);
                        }
                    }
                    Ok(v.last().unwrap().clone())
                }
            }
            _ => panic!("Should have func type"),
        }
    }
}

impl Analyzer for GItemNd {
    fn analyze(&self, cxt: &mut Context) -> Result<Type, ErrKind> {
        match self {
            GItemNd::Func(n) => n.analyze(cxt),
            GItemNd::Declare(n) => {
                n.analyze(cxt)?;
                if n.ty == Type::Int && n.try_retrieve_const().is_none() {
                    Err(ErrKind::GlobalNeedConst)
                } else {
                    Ok(Type::Void)
                }
            }
        }
    }
}

impl Analyzer for RootNd {
    fn analyze(&self, cxt: &mut Context) -> Result<Type, ErrKind> {
        for item in self.items.iter() {
            item.analyze(cxt)?;
        }
        Ok(Type::Void)
    }
}
