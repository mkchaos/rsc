mod context;
mod imp;

use super::parser::{parse, Parser};
use super::types::{ErrKind, RootNd, Type};
use context::Context;
pub use context::Semantic;

pub trait Analyzer: Parser {
    fn analyze(&self, cxt: &mut Context) -> Result<Type, ErrKind>;
}

pub fn analyze(code: &str) -> Result<(RootNd, Semantic), ErrKind> {
    let nd_res = parse::<RootNd>(code);
    if nd_res.is_err() {
        return Err(ErrKind::ParseErr);
    }
    let nd = nd_res.unwrap();
    let mut cxt = Context::new();
    nd.analyze(&mut cxt)?;
    Ok((nd, Semantic::new(cxt)?))
}

#[cfg(test)]
mod tests {
    use super::analyze;
    use crate::utils::load_code_from_file;

    #[test]
    fn test_analyze() {
        let code = load_code_from_file("example/test_a_1.c");
        let res = analyze(&code);
        // assert!(res.is_ok());
        if res.is_err() {
            println!("{:?}", res.err());
        } else {
            println!("Big ok");
        }
    }
}
