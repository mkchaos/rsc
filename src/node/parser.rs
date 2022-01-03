use super::node::*;
use crate::token::{SeqPack, Sequence, Token};

pub trait Parser: Sized {
    fn parse(seq: Sequence) -> SeqPack<Self>;
}

impl Parser for FactorNd {
    fn parse(seq: Sequence) -> SeqPack<Self> {
        let sp = seq.eat_value();
        if sp.is_some() {
            let (seq, n) = sp?;
            return Some((seq, FactorNd::Value(n)));
        }
        let sp = VarNd::parse(seq.clone());
        if sp.is_some() {
            let (seq, n) = sp?;
            if n.declared() {
                return None;
            } else {
                return Some((seq, FactorNd::Var(n)));
            }
        }
        let sp = seq.eat_fn(&vec![Token::LParen], ExprNd::parse, &vec![Token::RParen]);
        if sp.is_some() {
            let (seq, n) = sp?;
            return Some((seq, FactorNd::Expr(Box::new(n))));
        }
        None
    }
}

impl Parser for TermNd {
    fn parse(seq: Sequence) -> SeqPack<Self> {
        let (seq, a) = FactorNd::parse(seq)?;
        let sp = seq.swtich_eat(&vec![Token::Multiply, Token::Divide, Token::Modulo]);
        if sp.is_some() {
            let (seq, op) = sp?;
            let (seq, b) = TermNd::parse(seq)?;
            Some((
                seq,
                TermNd {
                    a: Box::new(a),
                    b: Some((Box::new(b), op)),
                },
            ))
        } else {
            Some((
                seq,
                TermNd {
                    a: Box::new(a),
                    b: None,
                },
            ))
        }
    }
}

impl Parser for ExprNd {
    fn parse(seq: Sequence) -> SeqPack<Self> {
        let (seq, a) = TermNd::parse(seq)?;
        let sp = seq.swtich_eat(&vec![Token::Add, Token::Minus]);
        if sp.is_some() {
            let (seq, op) = sp?;
            let (seq, b) = ExprNd::parse(seq)?;
            Some((
                seq,
                ExprNd {
                    a: Box::new(a),
                    b: Some((Box::new(b), op)),
                },
            ))
        } else {
            Some((
                seq,
                ExprNd {
                    a: Box::new(a),
                    b: None,
                },
            ))
        }
    }
}

impl Parser for VarNd {
    fn parse(seq: Sequence) -> SeqPack<Self> {
        let sp = seq.eat_type();
        if sp.is_some() {
            let (seq, ty) = sp?;
            let (seq, name) = seq.eat_name()?;
            return Some((seq, VarNd::new(Some(ty), name)));
        }
        let (seq, name) = seq.eat_name()?;
        Some((seq, VarNd::new(None, name)))
    }
}

impl Parser for StmtNd {
    fn parse(seq: Sequence) -> SeqPack<Self> {
        let sp = VarNd::parse(seq.clone());
        if sp.is_some() {
            let (mut seq, n) = sp?;
            let sp = seq
                .clone()
                .eat_fn(&vec![Token::Assign], ExprNd::parse, &vec![]);
            let mut expr = None;
            if sp.is_some() {
                let (s, n) = sp?;
                expr = Some(n);
                seq = s;
            }
            return Some((
                seq,
                StmtNd {
                    var: Some(n),
                    expr: expr,
                },
            ));
        }
        let sp = seq
            .clone()
            .eat_fn(&vec![Token::Assign], ExprNd::parse, &vec![]);
        if sp.is_some() {
            let (seq, n) = sp?;
            return Some((
                seq,
                StmtNd {
                    var: None,
                    expr: Some(n),
                },
            ));
        }
        Some((
            seq,
            StmtNd {
                var: None,
                expr: None,
            },
        ))
    }
}

impl Parser for RootNd {
    fn parse(seq: Sequence) -> SeqPack<Self> {
        let mut seq = seq;
        let mut stmts: Vec<StmtNd> = Vec::new();
        loop {
            let sp = seq
                .clone()
                .eat_fn(&vec![], StmtNd::parse, &vec![Token::Semicolon]);
            if sp.is_some() {
                let (s, n) = sp?;
                seq = s;
                stmts.push(n);
            } else {
                break;
            }
        }
        Some((seq, RootNd { stmts: stmts }))
    }
}
