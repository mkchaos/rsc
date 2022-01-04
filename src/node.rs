mod doter;
mod node;
mod parser;
mod semantic_analyzer;
mod context;

use crate::token::Sequence;
pub use node::*;
pub use parser::Parser;
pub use semantic_analyzer::SemanticAnalyzer;
pub use {context::Context, context::VarContext};
pub use context::SemanticErr;

pub fn parse<T: Parser>(seq: Sequence) -> Option<T> {
    let (seq, t) = T::parse(seq)?;
    if !seq.empty() {
        println!("not eat all seq; seq len: {}", seq.len());
        None
    } else {
        Some(t)
    }
}

pub fn analyze<T: SemanticAnalyzer>(nd: &T) -> Result<Context, SemanticErr> {
    let mut cxt = Context::new();
    nd.analyze(&mut cxt)?;
    Ok(cxt)
}
