use crate::core::*;

pub trait Compiler {
    fn compile(&self, prog: &mut ProgContext);
}

// impl Compiler for FactorNd {
//     fn compile(&self, prog: &mut ProgContext) {
//         match self {
//             FactorNd::Var(n) => n.compile(prog),
//             FactorNd::Value(v) => Some(prog.push_value(v.clone())),
//             FactorNd::Expr(n) => n.compile(prog),
//         }
//     }
// }

// impl Compiler for ExprNd {
//     fn compile(&self, prog: &mut ProgContext) {
//         if self.b.is_some() {
//             let (b, op) = self.b.as_ref().unwrap();
//             self.a.compile(prog);
//             b.compile(prog);
//             Some(prog.bin_op(op.clone()))
//         } else {
//             self.a.compile(prog)
//         }
//     }
// }

// impl Compiler for VarNd {
//     fn compile(&self, prog: &mut ProgContext) {
//         let v = prog.get_v_from_var(self);
//         if self.declared() {
//             prog.update_offset(self);
//             None
//         } else {
//             Some(prog.push_var(v))
//         }
//     }
// }

// impl Compiler for StmtNd {
//     fn compile(&self, prog: &mut ProgContext) {
//         let mut w = None;
//         if self.expr.is_some() {
//             w = self.expr.as_ref().unwrap().compile(prog);
//         }
//         if self.var.is_some() {
//             let var = self.var.as_ref().unwrap();
//             let v = prog.get_v_from_var(var);
//             if self.expr.is_none() {
//                 if var.declared() {
//                     // just push default
//                     prog.push_value(Value::Int(0));
//                     w = var.compile(prog);
//                 } else {
//                     // print
//                     return Some(prog.print_var(var));
//                 }
//             } else {
//                 if var.declared() {
//                     // just update offset
//                     w = var.compile(prog);
//                 } else {
//                     // mov to v
//                     w = Some(prog.pop(v));
//                 }
//             }
//         }
//         w
//     }
// }

// impl Compiler for ItemNd {
//     fn compile(&self, prog: &mut ProgContext) {
//         match self {
//             ItemNd::Block(n) => n.compile(prog),
//             ItemNd::Stmt(n) => n.compile(prog),
//         }
//     }
// }

// impl Compiler for BlockNd {
//     fn compile(&self, prog: &mut ProgContext) {
//         for it in self.items.iter() {
//             it.compile(prog);
//             prog.reset_stack_off();
//         }
//         None
//     }
// }
// impl Compiler for FuncNd {
//     fn compile(&self, prog: &mut ProgContext) {
//         prog.enter_func(&self.head.name.name);
//         self.block.compile(prog);
//         prog.ret();
//         prog.exit_func();
//         None
//     }
// }

// impl Compiler for GItemNd {
//     fn compile(&self, prog: &mut ProgContext) {
//         match self {
//             GItemNd::Func(n) => n.compile(prog),
//             GItemNd::Stmt(n) => n.compile(prog),
//         }
//     }
// }

// impl Compiler for RootNd {
//     fn compile(&self, prog: &mut ProgContext) {
//         for it in self.items.iter() {
//             match it {
//                 GItemNd::Stmt(n) => n.compile(prog),
//                 _ => None,
//             };
//         }
//         for it in self.items.iter() {
//             match it {
//                 GItemNd::Func(n) => n.compile(prog),
//                 _ => None,
//             };
//         }
//         None
//     }
// }
