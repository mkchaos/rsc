use super::*;
use crate::token::{get_value_type, Type};

pub trait SemanticAnalyzer {
    fn analyze(&self, cxt: &mut Context) -> Result<Type, SemanticErr>;
}

impl SemanticAnalyzer for FactorNd {
    fn analyze(&self, cxt: &mut Context) -> Result<Type, SemanticErr> {
        match self {
            FactorNd::Var(n) => n.analyze(cxt),
            FactorNd::Value(n) => Ok(get_value_type(*n)),
            FactorNd::Expr(n) => n.analyze(cxt),
        }
    }
}

impl SemanticAnalyzer for TermNd {
    fn analyze(&self, cxt: &mut Context) -> Result<Type, SemanticErr> {
        let ty = self.a.analyze(cxt)?;
        if ty != Type::Int {
            return Err(SemanticErr::MismatchType);
        }
        if self.b.is_some() {
            let ty = self.b.as_ref().unwrap().0.analyze(cxt)?;
            if ty != Type::Int {
                return Err(SemanticErr::MismatchType);
            }
        }
        Ok(Type::Int)
    }
}

impl SemanticAnalyzer for ExprNd {
    fn analyze(&self, cxt: &mut Context) -> Result<Type, SemanticErr> {
        let ty = self.a.analyze(cxt)?;
        if ty != Type::Int {
            return Err(SemanticErr::MismatchType);
        }
        if self.b.is_some() {
            let ty = self.b.as_ref().unwrap().0.analyze(cxt)?;
            if ty != Type::Int {
                return Err(SemanticErr::MismatchType);
            }
        }
        Ok(Type::Int)
    }
}

impl SemanticAnalyzer for VarNd {
    fn analyze(&self, cxt: &mut Context) -> Result<Type, SemanticErr> {
        if self.declared() {
            let vcxt = cxt.declare(self)?;
            *self.id.borrow_mut() = vcxt.id;
            Ok(Type::Void)
        } else {
            let vc = cxt.fetch(self)?;
            *self.id.borrow_mut() = vc.id;
            Ok(vc.ty)
        }
    }
}

impl SemanticAnalyzer for StmtNd {
    fn analyze(&self, cxt: &mut Context) -> Result<Type, SemanticErr> {
        let mut ty = Type::Void;
        if self.expr.is_some() {
            ty = self.expr.as_ref().unwrap().analyze(cxt)?;
        }
        if self.var.is_some() {
            let vn = self.var.as_ref().unwrap();
            let final_ty = vn.analyze(cxt)?;
            let declare_vn = cxt.fetch(vn)?;
            if self.expr.is_some() && ty != declare_vn.ty {
                return Err(SemanticErr::MismatchType);
            }
            ty = final_ty;
        }
        Ok(ty)
    }
}

impl SemanticAnalyzer for ItemNd {
    fn analyze(&self, cxt: &mut Context) -> Result<Type, SemanticErr> {
        match self {
            ItemNd::Block(n) => n.analyze(cxt),
            ItemNd::Stmt(n) => n.analyze(cxt),
        }
    }
}

impl SemanticAnalyzer for BlockNd {
    fn analyze(&self, cxt: &mut Context) -> Result<Type, SemanticErr> {
        cxt.enter_scope();
        for item in self.items.iter() {
            item.analyze(cxt)?;
        }
        cxt.exit_scope();
        Ok(Type::Void)
    }
}

impl SemanticAnalyzer for FuncHeadNd {
    fn analyze(&self, _cxt: &mut Context) -> Result<Type, SemanticErr> {
        Ok(Type::Void)
    }
}

impl SemanticAnalyzer for FuncNd {
    fn analyze(&self, _cxt: &mut Context) -> Result<Type, SemanticErr> {
        Ok(Type::Void)
    }
}

impl SemanticAnalyzer for GItemNd {
    fn analyze(&self, cxt: &mut Context) -> Result<Type, SemanticErr> {
        match self {
            GItemNd::Func(n) => n.analyze(cxt),
            GItemNd::Stmt(n) => n.analyze(cxt),
        }
    }
}

impl SemanticAnalyzer for RootNd {
    fn analyze(&self, cxt: &mut Context) -> Result<Type, SemanticErr> {
        for item in self.items.iter() {
            item.analyze(cxt)?;
        }
        Ok(Type::Void)
    }
}
