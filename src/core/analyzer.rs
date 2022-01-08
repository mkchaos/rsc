mod context;
mod imp;

use super::parser::{parse, Parser};
use super::types::{ErrKind, RootNd, Type};
use context::Context;
pub use context::SemanticInfo;

pub trait Analyzer: Parser {
    fn analyze(&self, cxt: &mut Context) -> Result<Type, ErrKind>;
}

pub fn analyze(code: &str) -> Result<(RootNd, SemanticInfo), ErrKind> {
    let nd_res = parse::<RootNd>(code);
    if nd_res.is_err() {
        return Err(ErrKind::ParseErr);
    }
    let nd = nd_res.unwrap();
    let mut cxt = Context::new();
    nd.analyze(&mut cxt)?;
    Ok((nd, SemanticInfo::new(cxt)))
}

#[cfg(test)]
mod tests {
    use super::analyze;
    use crate::utils::load_code_from_file;

    #[test]
    fn test_analyze() {
        let code = load_code_from_file("example/1.c");
        let res = analyze(&code);
        if res.is_err() {
            println!("{:?}", res.err());
        } else {
            println!("Big ok");
        }
    }
}
