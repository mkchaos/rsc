use crate::core::analyzer::SemanticInfo;
use crate::core::types::{Code, CodeAddr, Layout, MemAddr, Op, Token, Type, Value};

use std::collections::HashMap;

pub struct Program {
    pub memory: Vec<(Type, Value)>,
    pub codes: Vec<Code>,
}

impl Program {
    pub fn new(cxt: Context) -> Self {
        // Link
        let mut codes = Vec::new();
        macro_rules! link_address {
            ($addr: ident, $idx:ident) => {
                match $addr {
                    CodeAddr::Start(id) => {
                        let off = cxt.code_layout.get(id).unwrap().offset - $idx;
                        CodeAddr::Offset(off)
                    }
                    CodeAddr::End(id) => {
                        let off = cxt.code_layout.get(id).unwrap().last() - $idx;
                        CodeAddr::Offset(off)
                    }
                    _ => $addr.clone(),
                }
            };
        }
        for (_, v) in cxt.funcs {
            for (idx, c) in v.iter().enumerate() {
                let cc = match c {
                    Code::Call(addr) => Code::Call(link_address!(addr, idx)),
                    Code::Jump(addr) => Code::Jump(link_address!(addr, idx)),
                    Code::CondJump(addr) => Code::CondJump(link_address!(addr, idx)),
                    _ => c.clone(),
                };
                codes.push(cc);
            }
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
    funcs: HashMap<u32, Vec<Code>>,
    memory: Vec<(Type, Value)>,
    func_id: u32,
}

impl Context {
    pub fn new(info: SemanticInfo) -> Self {
        Context {
            code_layout: HashMap::new(),
            s_info: info,
            funcs: HashMap::new(),
            memory: Vec::new(),
            func_id: 0,
        }
    }

    pub fn enter_func(&mut self, id: u32) {
        self.func_id = id;
        self.funcs.insert(id, Vec::new());
    }

    pub fn exit_func(&mut self) {
        self.func_id = 0;
    }

    fn in_func(&self) -> bool {
        self.func_id != 0
    }

    pub fn get_cur(&self) -> usize {
        self.funcs[&self.func_id].len()
    }

    pub fn start(&mut self, id: u32) -> usize {
        let cur = self.get_cur();
        self.code_layout.insert(id, Layout::new(cur));
        cur
    }

    pub fn end(&mut self, id: u32) -> usize {
        let cur = self.get_cur();
        self.code_layout.get_mut(&id).unwrap().end(cur);
        cur
    }

    pub fn add_code(&mut self, code: Code) {
        self.funcs.get_mut(&self.func_id).unwrap().push(code);
    }

    pub fn push(&mut self, id: u32) {
        // self.funcs.get_mut(&self.func_id).unwrap().push(code);
    }

    pub fn pop(&mut self, id: u32) {
        // self.funcs.get_mut(&self.func_id).unwrap().push(code);
    }

    pub fn call(&mut self, id: u32) {
        // self.funcs.get_mut(&self.func_id).unwrap().push(code);
    }

    pub fn add_memory(&mut self, ty: Type, v: Value) {
        self.memory.push((ty, v));
    }
}
