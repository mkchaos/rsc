mod imp;
mod context;

use super::analyzer::{analyze, Analyzer};
use crate::core::types::ErrKind;
use context::Context;
pub use context::Program;

pub trait Compiler: Analyzer {
    fn compile(&self, prog: &mut Context);
}

pub fn compile(code: &str) -> Result<Program, ErrKind> {
    let (nd, info) = analyze(code)?;
    let mut cxt = Context::new(info);
    nd.compile(&mut cxt);
    Ok(Program::new(cxt))
}

#[cfg(test)]
mod tests {
    use crate::utils::load_code_from_file;

    #[test]
    fn test_compile() {
        let code = load_code_from_file("example/test_c_1.c");
        let prog = super::compile(&code);
        if prog.is_err() {
            println!("{:?}", prog.err());
        } else {
            let prog = prog.unwrap();
            for c in prog.codes {
                println!("{:?}", c);
            }
            println!("Big ok");
        }
    }
}
