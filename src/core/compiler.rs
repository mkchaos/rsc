mod imp;
mod prog;

use super::analyzer::Analyzer;
use crate::core::types::ErrKind;
use prog::Context;
pub use prog::Program;
// use crate::vm::ProgContext;

pub trait Compiler: Analyzer {
    fn compile(&self, prog: &mut Context);
}

// pub fn compile(&code: &str) -> Result<Program, ErrKind> {}
