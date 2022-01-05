use crate::core::{Token, Value, Op, Addr, Code, SemanticContext, VarContext};

use std::collections::HashMap;


pub struct ProgContext {
    global_codes: Vec<Code>,
    func_codes: Vec<Code>,
    // var_cxts: HashMap<usize, VarContext>,
}

impl ProgContext {
    pub fn new(cxt: &SemanticContext) -> Self {
        ProgContext {
            global_codes: Vec::new(),
            func_codes: Vec::new(),
            // var_cxts: cxt.freeze(),
        }
    }

    // pub fn gen_codes(&self) {

    // }

    pub fn get_codes(&mut self) -> &mut Vec<Code> {
        &mut self.func_codes
    }

    pub fn debug(&self) {
        for it in self.global_codes.iter() {
            println!("{:?}", it);
        }
        for it in self.func_codes.iter() {
            println!("{:?}", it);
        }
    }

    pub fn enter_func(&mut self) {
    }

    pub fn exit_func(&mut self) {

    }

    pub fn push_value(&mut self, v: Value) {

    }

    pub fn push_var(&mut self, id: usize) {}

    pub fn op(&mut self, t: Token) {}

    pub fn get_code_cur(&self) -> usize {
        0
    }

    pub fn pop(&mut self) {

    }

    pub fn ret(&mut self) {}

    pub fn call(&mut self, id: usize) {}

    pub fn print(&mut self) {}
}
