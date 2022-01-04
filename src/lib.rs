mod node;
mod token;
mod vm;

use node::*;
use std::fs;
use token::*;
use vm::*;

pub fn load_code(path: &str) -> String {
    fs::read_to_string(path).expect("Something went wrong reading the file")
}

pub fn pipeline<T: Parser + SemanticAnalyzer + Compiler>(code: &str) {
    match lexer(code) {
        Ok(s) => {
            let seq = Sequence::new(s);
            println!("{:?}", seq);
            let nd = parse::<T>(seq);
            if nd.is_none() {
                println!("parse error");
                return;
            }
            let nd = nd.unwrap();
            match analyze::<T>(&nd) {
                Ok(cxt) => {
                    for (_, v) in cxt.freeze() {
                        println!("{:?}", v);
                    }
                    let mut prog = Program::new(&cxt);
                    nd.compile(&mut prog);
                    let mut vm = VM::new(100, prog);
                    vm.execute();
                }
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_pipeline() {
        let code = "int a=1;int b=2;int c=a+b+3;a;b;c;";
        super::pipeline::<super::RootNd>(code);
    }

    #[test]
    fn test_func() {
        let code = super::load_code("example/1.c");
        super::pipeline::<super::RootNd>(&code);
    }
}
