use crate::core::analyzer::SemanticInfo;
use crate::core::types::{Code, CodeAddr, Layout, MemAddr, Op, Token, Type, Value};

use std::collections::HashMap;

pub struct Program {
    pub memory: Vec<i32>,
    pub codes: Vec<Code>,
}

impl Program {
    pub fn new(cxt: Context) -> Self {
        // Link
        let mut codes = Vec::new();
        macro_rules! link_address {
            ($addr: ident, $idx:ident) => {
                match $addr {
                    CodeAddr::NameStart(id) => {
                        let off = cxt.code_layout.get(id).unwrap().offset as isize - $idx as isize;
                        CodeAddr::Offset(off)
                    }
                    CodeAddr::NameEnd(id) => {
                        let off = cxt.code_layout.get(id).unwrap().last() as isize - $idx as isize;
                        CodeAddr::Offset(off)
                    }
                    _ => $addr.clone(),
                }
            };
        }
        for (idx, c) in cxt.codes.iter().enumerate() {
            println!("idx {:?}", idx);
            let cc = match c {
                Code::Call(addr) => Code::Call(link_address!(addr, idx)),
                Code::Jump(addr) => Code::Jump(link_address!(addr, idx)),
                Code::CondJump(addr) => Code::CondJump(link_address!(addr, idx)),
                _ => c.clone(),
            };
            codes.push(cc);
        }
        Program {
            memory: cxt.memory,
            codes: codes,
        }
    }
}

pub struct Context {
    code_layout: HashMap<u32, Layout>,
    s_info: SemanticInfo,
    codes: Vec<Code>,
    memory: Vec<i32>,
    func_id: u32,
}

impl Context {
    pub fn new(info: SemanticInfo) -> Self {
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
        println!("In {:?} {:?}", self.func_id, self.get_cur());
        self.enter(id);
    }

    pub fn exit_func(&mut self) {
        self.exit(self.func_id);
        println!("Out {:?} {:?}", self.func_id, self.get_cur());
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
        let code = Code::Pop(self.get_var_addr(id));
        self.codes.push(code);
    }

    pub fn call(&mut self, id: u32) {
        let code = Code::Call(CodeAddr::NameStart(id));
        self.codes.push(code);
    }

    pub fn add_memory(&mut self, v: i32) {
        self.memory.push(v);
    }
}
