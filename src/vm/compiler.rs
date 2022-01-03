use crate::node::node::*;
use crate::node::semantic_analyzer::{Context, VarContext};
use crate::token::{Token, Value};

use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum V {
    Direct(usize),
    Indirect(usize),
    Value(i32),
    NoWhere,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Instrument {
    Mov(V, V),
    // Op(Token, V, V),
    BinOp(Token, V, V, V),
    Print(V),
}

pub struct Program {
    pub inss: Vec<Instrument>,
    vars: HashMap<usize, VarContext>,
    mem_offset: usize,
    global: bool,
    stack_off: usize,
}

impl Program {
    pub fn new(cxt: &Context) -> Self {
        Program {
            inss: Vec::new(),
            vars: cxt.freeze(),
            mem_offset: 0,
            global: true,
            stack_off: 0,
        }
    }

    fn update_offset(&mut self, var: &VarNd) {
        let id = *var.id.borrow();
        let cxt = self.vars[&id];
        if var.declared() {
            self.mem_offset = cxt.mem_offset + 1;
        }
    }

    fn reset_stack_off(&mut self) {
        self.stack_off = 0;
    }

    fn get_v_from_var(&self, var: &VarNd) -> V {
        let id = *var.id.borrow();
        let cxt = self.vars[&id];
        println!("get {} {:?} {:?}", id, var, cxt);
        if cxt.scope_id == 0 {
            V::Direct(cxt.mem_offset)
        } else {
            V::Indirect(cxt.mem_offset)
        }
    }

    fn get_v_from_off(&self, off: usize) -> V {
        if self.global {
            V::Direct(off)
        } else {
            V::Indirect(off)
        }
    }

    fn push(&mut self, v: Value) -> V {
        let off = self.mem_offset + self.stack_off;
        self.stack_off += 1;
        let mem_v = match v {
            Value::Int(v) => v,
            Value::Bool(v) => {
                if v {
                    1
                } else {
                    0
                }
            }
        };
        let v = self.get_v_from_off(off);
        self.inss.push(Instrument::Mov(v, V::Value(mem_v)));
        v
    }

    fn push_var(&mut self, var_v: V) -> V {
        println!("pushvar {:?}", var_v);
        let off = self.mem_offset + self.stack_off;
        self.stack_off += 1;
        let v = self.get_v_from_off(off);
        self.inss.push(Instrument::Mov(v, var_v));
        v
    }

    fn pop(&mut self, v: V) -> V {
        self.stack_off -= 1;
        let off = self.mem_offset + self.stack_off;
        self.inss.push(Instrument::Mov(v, self.get_v_from_off(off)));
        v
    }

    fn bin_op(&mut self, op: Token) -> V {
        let v2 = self.get_v_from_off(self.mem_offset + self.stack_off - 1);
        let v1 = self.get_v_from_off(self.mem_offset + self.stack_off - 2);
        self.inss.push(Instrument::BinOp(op, v1, v2, v1));
        self.stack_off -= 1;
        v1
    }

    fn print_var(&mut self, var: &VarNd) -> V {
        let v = self.get_v_from_var(var);
        self.inss.push(Instrument::Print(v));
        v
    }
}

pub trait Compiler {
    fn compile(&self, prog: &mut Program) -> V;
}

impl Compiler for FactorNd {
    fn compile(&self, prog: &mut Program) -> V {
        match self {
            FactorNd::Var(n) => n.compile(prog),
            FactorNd::Value(v) => prog.push(v.clone()),
            FactorNd::Expr(n) => n.compile(prog),
        }
    }
}

impl Compiler for TermNd {
    fn compile(&self, prog: &mut Program) -> V {
        if self.b.is_some() {
            let (b, op) = self.b.as_ref().unwrap();
            self.a.compile(prog);
            b.compile(prog);
            prog.bin_op(op.clone())
        } else {
            self.a.compile(prog)
        }
    }
}

impl Compiler for ExprNd {
    fn compile(&self, prog: &mut Program) -> V {
        if self.b.is_some() {
            let (b, op) = self.b.as_ref().unwrap();
            self.a.compile(prog);
            b.compile(prog);
            prog.bin_op(op.clone())
        } else {
            self.a.compile(prog)
        }
    }
}

// No Push Here
impl Compiler for VarNd {
    fn compile(&self, prog: &mut Program) -> V {
        let v = prog.get_v_from_var(self);
        if self.declared() {
            prog.update_offset(self);
            V::NoWhere
        } else {
            prog.push_var(v)
        }
    }
}

impl Compiler for StmtNd {
    fn compile(&self, prog: &mut Program) -> V {
        let mut w = V::NoWhere;
        if self.expr.is_some() {
            w = self.expr.as_ref().unwrap().compile(prog);
        }
        if self.var.is_some() {
            let var = self.var.as_ref().unwrap();
            let v = prog.get_v_from_var(var);
            if self.expr.is_none() {
                if var.declared() {
                    // just push default
                    prog.push(Value::Int(0));
                    w = var.compile(prog);
                } else {
                    // print
                    return prog.print_var(var);
                }
            } else {
                if var.declared() {
                    // just update offset
                    w = var.compile(prog);
                } else {
                    // mov to v
                    w = prog.pop(v);
                }
            }
        }
        w
    }
}

impl Compiler for RootNd {
    fn compile(&self, prog: &mut Program) -> V {
        for st in self.stmts.iter() {
            st.compile(prog);
            prog.reset_stack_off();
        }
        V::NoWhere
    }
}
