mod imp;

use super::types::{SeqPack, Sequence};
pub use imp::*;

pub trait Parser: Sized {
    fn parse(seq: Sequence) -> SeqPack<Self>;
}

pub fn parse<T: Parser>(code: &str) -> Result<T, String> {
    let lex_res = super::lexer::lexer(code);
    if let Err(e) = lex_res {
        return Err(format!("Lex error: {:?}", e));
    }
    let seq = Sequence::new(lex_res.unwrap());
    let parse_res = T::parse(seq);
    if parse_res.is_none() {
        Err("Parse error".to_owned())
    } else {
        let (seq, t) = parse_res.unwrap();
        if !seq.empty() {
            Err(format!("Not eat all seq, remains {:?}", seq.len()))
        } else {
            Ok(t)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::parse;
    use super::super::types::nodes::*;

    #[test]
    fn test_parse_func_call() {
        let code = "f(1,2,3)";
        let parse_res = parse::<FuncCallNd>(code);
        parse_res.unwrap();
    }

    #[test]
    fn test_parse_func() {
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
    fn test_parse_expr() {
        let code = "1+f(12*2,42)*2";
        let parse_res = parse::<ExprNd>(code);
        parse_res.unwrap();
        let code = "(-1+2)*(3-4)&&!1||6+10";
        let parse_res = parse::<ExprNd>(code);
        parse_res.unwrap();
    }

    #[test]
    fn test_parse_block() {
        let code = "{}";
        let parse_res = parse::<BlockNd>(code);
        parse_res.unwrap();
        let code = "{a=1;b=1;}";
        let parse_res = parse::<BlockNd>(code);
        parse_res.unwrap();
    }

    #[test]
    fn test_parse_if_else() {
        let code = "if(1){}";
        let parse_res = parse::<IfNd>(code);
        parse_res.unwrap();
        let code = "if(1){{}} else {}";
        let parse_res = parse::<IfNd>(code);
        parse_res.unwrap();
        let code = "if(1){{}} else if(2) {}";
        let parse_res = parse::<IfNd>(code);
        parse_res.unwrap();
    }

    #[test]
    fn test_parse_while() {
        let code = "while(1){}";
        let parse_res = parse::<WhileNd>(code);
        parse_res.unwrap();
    }

    #[test]
    fn test_parse_cond() {
        let code = "continue;";
        let parse_res = parse::<ContinueNd>(code);
        parse_res.unwrap();
        let code = "break;";
        let parse_res = parse::<BreakNd>(code);
        parse_res.unwrap();
        let code = "return;";
        let parse_res = parse::<ReturnNd>(code);
        parse_res.unwrap();
        let code = "return 0;";
        let parse_res = parse::<ReturnNd>(code);
        parse_res.unwrap();
        let code = "if (1) return 0;";
        let parse_res = parse::<IfNd>(code);
        parse_res.unwrap();
    }

    #[test]
    fn test_parse_multiple() {
        let code = "if(1)if(1){}else;else while(1);";
        let parse_res = parse::<IfNd>(code);
        parse_res.unwrap();
    }
}
