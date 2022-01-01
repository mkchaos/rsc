use super::node::*;
use crate::token::{SeqSlice, Token};

pub enum ParseErr {
    // Bad(Token),
    NoToken,
}

pub struct Seq<T>(pub SeqSlice, pub T);

pub trait Parser: Sized {
    fn parse(seq: &SeqSlice) -> Result<Seq<Self>, ParseErr>;
}

impl Parser for FactorNd {
    fn parse(seq: &SeqSlice) -> Result<Seq<Self>, ParseErr> {
        if let Some(&Token::Value(v)) = seq.get(0) {
            return Ok(Seq(seq.advance(1), FactorNd::Value(v)));
        } else {
            return Err(ParseErr::NoToken);
        }
    }
}

impl Parser for TermNd {
    fn parse(seq: &SeqSlice) -> Result<Seq<Self>, ParseErr> {
        let Seq(s, a) = FactorNd::parse(seq)?;
        let t = s.get(0);
        match t {
            Some(&Token::Multiply) | Some(&Token::Divide) | Some(&Token::Modulo) => {
                let Seq(s, b) = TermNd::parse(&s.advance(1))?;
                Ok(Seq(
                    s,
                    TermNd {
                        a: Box::new(a),
                        b: Some((Box::new(b), t.unwrap().clone())),
                    },
                ))
            }
            _ => Ok(Seq(
                s,
                TermNd {
                    a: Box::new(a),
                    b: None,
                },
            )),
        }
    }
}

impl Parser for ExprNd {
    fn parse(seq: &SeqSlice) -> Result<Seq<Self>, ParseErr> {
        let Seq(s, a) = TermNd::parse(seq)?;
        let t = s.get(0);
        match t {
            Some(&Token::Add) | Some(&Token::Minus) => {
                let Seq(s, b) = ExprNd::parse(&s.advance(1))?;
                Ok(Seq(
                    s,
                    ExprNd {
                        a: Box::new(a),
                        b: Some((Box::new(b), t.unwrap().clone())),
                    },
                ))
            }
            _ => Ok(Seq(
                s,
                ExprNd {
                    a: Box::new(a),
                    b: None,
                },
            )),
        }
    }
}
