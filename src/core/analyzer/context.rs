use crate::core::types::{ErrKind, FuncInfo, Layout, ScopeInfo, Type, VarInfo, get_type_size};
use std::collections::HashMap;

pub struct Context {
    names: HashMap<String, Vec<(u32, u32)>>, // (scope_id, id)
    mem_layout: Vec<Layout>,
    vars: HashMap<u32, VarInfo>,
    scopes: HashMap<u32, ScopeInfo>,
    funcs: HashMap<u32, FuncInfo>,
    scope_stack: Vec<u32>,
    cur_offset: usize,
    cur_func_id: u32,
}

impl Context {
    pub fn new() -> Self {
        let mut cxt = Context {
            names: HashMap::new(),
            mem_layout: Vec::new(),
            vars: HashMap::new(),
            scopes: HashMap::new(),
            funcs: HashMap::new(),
            scope_stack: Vec::new(),
            cur_offset: 0,
            cur_func_id: 0,
        };
        let id = cxt.new_mem_layout(); // into program
        cxt.scope_stack.push(id);
        cxt
    }

    fn get_current_scope_id(&self) -> u32 {
        *self.scope_stack.last().unwrap()
    }

    fn new_mem_layout(&mut self) -> u32 {
        let id = self.mem_layout.len() as u32;
        self.mem_layout.push(Layout::new(self.cur_offset));
        id
    }

    pub fn enter_scope(&mut self) -> u32 {
        let id = self.new_mem_layout();
        self.scopes.insert(id, ScopeInfo { id: id });
        self.scope_stack.push(id);
        id
    }

    pub fn exit_scope(&mut self) {
        let idx = self.scope_stack.pop().unwrap() as usize;
        self.mem_layout[idx].end(self.cur_offset);
        if self.get_current_scope_id() == 0 {
            self.cur_func_id = 0;
        }
    }

    fn declare(&mut self, name: &str, sz: usize) -> Result<u32, ErrKind> {
        let scope_id = self.get_current_scope_id();
        if self.names.contains_key(name) {
            let ids = self.names.get(name).unwrap();
            if ids.iter().any(|(sid, _)| *sid == scope_id) {
                Err(ErrKind::ReDeclare)
            } else {
                let id = self.new_mem_layout();
                self.cur_offset += sz;
                self.mem_layout.last_mut().unwrap().end(self.cur_offset);
                self.names.get_mut(name).unwrap().push((scope_id, id));
                Ok(id)
            }
        } else {
            let id = self.new_mem_layout();
            self.cur_offset += sz;
            self.mem_layout.last_mut().unwrap().end(self.cur_offset);
            self.names.insert(name.to_owned(), vec![(scope_id, id)]);
            Ok(id)
        }
    }

    pub fn get_type_by_id(&self, id: u32) -> Result<Type, ErrKind> {
        match self.funcs.get(&id) {
            Some(f) => Ok(f.ty.clone()),
            None => match self.vars.get(&id) {
                Some(v) => Ok(v.ty.clone()),
                None => Err(ErrKind::NoDeclare),
            },
        }
    }

    pub fn fetch(&self, name: &str) -> Result<u32, ErrKind> {
        if self.names.contains_key(name) {
            let ids = self.names.get(name).unwrap();
            for id in self.scope_stack.iter().rev() {
                for (sid, vid) in ids.iter() {
                    if *sid == *id {
                        return Ok(*vid);
                    }
                }
            }
            Err(ErrKind::NoDeclare)
        } else {
            Err(ErrKind::NoDeclare)
        }
    }

    pub fn declare_var(&mut self, name: &str, ty: &Type) -> Result<u32, ErrKind> {
        let id = self.declare(name, get_type_size(ty.clone()))?;
        self.vars.insert(
            id,
            VarInfo {
                id: id,
                scope_id: self.get_current_scope_id(),
                func_id: self.cur_func_id,
                ty: ty.clone(),
            },
        );
        Ok(id)
    }

    pub fn declare_fn(&mut self, name: &str, ty: &Type) -> Result<u32, ErrKind> {
        let id = self.declare(name, 0)?;
        self.cur_func_id = id;
        self.funcs.insert(id, FuncInfo::new(id, ty.clone()));
        Ok(id)
    }

    pub fn impl_fn(&mut self, name: &str, ty: &Type) -> Result<u32, ErrKind> {
        match self.fetch(name) {
            Ok(id) => {
                let finfo = self.funcs.get_mut(&id).unwrap();
                if finfo.has_impl {
                    Err(ErrKind::ReImpl)
                } else if finfo.ty != ty.clone() {
                    Err(ErrKind::TypeErr)
                } else {
                    finfo.has_impl = true;
                    Ok(id)
                }
            }
            Err(_) => {
                let id = self.declare_fn(name, ty)?;
                self.funcs.get_mut(&id).unwrap().has_impl = true;
                Ok(id)
            }
        }
    }
}

pub struct Semantic {
    pub mem_layout: Vec<Layout>,
    pub vars: HashMap<u32, VarInfo>,
    pub funcs: HashMap<u32, FuncInfo>,
    pub main_func_id: u32,
}

impl Semantic {
    pub fn new(cxt: Context) -> Result<Self, ErrKind> {
        let main_id = match cxt.fetch("main") {
            Ok(id) => id,
            Err(_) => return Err(ErrKind::NoMainFunc),
        };
        let ty = cxt.get_type_by_id(main_id)?;
        if ty != Type::Func(vec![Type::Int]) {
            // main type error
            return Err(ErrKind::TypeErr);
        }
        for (_, info) in cxt.funcs.iter() {
            if !info.has_impl {
                // no impl func
                return Err(ErrKind::FuncNoImpl);
            }
        }
        let mut mem_layout = cxt.mem_layout;
        for (_, v) in cxt.vars.iter() {
            if !v.is_global() {
                let off = mem_layout[v.func_id as usize].offset;
                mem_layout[v.id as usize].offset -= off;
            }
        }
        for l in mem_layout.iter() {
            println!("layout: {:?}", l.clone());
        }
        Ok(Semantic {
            mem_layout: mem_layout,
            vars: cxt.vars,
            funcs: cxt.funcs,
            main_func_id: main_id,
        })
    }
}
