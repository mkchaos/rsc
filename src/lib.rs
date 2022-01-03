mod token;
mod node;
mod vm;

#[cfg(test)]
mod tests {
    use crate::node::node::{RootNd};
    use crate::node::parser::*;
    use crate::token::{lexer, Sequence};
    use crate::node::semantic_analyzer::*;
    use crate::vm::compiler::{Compiler, Program};
    use crate::vm::vm::{VM};
    
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_pipeline() {
        let code = "int a=1;int b=2;int c=a+b+3;a;b;c;";
        if let Ok(s) = lexer(code) {
            let seq = Sequence::new(s);
            if let Some((_, n)) = RootNd::parse(seq) {
                let mut cxt = Context::new();
                // println!("{}", n.stmts.len())
                match n.analyze(&mut cxt) {
                    Ok(_) =>{ 
                        let mut prog = Program::new(&cxt);
                        n.compile(&mut prog);
                        println!("ok");
                        for ins in prog.inss.iter() {
                            println!("{:?}", ins);
                        }
                        let mut vm = VM::new(100, prog);
                        vm.execute();
                    }
                    Err(t) => {
                        println!("{:?}", t);
                    }
                }
                println!("{:?} {}", cxt.cur_var_id, cxt.mem_cur);
            }
        }
    }
}
