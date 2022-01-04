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
        let mut seq = seq;
        let sp = VarNd::parse(seq.clone());
        let mut var = None;
        let mut expr = None;
        if sp.is_some() {
            let (s, n) = sp?;
            var = Some(n);
            match s.get(0) {
                Some(Token::Assign) => {
                    seq = s.advance(1);
                }
                _ => return Some((s, StmtNd::new(var, None))),
            }
        }
        let sp = ExprNd::parse(seq.clone());
        if sp.is_some() {
            let (s, n) = sp?;
            seq = s;
            expr = Some(n);
        }
        Some((seq, StmtNd::new(var, expr)))
    }
}

impl Parser for ItemNd {
    fn parse(seq: Sequence) -> SeqPack<Self> {
        let sp = BlockNd::parse(seq.clone());
        if sp.is_some() {
            let (seq, n) = sp?;
            return Some((seq, ItemNd::Block(n)));
        }
        let sp = StmtNd::parse(seq);
        if sp.is_some() {
            let (seq, n) = sp?;
            let (seq, _) = seq.eat(Token::Semicolon)?;
            return Some((seq, ItemNd::Stmt(n)));
        }
        None
    }
}

impl Parser for BlockNd {
    fn parse(seq: Sequence) -> SeqPack<Self> {
        let mut items = Vec::new();
        let (mut mseq, _) = seq.eat(Token::LBrace)?;
        while mseq.get(0) != Some(Token::RBrace) {
            let (seq, n) = ItemNd::parse(mseq)?;
            items.push(n);
            mseq = seq;
        }
        Some((mseq.advance(1), BlockNd { items: items }))
    }
}

impl Parser for FuncHeadNd {
    fn parse(seq: Sequence) -> SeqPack<Self> {
        let (seq, vn) = VarNd::parse(seq)?;
        let (seq, _) = seq.eats(&vec![Token::LParen, Token::RParen])?;
        Some((seq, FuncHeadNd { name: Box::new(vn) }))
    }
}

impl Parser for FuncNd {
    fn parse(seq: Sequence) -> SeqPack<Self> {
        let (seq, head) = FuncHeadNd::parse(seq)?;
        let (seq, block) = BlockNd::parse(seq)?;
        Some((
            seq,
            FuncNd {
                head: Box::new(head),
                block: Box::new(block),
            },
        ))
    }
}

impl Parser for GItemNd {
    fn parse(seq: Sequence) -> SeqPack<Self> {
        let sp = StmtNd::parse(seq.clone());
        if sp.is_some() {
            let (seq, n) = sp?;
            let (seq, _) = seq.eat(Token::Semicolon)?;
            if n.declared() {
                return None;
            }
            return Some((seq, GItemNd::Stmt(n)));
        }
        let sp = FuncNd::parse(seq);
        if sp.is_some() {
            let (seq, n) = sp?;
            return Some((seq, GItemNd::Func(n)));
        }
        None
    }
}

impl Parser for RootNd {
    fn parse(seq: Sequence) -> SeqPack<Self> {
        let mut items = Vec::new();
        let mut mseq = seq;
        while !mseq.empty() {
            let (seq, n) = ItemNd::parse(mseq)?;
            items.push(n);
            mseq = seq;
        }
        Some((mseq.advance(1), RootNd { items: items }))
    }
}
