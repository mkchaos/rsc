use crate::token::{Token, Value};

#[derive(Debug, Clone)]
pub enum FactorNd {
    Value(Value),
}

#[derive(Debug, Clone)]
pub struct TermNd {
    pub a: Box<FactorNd>,
    pub b: Option<(Box<TermNd>, Token)>,
}

#[derive(Debug, Clone)]
pub struct ExprNd {
    pub a: Box<TermNd>,
    pub b: Option<(Box<ExprNd>, Token)>,
}
