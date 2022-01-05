use super::VarContext;
use crate::core::{get_type_size, SemanticErr, Type};

use std::collections::HashMap;

#[derive(Clone)]
pub struct ScopeContext {
    id: u32,
    pub offset: usize,
    cur: usize,
    vars: HashMap<String, VarContext>,
}

impl ScopeContext {
    pub fn new(id: u32, offset: usize) -> Self {
        ScopeContext {
            id: id,
            offset: offset,
            cur: 0,
            vars: HashMap::new(),
        }
    }

    fn cur_offet(&self) -> usize {
        self.cur + self.offset
    }

    pub fn fetch(&self, name: &str) -> Option<VarContext> {
        match self.vars.get(name) {
            Some(n) => Some(*n),
            None => None,
        }
    }

    pub fn declare(&mut self, name: &str, ty: Type, var_id: u32) -> Result<VarContext, SemanticErr> {
        if self.vars.contains_key(name) {
            Err(SemanticErr::DoubleDeclare)
        } else {
            let sz = get_type_size(ty);
            let var_cxt = VarContext::new(var_id, self.id, self.cur_offet(), sz);
            self.cur += sz;
            self.vars.insert(name.to_owned(), var_cxt);
            Ok(var_cxt)
        }
    }
}
