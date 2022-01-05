use crate::core::*;

pub trait SemanticAnalyzer {
    fn analyze(&self, cxt: &mut SemanticContext) -> Result<(), SemanticErr>;
}

impl SemanticAnalyzer for FactorNd {
    fn analyze(&self, cxt: &mut SemanticContext) -> Result<(), SemanticErr> {
        match self {
            FactorNd::Var(n) => n.analyze(cxt),
            FactorNd::Value(_) => Ok(()),
            FactorNd::Func(n) => n.analyze(cxt),
        }
    }
}

impl SemanticAnalyzer for ExprNd {
    fn analyze(&self, cxt: &mut SemanticContext) -> Result<(), SemanticErr> {

        Ok(())
    }
}

impl SemanticAnalyzer for VarNd {
    fn analyze(&self, cxt: &mut SemanticContext) -> Result<(), SemanticErr> {
        let var_cxt = cxt.fetch(&self.name)?;
        self.set_id(var_cxt.id);
        Ok(())
    }
}

impl SemanticAnalyzer for AssignNd {
    fn analyze(&self, cxt: &mut SemanticContext) -> Result<(), SemanticErr> {
        self.expr.analyze(cxt)?;
        self.var.analyze(cxt)?;
        Ok(())
    }
}

impl SemanticAnalyzer for DeclareNd {
    fn analyze(&self, cxt: &mut SemanticContext) -> Result<(), SemanticErr> {
        if self.expr.is_some() {
            self.expr.as_ref().unwrap().analyze(cxt)?;
        }
        let var_cxt = cxt.declare(self.ty.clone(), &self.var.name)?;
        self.var.set_id(var_cxt.id);
        Ok(())
    }
}

impl SemanticAnalyzer for StmtNd {
    fn analyze(&self, cxt: &mut SemanticContext) -> Result<(), SemanticErr> {
        match self {
            StmtNd::Assign(n) => n.analyze(cxt),
            StmtNd::Declare(n) => n.analyze(cxt),
            StmtNd::Expr(n) => n.analyze(cxt),
            StmtNd::Print(n) => n.analyze(cxt),
            _ => Ok(()),
        }
    }
}

impl SemanticAnalyzer for ItemNd {
    fn analyze(&self, cxt: &mut SemanticContext) -> Result<(), SemanticErr> {
        match self {
            ItemNd::Block(n) => n.analyze(cxt),
            ItemNd::Stmt(n) => n.analyze(cxt),
        }
    }
}

impl SemanticAnalyzer for BlockNd {
    fn analyze(&self, cxt: &mut SemanticContext) -> Result<(), SemanticErr> {
        cxt.enter_scope();
        for item in self.items.iter() {
            item.analyze(cxt)?;
        }
        cxt.exit_scope();
        Ok(())
    }
}

impl SemanticAnalyzer for FuncNd {
    fn analyze(&self, cxt: &mut SemanticContext) -> Result<(), SemanticErr> {
        if cxt.fetch(&self.var.name).is_ok() {
            return Err(SemanticErr::DoubleDeclare);
        }
        if self.is_impl() {
            cxt.enter_scope();
            for (t, v) in self.params.iter() {
                let v = v.as_ref().unwrap();
                let var_cxt = cxt.declare(t.clone(), &v.name)?;
                v.set_id(var_cxt.id);
            }
            self.block.as_ref().unwrap().analyze(cxt)?;
            cxt.exit_scope();
        }
        Ok(())
    }
}

impl SemanticAnalyzer for FuncCallNd {
    fn analyze(&self, cxt: &mut SemanticContext) -> Result<(), SemanticErr> {
        for p in self.params.iter() {
            p.analyze(cxt)?;
        }
        Ok(())
    }
}

impl SemanticAnalyzer for GItemNd {
    fn analyze(&self, cxt: &mut SemanticContext) -> Result<(), SemanticErr> {
        match self {
            GItemNd::Func(n) => n.analyze(cxt),
            GItemNd::Declare(n) => n.analyze(cxt),
        }
    }
}

impl SemanticAnalyzer for RootNd {
    fn analyze(&self, cxt: &mut SemanticContext) -> Result<(), SemanticErr> {
        for item in self.items.iter() {
            item.analyze(cxt)?;
        }
        Ok(())
    }
}
