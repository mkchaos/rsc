mod compiler;
mod core;
mod lexer;
mod parser;
mod semantic_analyzer;
mod vm;

#[cfg(test)]
mod tests {
    use std::fs;
    use super::lexer::lexer;
    use super::parser::Parser;
    use crate::core::*;

    fn load_code_from_file(path: &str) -> String {
        fs::read_to_string(path).expect("No file")
    }

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_parse_expr() {
        let code = "1+f(12*2,42)*2";
        let lex_res = lexer(code);
        assert!(lex_res.is_ok(), "Lex error");
        let seq = Sequence::new(lex_res.unwrap());
        println!("seq {:?}", seq);
        let parse_res = ExprNd::parse(seq);
        assert!(parse_res.is_some(), "parse error");
        println!("{:?}", parse_res.unwrap());
    }


    #[test]
    fn test_parse_fn() {
        let code = "int f(){int a = 1;};";
        let lex_res = lexer(code);
        assert!(lex_res.is_ok(), "Lex error");
        let seq = Sequence::new(lex_res.unwrap());
        println!("seq {:?}", seq);
        let parse_res = FuncNd::parse(seq);
        assert!(parse_res.is_some(), "parse error");
        // println!("{:?}", parse_res.unwrap());
        println!("ok");
    }

    #[test]
    fn test_parser() {
        let code = &load_code_from_file("example/1.c");
        let lex_res = lexer(code);
        assert!(lex_res.is_ok(), "Lex error");
        let seq = Sequence::new(lex_res.unwrap());
        println!("seq {:?}", seq);
        let parse_res = RootNd::parse(seq);
        assert!(parse_res.is_some(), "parse error");
        // println!("{:?}", parse_res.unwrap());
        println!("ok");
    }
}
