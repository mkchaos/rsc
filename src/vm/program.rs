use crate::node::*;
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

    pub fn update_offset(&mut self, var: &VarNd) {
        let id = *var.id.borrow();
        let cxt = self.vars[&id];
        if var.declared() {
            self.mem_offset = cxt.mem_offset + 1;
        }
    }

    pub fn reset_stack_off(&mut self) {
        self.stack_off = 0;
    }

    pub fn get_v_from_var(&self, var: &VarNd) -> V {
        let id = *var.id.borrow();
        let cxt = self.vars[&id];
        if cxt.scope_id == 0 {
            V::Direct(cxt.mem_offset)
        } else {
            V::Indirect(cxt.mem_offset)
        }
    }

    pub fn get_v_from_off(&self, off: usize) -> V {
        if self.global {
            V::Direct(off)
        } else {
            V::Indirect(off)
        }
    }

    pub fn push(&mut self, v: Value) -> V {
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
        self.inss.push(Instrument::Mov(V::Value(mem_v), v));
        v
    }

    pub fn push_var(&mut self, var_v: V) -> V {
        let off = self.mem_offset + self.stack_off;
        self.stack_off += 1;
        let v = self.get_v_from_off(off);
        self.inss.push(Instrument::Mov(var_v, v));
        v
    }

    pub fn pop(&mut self, v: V) -> V {
        self.stack_off -= 1;
        let off = self.mem_offset + self.stack_off;
        self.inss.push(Instrument::Mov(self.get_v_from_off(off), v));
        v
    }

    pub fn bin_op(&mut self, op: Token) -> V {
        let v2 = self.get_v_from_off(self.mem_offset + self.stack_off - 1);
        let v1 = self.get_v_from_off(self.mem_offset + self.stack_off - 2);
        self.inss.push(Instrument::BinOp(op, v1, v2, v1));
        self.stack_off -= 1;
        v1
    }

    pub fn print_var(&mut self, var: &VarNd) -> V {
        let v = self.get_v_from_var(var);
        self.inss.push(Instrument::Print(v));
        v
    }
}