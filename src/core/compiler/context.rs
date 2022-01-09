use crate::core::analyzer::Semantic;
use crate::core::types::{Code, CodeAddr, Layout, MemAddr, Type};

use std::collections::HashMap;

pub struct Program {
    pub memory: Vec<i32>,
    pub codes: Vec<Code>,
    pub start_pc: usize,
}

impl Program {
    pub fn new(cxt: Context) -> Self {
        // Link
        let mut codes = Vec::new();
        macro_rules! link_address {
            ($addr: ident) => {
                match $addr {
                    CodeAddr::NameStart(id) => {
                        CodeAddr::Direct(cxt.code_layout.get(id).unwrap().offset as usize)
                    }
                    CodeAddr::NameEnd(id) => {
                        CodeAddr::Direct(cxt.code_layout.get(id).unwrap().last() as usize)
                    }
                    _ => $addr.clone(),
                }
            };
        }
        for c in cxt.codes.iter() {
            let cc = match c {
                Code::Call(addr, n) => Code::Call(link_address!(addr), *n),
                Code::Jump(addr) => Code::Jump(link_address!(addr)),
                Code::CondJump(addr) => Code::CondJump(link_address!(addr)),
                _ => c.clone(),
            };
            codes.push(cc);
        }

        let pc = cxt.code_layout[&cxt.s_info.main_func_id].offset;
        Program {
            memory: cxt.memory,
            codes: codes,
            start_pc: pc,
        }
    }
}

pub struct Context {
    code_layout: HashMap<u32, Layout>,
    s_info: Semantic,
    codes: Vec<Code>,
    memory: Vec<i32>,
    func_id: u32,
}

impl Context {
    pub fn new(info: Semantic) -> Self {
        Context {
            code_layout: HashMap::new(),
            s_info: info,
            codes: Vec::new(),
            memory: Vec::new(),
            func_id: 0,
        }
    }

    pub fn enter_func(&mut self, id: u32) {
        self.func_id = id;
        self.enter(id);
    }

    pub fn exit_func(&mut self) {
        self.exit(self.func_id);
        match self.s_info.funcs[&self.func_id].ty.clone() {
            Type::Func(v) => {
                let ret_ty = v.last().unwrap();
                if ret_ty.clone() == Type::Int {
                    self.add_code(Code::PushValue(0));
                    self.add_code(Code::Ret(1));
                } else {
                    self.add_code(Code::Ret(0));
                }
            }
            _ => panic!("not func type"),
        }
        self.func_id = 0;
    }

    pub fn get_cur(&self) -> usize {
        self.codes.len()
    }

    pub fn enter(&mut self, id: u32) -> usize {
        let cur = self.get_cur();
        self.code_layout.insert(id, Layout::new(cur));
        cur
    }

    pub fn get_scope_size(&self, id: u32) -> usize {
        self.s_info.mem_layout[id as usize].size
    }

    pub fn exit(&mut self, id: u32) -> usize {
        let cur = self.get_cur();
        self.code_layout.get_mut(&id).unwrap().end(cur);
        cur
    }

    pub fn add_code(&mut self, code: Code) {
        self.codes.push(code);
    }

    fn get_var_addr(&self, id: u32) -> MemAddr {
        let global = self.s_info.vars[&id].is_global();
        if global {
            MemAddr::Direct(self.s_info.mem_layout[id as usize].offset)
        } else {
            MemAddr::Indirect(self.s_info.mem_layout[id as usize].offset)
        }
    }

    pub fn push(&mut self, id: u32) {
        let code = Code::Push(self.get_var_addr(id));
        self.codes.push(code);
    }

    pub fn pop(&mut self, id: u32) {
        let code = Code::PopMov(self.get_var_addr(id));
        self.codes.push(code);
    }

    pub fn call(&mut self, id: u32) {
        let num = match self.s_info.funcs[&id].ty.clone() {
            Type::Func(v) => v.len() - 1,
            _ => panic!("Not func type"),
        };
        let code = Code::Call(CodeAddr::NameStart(id), num);
        self.codes.push(code);
    }

    pub fn add_memory(&mut self, v: i32) {
        self.memory.push(v);
    }
}
