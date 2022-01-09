use super::token::Type;

#[derive(Debug, PartialEq, Clone)]
pub struct FuncInfo {
    pub id: u32,
    pub ty: Type,
    pub has_impl: bool,
}

impl FuncInfo {
    pub fn new(id: u32, ty: Type) -> Self {
        FuncInfo {
            id: id,
            ty: ty,
            has_impl: false,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct VarInfo {
    pub id: u32,
    pub scope_id: u32,
    pub func_id: u32,
    pub ty: Type,
}

impl VarInfo {
    pub fn is_global(&self) -> bool {
        self.func_id == 0
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ScopeInfo {
    pub id: u32,
    pub is_loop: bool,
    pub func_id: u32,
}

impl ScopeInfo {
    pub fn is_global(&self) -> bool {
        self.func_id == 0
    }
}
