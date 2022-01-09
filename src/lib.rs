mod core;
mod utils;

use crate::core::{compile, ErrKind, VM};
use crate::utils::load_code_from_file;

pub fn compile_and_run(path: &str) -> Result<Vec<i32>, ErrKind> {
    let code = load_code_from_file(path);
    let prog = compile(&code)?;
    let mut vm = VM::new(1000, prog);
    vm.execute()
}

#[cfg(test)]
mod tests {
    use super::compile_and_run;

    #[test]
    fn gcd_example() {
        assert_eq!(vec![9], compile_and_run("example/gcd.c").unwrap());
    }
}
