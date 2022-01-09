use super::{Compiler, Context};
use crate::core::types::nodes::*;
use crate::core::types::{CalcItem, Code, CodeAddr, Type, Value};

impl Compiler for FactorNd {
    fn compile(&self, cxt: &mut Context) {
        match self {
            FactorNd::Var(n) => n.compile(cxt),
            FactorNd::Value(v) => {
                if let Value::Int(num) = v {
                    cxt.add_code(Code::PushValue(*num));
                }
            }
            FactorNd::Func(n) => n.compile(cxt),
        }
    }
}

impl Compiler for ExprNd {
    fn compile(&self, cxt: &mut Context) {
        for it in self.stack.iter() {
            match it {
                CalcItem::Op(op) => {
                    cxt.add_code(Code::Op(op.clone()));
                }
                CalcItem::Factor(f) => {
                    f.compile(cxt);
                }
            }
        }
    }
}

impl Compiler for VarNd {
    fn compile(&self, cxt: &mut Context) {
        cxt.push(self.get_id());
    }
}

impl Compiler for AssignNd {
    fn compile(&self, cxt: &mut Context) {
        self.expr.compile(cxt);
        cxt.pop(self.var.get_id());
    }
}

impl Compiler for DeclareNd {
    fn compile(&self, cxt: &mut Context) {
        match &self.expr {
            Some(e) => {
                e.compile(cxt);
            }
            None => {
                cxt.add_code(Code::PushValue(0));
            }
        };
    }
}

impl Compiler for StmtNd {
    fn compile(&self, cxt: &mut Context) {
        match self {
            StmtNd::Assign(n) => {
                n.compile(cxt);
            }
            StmtNd::Declare(n) => n.compile(cxt),
            StmtNd::Expr(n) => {
                n.compile(cxt);
                cxt.add_code(Code::Pop(1));
            }
            StmtNd::Print(n) => {
                cxt.push(n.get_id());
                cxt.add_code(Code::Print);
                cxt.add_code(Code::Pop(1));
            }
            _ => {}
        }
    }
}

impl Compiler for IfNd {
    fn compile(&self, cxt: &mut Context) {
        cxt.enter(self.get_id());
        self.expr.compile(cxt);
        cxt.add_code(Code::CondJump(CodeAddr::NameEnd(self.get_id())));
        self.item.compile(cxt);
        cxt.exit(self.get_id());
        if self.els.is_some() {
            self.els.as_ref().unwrap().compile(cxt);
        }
    }
}

impl Compiler for ElsNd {
    fn compile(&self, cxt: &mut Context) {
        match self {
            ElsNd::If(n) => n.compile(cxt),
            ElsNd::Item(n) => n.compile(cxt),
        }
    }
}

impl Compiler for WhileNd {
    fn compile(&self, cxt: &mut Context) {
        cxt.enter(self.get_id());
        self.expr.compile(cxt);
        cxt.add_code(Code::CondJump(CodeAddr::NameEnd(self.get_id())));
        self.item.compile(cxt);
        cxt.add_code(Code::Jump(CodeAddr::NameStart(self.get_id())));
        cxt.exit(self.get_id());
    }
}

impl Compiler for BreakNd {
    fn compile(&self, cxt: &mut Context) {
        cxt.add_code(Code::Pop(self.get_pop_off()));
        cxt.add_code(Code::Jump(CodeAddr::NameEnd(self.get_id())));
    }
}

impl Compiler for ContinueNd {
    fn compile(&self, cxt: &mut Context) {
        cxt.add_code(Code::Pop(self.get_pop_off()));
        cxt.add_code(Code::Jump(CodeAddr::NameStart(self.get_id())));
    }
}

impl Compiler for ReturnNd {
    fn compile(&self, cxt: &mut Context) {
        match self.expr.as_ref() {
            Some(n) => {
                n.compile(cxt);
                cxt.add_code(Code::Ret(self.get_sz()));
            }
            None => {
                let sz = self.get_sz();
                for _ in 0..sz {
                    cxt.add_code(Code::PushValue(0));
                }
                cxt.add_code(Code::Ret(0));
            }
        }
    }
}

impl Compiler for ItemNd {
    fn compile(&self, cxt: &mut Context) {
        match self {
            ItemNd::Block(n) => n.compile(cxt),
            ItemNd::Stmt(n) => n.compile(cxt),
            ItemNd::If(n) => n.compile(cxt),
            ItemNd::While(n) => n.compile(cxt),
            ItemNd::Break(n) => n.compile(cxt),
            ItemNd::Continue(n) => n.compile(cxt),
            ItemNd::Return(n) => n.compile(cxt),
        }
    }
}

impl Compiler for BlockNd {
    fn compile(&self, cxt: &mut Context) {
        cxt.enter(self.get_id());
        for it in self.items.iter() {
            it.compile(cxt);
        }
        let sz = cxt.get_scope_size(self.get_id());
        cxt.add_code(Code::Pop(sz));
        cxt.exit(self.get_id());
    }
}

impl Compiler for FuncNd {
    fn compile(&self, cxt: &mut Context) {
        if self.is_impl() {
            let id = self.var.get_id();
            cxt.enter_func(id);
            self.block.as_ref().unwrap().compile(cxt);
            cxt.exit_func();
        }
    }
}

impl Compiler for FuncCallNd {
    fn compile(&self, cxt: &mut Context) {
        for p in self.params.iter() {
            p.compile(cxt);
        }
        let id = self.var.get_id();
        cxt.call(id);
    }
}

impl Compiler for GItemNd {
    fn compile(&self, cxt: &mut Context) {
        match self {
            GItemNd::Func(n) => n.compile(cxt),
            GItemNd::Declare(n) => {
                // add memory
                if n.ty == Type::Int {
                    let v = n.try_retrieve_const().unwrap();
                    cxt.add_memory(v);
                }
            }
        }
    }
}

impl Compiler for RootNd {
    fn compile(&self, cxt: &mut Context) {
        for it in self.items.iter() {
            it.compile(cxt);
        }
    }
}
