use crate::core::*;
use crate::context::ProgContext;

pub trait Compiler {
    fn compile(&self, prog: &mut ProgContext);
}

impl Compiler for FactorNd {
    fn compile(&self, prog: &mut ProgContext) {
        match self {
            FactorNd::Var(n) => n.compile(prog),
            FactorNd::Value(v) => {},
            FactorNd::Func(n) => n.compile(prog),
        }
    }
}

impl Compiler for ExprNd {
    fn compile(&self, prog: &mut ProgContext) {}
}

impl Compiler for VarNd {
    fn compile(&self, prog: &mut ProgContext) {}
}

impl Compiler for AssignNd {
    fn compile(&self, prog: &mut ProgContext) {}
}

impl Compiler for DeclareNd {
    fn compile(&self, prog: &mut ProgContext) {}
}

impl Compiler for StmtNd {
    fn compile(&self, prog: &mut ProgContext) {
        match self {
            StmtNd::Assign(n) => n.compile(prog),
            StmtNd::Declare(n) => n.compile(prog),
            StmtNd::Expr(n) => n.compile(prog),
            StmtNd::Print(n) => n.compile(prog),
            _ => {}
        }
    }
}

impl Compiler for ItemNd {
    fn compile(&self, prog: &mut ProgContext) {
        match self {
            ItemNd::Block(n) => n.compile(prog),
            ItemNd::Stmt(n) => n.compile(prog),
        }
    }
}

impl Compiler for BlockNd {
    fn compile(&self, prog: &mut ProgContext) {
        for it in self.items.iter() {
            it.compile(prog);
        }
    }
}

impl Compiler for FuncNd {
    fn compile(&self, prog: &mut ProgContext) {
        // prog.enter_func(&self.head.name.name);
        // self.block.compile(prog);
        // prog.ret();
        // prog.exit_func();
    }
}

impl Compiler for FuncCallNd {
    fn compile(&self, prog: &mut ProgContext) {
        // prog.enter_func(&self.head.name.name);
        // self.block.compile(prog);
        // prog.ret();
        // prog.exit_func();
    }
}

impl Compiler for GItemNd {
    fn compile(&self, prog: &mut ProgContext) {
        match self {
            GItemNd::Func(n) => n.compile(prog),
            GItemNd::Declare(n) => n.compile(prog),
        }
    }
}

impl Compiler for RootNd {
    fn compile(&self, prog: &mut ProgContext) {
        for it in self.items.iter() {
            it.compile(prog);
        }
    }
}
