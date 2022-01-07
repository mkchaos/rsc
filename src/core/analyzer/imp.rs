use super::context::Context;
use super::Analyzer;
use crate::core::types::nodes::*;
use crate::core::types::{get_type_size, CalcItem, ErrKind, Type};

impl Analyzer for FactorNd {
    fn analyze(&self, cxt: &mut Context) -> Result<(), ErrKind> {
        match self {
            FactorNd::Var(n) => n.analyze(cxt),
            FactorNd::Value(_) => Ok(()),
            FactorNd::Func(n) => n.analyze(cxt),
        }
    }
}

impl Analyzer for ExprNd {
    fn analyze(&self, cxt: &mut Context) -> Result<(), ErrKind> {
        for it in self.stack.iter() {
            if let CalcItem::Factor(f) = it {
                f.analyze(cxt)?;
            }
        }
        // TODO: Calc Type Error
        Ok(())
    }
}

impl Analyzer for VarNd {
    fn analyze(&self, cxt: &mut Context) -> Result<(), ErrKind> {
        let id = cxt.fetch(&self.name)?;
        self.set_id(id);
        Ok(())
    }
}

impl Analyzer for AssignNd {
    fn analyze(&self, cxt: &mut Context) -> Result<(), ErrKind> {
        self.expr.analyze(cxt)?;
        self.var.analyze(cxt)?;
        Ok(())
    }
}

impl Analyzer for DeclareNd {
    fn analyze(&self, cxt: &mut Context) -> Result<(), ErrKind> {
        if self.expr.is_some() {
            self.expr.as_ref().unwrap().analyze(cxt)?;
        }
        let id = cxt.declare_var(&self.var.name, 1)?;
        self.var.set_id(id);
        Ok(())
    }
}

impl Analyzer for StmtNd {
    fn analyze(&self, cxt: &mut Context) -> Result<(), ErrKind> {
        match self {
            StmtNd::Assign(n) => n.analyze(cxt),
            StmtNd::Declare(n) => n.analyze(cxt),
            StmtNd::Expr(n) => n.analyze(cxt),
            StmtNd::Print(n) => n.analyze(cxt),
            _ => Ok(()),
        }
    }
}

impl Analyzer for ItemNd {
    fn analyze(&self, cxt: &mut Context) -> Result<(), ErrKind> {
        match self {
            ItemNd::Block(n) => n.analyze(cxt),
            ItemNd::Stmt(n) => n.analyze(cxt),
        }
    }
}

impl Analyzer for BlockNd {
    fn analyze(&self, cxt: &mut Context) -> Result<(), ErrKind> {
        cxt.enter_scope();
        for item in self.items.iter() {
            item.analyze(cxt)?;
        }
        cxt.exit_scope();
        Ok(())
    }
}

impl Analyzer for FuncNd {
    fn analyze(&self, cxt: &mut Context) -> Result<(), ErrKind> {
        let name = &self.var.name;
        let id = if self.is_impl() {
            cxt.impl_fn(name, &self.ret_ty)?
        } else {
            cxt.declare_fn(name, &self.ret_ty)?
        };
        self.var.set_id(id);
        if self.is_impl() {
            cxt.enter_scope();
            for (t, v) in self.params.iter() {
                if v.is_none() {
                    return Err(ErrKind::FormatErr);
                }
                let v = v.as_ref().unwrap();
                let id = cxt.declare_var(&v.name, get_type_size(t.clone()))?;
                v.set_id(id);
            }
            self.block.as_ref().unwrap().analyze(cxt)?;
            cxt.exit_scope();
        }
        Ok(())
    }
}

impl Analyzer for FuncCallNd {
    fn analyze(&self, cxt: &mut Context) -> Result<(), ErrKind> {
        let name = &self.var.name;
        let id = cxt.fetch(name)?;
        let ty = cxt.get_fn_type(id)?;
        match ty {
            Type::Func(v) => {
                if self.params.len() != v.len() {
                    Err(ErrKind::TypeErr)
                } else {
                    // TODO Match Types
                    for p in self.params.iter() {
                        p.analyze(cxt)?;
                    }
                    Ok(())
                }
            }
            _ => Err(ErrKind::TypeErr),
        }
    }
}

impl Analyzer for GItemNd {
    fn analyze(&self, cxt: &mut Context) -> Result<(), ErrKind> {
        match self {
            GItemNd::Func(n) => n.analyze(cxt),
            GItemNd::Declare(n) => n.analyze(cxt),
        }
    }
}

impl Analyzer for RootNd {
    fn analyze(&self, cxt: &mut Context) -> Result<(), ErrKind> {
        for item in self.items.iter() {
            item.analyze(cxt)?;
        }
        Ok(())
    }
}
