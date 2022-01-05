use super::scope::ScopeContext;
use super::var::VarContext;
use crate::core::*;

#[derive(Clone)]
pub struct SemanticContext {
    cur: usize,
    func_offset: usize,
    vars: Vec<VarContext>,
    scopes: Vec<ScopeContext>,
    scope_id_stack: Vec<u32>,
}

impl SemanticContext {
    pub fn new() -> Self {
        SemanticContext {
            cur: 0,
            func_offset: 0,
            vars: Vec::new(),
            scopes: vec![ScopeContext::new(0, 0)],
            scope_id_stack: Vec::new(),
        }
    }

    fn in_func(&self) -> bool {
        self.scope_id_stack.len() == 0
    }

    fn offset(&self) -> usize {
        if self.in_func() {
            self.cur - self.func_offset
        } else {
            self.cur
        }
    }

    fn cur_scope_id(&self) -> u32 {
        if self.scope_id_stack.len() == 0 {
            0
        } else {
            *self.scope_id_stack.last().unwrap()
        }
    }

    pub fn enter_scope(&mut self) {
        if self.cur_scope_id() == 0 {
            self.func_offset = self.cur;
        }
        let id = self.scopes.len() as u32;
        let scope = ScopeContext::new(id, self.offset());
        self.scopes.push(scope);
        self.scope_id_stack.push(id);
    }

    pub fn exit_scope(&mut self) {
        let id = self.scope_id_stack.pop().expect("scope parse error");
        let off = self.scopes[id as usize].offset;
        if self.in_func() {
            self.cur = self.func_offset + self.scopes[id as usize].offset;
        }
    }

    pub fn declare(&mut self, ty: Type, name: &str) -> Result<VarContext, SemanticErr> {
        let ty_size = get_type_size(ty);
        let id = self.cur_scope_id() as usize;
        let scope = &mut self.scopes[id];
        let cxt = scope.declare(name, ty, self.vars.len() as u32)?;
        self.vars.push(cxt);
        self.cur += ty_size;
        Ok(cxt)
    }

    pub fn fetch(&self, name: &str) -> Result<VarContext, SemanticErr> {
        for id in self.scope_id_stack.iter().rev() {
            let scope = &self.scopes[*id as usize];
            if let Some(cxt) = scope.fetch(name) {
                return Ok(cxt);
            }
        }
        match self.scopes[0].fetch(name) {
            Some(cxt) => Ok(cxt),
            None => Err(SemanticErr::NoDeclareUse),
        }
    }
}
