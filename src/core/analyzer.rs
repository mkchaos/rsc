mod context;
mod imp;

use super::parser::{parse, Parser};
use super::types::ErrKind;
use context::Context;
use imp::*;

pub trait Analyzer {
    fn analyze(&self, cxt: &mut Context) -> Result<(), ErrKind>;
}

pub fn analyze<T: Parser + Analyzer>(code: &str) -> Result<(), ErrKind> {
    let nd_res = parse::<T>(code);
    if nd_res.is_err() {
        return Err(ErrKind::ParseErr);
    }
    let nd = nd_res.unwrap();
    let mut cxt = Context::new();
    nd.analyze(&mut cxt)
}

#[cfg(test)]
mod tests {
    use super::parse;
    use super::super::types::nodes::*;

    #[test]
    fn test_analyze_func_call() {
        let code = "f(1,2,3)";
        let parse_res = parse::<FuncCallNd>(code);
        parse_res.unwrap();
    }

    #[test]
    fn test_analyze_func() {
        let code = "int foo(int a);";
        let parse_res = parse::<FuncNd>(code);
        parse_res.unwrap();
        let code = "int foo(int a, int);";
        let parse_res = parse::<FuncNd>(code);
        parse_res.unwrap();
        let code = "int foo(){}";
        let parse_res = parse::<FuncNd>(code);
        parse_res.unwrap();
    }

    #[test]
    fn test_analyze_expr() {
        let code = "1+f(12*2,42)*2";
        let parse_res = parse::<ExprNd>(code);
        parse_res.unwrap();
        let code = "(-1+2)*(3-4)&&!1||6+10";
        let parse_res = parse::<ExprNd>(code);
        parse_res.unwrap();
    }

    #[test]
    fn test_analyze_block() {
        let code = "{}";
        let parse_res = parse::<BlockNd>(code);
        parse_res.unwrap();
        let code = "{a=1;b=1;}";
        let parse_res = parse::<BlockNd>(code);
        parse_res.unwrap();
    }
}
