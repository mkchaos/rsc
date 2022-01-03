mod token;
mod node;
mod vm;

#[cfg(test)]
mod tests {
    use crate::node::node::{RootNd};
    use crate::node::parser::*;
    use crate::token::{lexer, Sequence};
    use crate::node::semantic_analyzer::*;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_pipeline() {
        let code = "int a=1;int b=1;int c=a+b+1;";
        if let Ok(s) = lexer(code) {
            let seq = Sequence::new(s);
            if let Some((_, n)) = RootNd::parse(seq) {
                let mut cxt = Context::new();
                // println!("{}", n.stmts.len())
                match n.analyze(&mut cxt) {
                    Ok(_) =>{
                        println!("ok");
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
