mod imp;
mod prog;

use super::analyzer::{analyze, Analyzer};
use crate::core::types::ErrKind;
use prog::Context;
pub use prog::Program;

pub trait Compiler: Analyzer {
    fn compile(&self, prog: &mut Context);
}

pub fn compile(code: &str) -> Result<Program, ErrKind> {
    let (nd, info) = analyze(code)?;
    let mut cxt = Context::new(info);
    nd.compile(&mut cxt);
    Ok(Program::new(cxt))
}
