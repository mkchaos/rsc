use super::node::*;
use crate::token::{get_value_type, Type};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SemanticErr {
    DoubleDeclare,
    NoDeclareUse,
    MismatchType,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct VarContext {
    pub id: usize,
    pub scope_id: usize,
    pub mem_offset: usize,
    pub ty: Type,
}

#[derive(Clone)]
pub struct ScopeContext {
    id: usize,
    mem_offset: usize,
    mem_cur: usize,
    vars: HashMap<String, VarContext>,
}

impl ScopeContext {
    fn new(id: usize, off: usize) -> Self {
        ScopeContext {
            id: id,
            mem_offset: off,
            mem_cur: 0,
            vars: HashMap::new(),
        }
    }

    fn fetch(&self, name: &str) -> Option<VarContext> {
        match self.vars.get(name) {
            Some(n) => Some(*n),
            None => None,
        }
    }

    fn declare(&mut self, name: &str, ty: &Type, vid: usize) -> Result<VarContext, SemanticErr> {
        if self.vars.contains_key(name) {
            Err(SemanticErr::DoubleDeclare)
        } else {
            let var_cxt = VarContext {
                id: vid,
                scope_id: self.id,
                mem_offset: self.mem_cur,
                ty: ty.clone(),
            };
            self.mem_cur += 1;
            self.vars.insert(name.to_owned(), var_cxt);
            Ok(var_cxt)
        }
    }
}

pub struct Context {
    cur_scope_id: usize,
    scope_stack: Vec<usize>,
    pub cur_var_id: usize,
    pub mem_cur: usize,
    scopes: Vec<ScopeContext>,
}

#[allow(dead_code)]
impl Context {
    pub fn new() -> Self {
        let mut scopes = Vec::new();
        scopes.push(ScopeContext::new(0, 0));
        Context {
            cur_scope_id: 0,
            scope_stack: Vec::new(),
            cur_var_id: 0,
            mem_cur: 0,
            scopes: scopes,
        }
    }

    pub fn enter_scope(&mut self) {
        self.scope_stack.push(self.cur_scope_id);
        self.cur_scope_id = self.scopes.len();
        self.scopes
            .push(ScopeContext::new(self.cur_scope_id, self.mem_cur));
    }

    pub fn exit_scope(&mut self) {
        self.cur_scope_id = self.scope_stack.pop().expect("scope parse error");
        self.mem_cur = self.scopes[self.cur_scope_id].mem_offset;
    }

    pub fn declare(&mut self, var: &VarNd) -> Result<VarContext, SemanticErr> {
        let scope = &mut self.scopes[self.cur_scope_id];
        let cxt = scope.declare(&var.name, &var.ty.unwrap(), self.cur_var_id)?;
        self.mem_cur = scope.mem_cur;
        self.cur_var_id += 1;
        Ok(cxt)
    }

    pub fn fetch(&self, var: &VarNd) -> Result<VarContext, SemanticErr> {
        match self.scopes[self.cur_scope_id].fetch(&var.name) {
            Some(cxt) => return Ok(cxt),
            _ => {}
        }
        for sid in self.scope_stack.iter().rev() {
            let scope = &self.scopes[*sid];
            println!("{:?}", scope.vars.len());
            match scope.fetch(&var.name) {
                Some(cxt) => return Ok(cxt),
                _ => {}
            }
        }
        Err(SemanticErr::NoDeclareUse)
    }
}

pub trait SemanticAnalyzer {
    fn analyze(&self, cxt: &mut Context) -> Result<Type, SemanticErr>;
}

impl SemanticAnalyzer for FactorNd {
    fn analyze(&self, cxt: &mut Context) -> Result<Type, SemanticErr> {
        match self {
            FactorNd::Var(n) => n.analyze(cxt),
            FactorNd::Value(n) => Ok(get_value_type(*n)),
            FactorNd::Expr(n) => n.analyze(cxt),
        }
    }
}

impl SemanticAnalyzer for TermNd {
    fn analyze(&self, cxt: &mut Context) -> Result<Type, SemanticErr> {
        let ty = self.a.analyze(cxt)?;
        if ty != Type::Int {
            return Err(SemanticErr::MismatchType);
        }
        if self.b.is_some() {
            let ty = self.b.as_ref().unwrap().0.analyze(cxt)?;
            if ty != Type::Int {
                return Err(SemanticErr::MismatchType);
            }
        }
        Ok(Type::Int)
    }
}

impl SemanticAnalyzer for ExprNd {
    fn analyze(&self, cxt: &mut Context) -> Result<Type, SemanticErr> {
        let ty = self.a.analyze(cxt)?;
        if ty != Type::Int {
            return Err(SemanticErr::MismatchType);
        }
        if self.b.is_some() {
            let ty = self.b.as_ref().unwrap().0.analyze(cxt)?;
            if ty != Type::Int {
                return Err(SemanticErr::MismatchType);
            }
        }
        Ok(Type::Int)
    }
}

impl SemanticAnalyzer for VarNd {
    fn analyze(&self, cxt: &mut Context) -> Result<Type, SemanticErr> {
        if self.declared() {
            let vcxt = cxt.declare(self)?;
            *self.id.borrow_mut() = vcxt.id;
            Ok(Type::Void)
        } else {
            let vc = cxt.fetch(self)?;
            Ok(vc.ty)
        }
    }
}

impl SemanticAnalyzer for StmtNd {
    fn analyze(&self, cxt: &mut Context) -> Result<Type, SemanticErr> {
        let mut ty = Type::Void;
        if self.expr.is_some() {
            ty = self.expr.as_ref().unwrap().analyze(cxt)?;
        }
        if self.var.is_some() {
            let vn = self.var.as_ref().unwrap();
            let final_ty = vn.analyze(cxt)?;
            let declare_vn = cxt.fetch(vn)?;
            if ty != declare_vn.ty {
                return Err(SemanticErr::MismatchType);
            }
            ty = final_ty;
        }
        Ok(ty)
    }
}

impl SemanticAnalyzer for RootNd {
    fn analyze(&self, cxt: &mut Context) -> Result<Type, SemanticErr> {
        for stmt in self.stmts.iter() {
            stmt.analyze(cxt)?;
        }
        Ok(Type::Void)
    }
}
