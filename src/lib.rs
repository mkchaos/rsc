mod core;
mod utils;

use crate::core::{compile, Code, ErrKind, VM};
use crate::utils::load_code_from_file;

pub fn compile_and_run(path: &str) -> Result<Vec<i32>, ErrKind> {
    let code = load_code_from_file(path);
    let prog = compile(&code)?;
    let mut vm = VM::new(1000, prog);
    vm.execute()
}

pub fn compile_to_code(path: &str) -> Result<Vec<Code>, ErrKind> {
    let code = load_code_from_file(path);
    let prog = compile(&code)?;
    for c in prog.codes.iter() {
        println!("{:?}", c.clone());
    }
    Ok(prog.codes)
}

#[cfg(test)]
mod tests {
    use super::compile_and_run;

    #[test]
    fn gcd_example() {
        assert_eq!(vec![9], compile_and_run("example/gcd.c").unwrap());
    }
}
