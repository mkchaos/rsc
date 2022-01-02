mod token;
mod node;

#[cfg(test)]
mod tests {
    use crate::node::node::{RootNd};
    use crate::node::parser::*;
    use crate::token::{lexer, Sequence};
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_pipeline() {
        let code = "int a=1;b=1;";
        if let Ok(s) = lexer(code) {
            let seq = Sequence::new(s);
            if let Some((_, n)) = RootNd::parse(seq) {
                println!("{}", n.stmts.len())
            }
        }
    }
}
