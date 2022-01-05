#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct VarContext {
    pub id: u32,
    pub scope_id: u32,
    pub offset: usize,
    pub size: usize,
}

impl VarContext {
    pub fn new(id: u32, sid: u32, offset: usize, size: usize) -> Self {
        VarContext {
            id: id,
            scope_id: sid,
            offset: offset,
            size: size,
        }
    }
}
