use crate::node::*;
use crate::token::{Token, Value};

use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum V {
    Direct(usize),
    Indirect(usize),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Instrument {
    Mov(V, V),
    Set(V, i32),
    // Op(Token, V, V),
    BinOp(Token, V, V, V),
    Print(V),
    Ret,
    Call,
}

pub struct Program {
    pub inss: Vec<Instrument>,
    funcs: HashMap<String, usize>,
    vars: HashMap<usize, VarContext>,
    mem_offset: usize,
    func_offset: usize,
    stack_off: usize,
}

impl Program {
    pub fn new(cxt: &Context) -> Self {
        Program {
            inss: Vec::new(),
            funcs: HashMap::new(),
            vars: cxt.freeze(),
            mem_offset: 0,
            func_offset: 0,
            stack_off: 0,
        }
    }

    fn in_func(&self) -> bool {
        self.func_offset != 0
    }

    pub fn enter_func(&mut self, name: &str) {
        self.func_offset = self.mem_offset;
        self.funcs.insert(name.to_owned(), self.func_offset);
    }

    pub fn exit_func(&mut self) {
        self.func_offset = 0;
    }

    pub fn main_entry(&self) -> usize {
        self.funcs["main"]
    }

    pub fn ret(&mut self) {
        self.inss.push(Instrument::Ret);
    }

    pub fn update_offset(&mut self, var: &VarNd) {
        let id = *var.id.borrow();
        let cxt = self.vars[&id];
        if var.declared() {
            self.mem_offset = cxt.mem_offset + self.func_offset  + 1;
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
        if self.in_func() {
            V::Indirect(off)
        } else {
            V::Direct(off)
        }
    }

    pub fn get_cur_off(&self) -> usize {
        self.mem_offset + self.stack_off - self.func_offset
    }

    pub fn push_value(&mut self, v: Value) -> V {
        let off = self.get_cur_off();
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
        self.inss.push(Instrument::Set(v, mem_v));
        v
    }

    pub fn push_var(&mut self, var_v: V) -> V {
        let off = self.get_cur_off();
        self.stack_off += 1;
        let v = self.get_v_from_off(off);
        self.inss.push(Instrument::Mov(var_v, v));
        v
    }

    pub fn pop(&mut self, v: V) -> V {
        self.stack_off -= 1;
        let off = self.get_cur_off();
        self.inss.push(Instrument::Mov(self.get_v_from_off(off), v));
        v
    }

    pub fn bin_op(&mut self, op: Token) -> V {
        let off = self.get_cur_off();
        let v2 = self.get_v_from_off(off - 1);
        let v1 = self.get_v_from_off(off - 2);
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
