mod core;
mod utils;

use crate::utils::load_code_from_file;
use crate::core::{compile, VM, ErrKind};

pub fn compile_and_run(path: &str) -> Result<(), ErrKind> {
    let code = load_code_from_file(path);
    let prog = compile(&code)?;
    let mut vm = VM::new(1000, prog);
    vm.execute()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    #[should_panic(expected = "xxx")]
    fn it_panics() {
        // let result = 2 + 2;
        assert_eq!(1, 2, "xxx");
    }
}
