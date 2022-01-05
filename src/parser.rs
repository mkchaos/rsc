use crate::core::*;

pub trait Parser: Sized {
    fn parse(seq: Sequence) -> SeqPack<Self>;
}

impl Parser for FactorNd {
    fn parse(seq: Sequence) -> SeqPack<Self> {
        if let Some((seq, n)) = seq.clone().eat_value() {
            return Some((seq, FactorNd::Value(n)));
        }
        if let Some((seq, n)) = FuncCallNd::parse(seq.clone()) {
            return Some((seq, FactorNd::Func(n)));
        }
        if let Some((seq, n)) = VarNd::parse(seq.clone()) {
            return Some((seq, FactorNd::Var(n)));
        }
        None
    }
}

impl Parser for ExprNd {
    fn parse(seq: Sequence) -> SeqPack<Self> {
        let (seq, st) = get_calc_stack(seq)?;
        Some((seq, ExprNd::new(st)))
    }
}

impl Parser for VarNd {
    fn parse(seq: Sequence) -> SeqPack<Self> {
        let (seq, name) = seq.eat_name()?;
        Some((seq, VarNd::new(name)))
    }
}

impl Parser for AssignNd {
    fn parse(seq: Sequence) -> SeqPack<Self> {
        let (seq, v) = VarNd::parse(seq)?;
        let (seq, _) = seq.eat(Token::Eq)?;
        let (seq, ex) = ExprNd::parse(seq)?;
        Some((seq, AssignNd::new(v, ex)))
    }
}

impl Parser for DeclareNd {
    fn parse(seq: Sequence) -> SeqPack<Self> {
        let (seq, ty) = seq.eat_type()?;
        let (seq, v) = VarNd::parse(seq)?;
        let sp = seq.clone().eat(Token::Eq);
        if sp.is_none() {
            Some((seq, DeclareNd::new(ty, v, None)))
        } else {
            let (seq, _) = sp?;
            let (seq, ex) = ExprNd::parse(seq)?;
            Some((seq, DeclareNd::new(ty, v, Some(ex))))
        }
    }
}

impl Parser for StmtNd {
    fn parse(seq: Sequence) -> SeqPack<Self> {
        if let Some((seq, n)) = DeclareNd::parse(seq.clone()) {
            let (seq, _) = seq.eat(Token::Semicolon)?;
            return Some((seq, StmtNd::Declare(n)));
        }
        if let Some((seq, n)) = AssignNd::parse(seq.clone()) {
            let (seq, _) = seq.eat(Token::Semicolon)?;
            return Some((seq, StmtNd::Assign(n)));
        }
        if let Some((seq, n)) = ExprNd::parse(seq.clone()) {
            let (seq, _) = seq.eat(Token::Semicolon)?;
            return Some((seq, StmtNd::Expr(n)));
        }
        if let Some((seq, n)) = VarNd::parse(seq.clone()) {
            let (seq, _) = seq.eat(Token::Semicolon)?;
            return Some((seq, StmtNd::Print(n)));
        }
        let (seq, _) = seq.eat(Token::Semicolon)?;
        Some((seq, StmtNd::Empty))
    }
}

impl Parser for ItemNd {
    fn parse(seq: Sequence) -> SeqPack<Self> {
        if let Some((seq, n)) = BlockNd::parse(seq.clone()) {
            return Some((seq, ItemNd::Block(n)));
        }
        if let Some((seq, n)) = StmtNd::parse(seq.clone()) {
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
        Some((mseq.advance(1), BlockNd::new(items)))
    }
}

impl Parser for FuncNd {
    fn parse(seq: Sequence) -> SeqPack<Self> {
        let (seq, ret_ty) = seq.eat_type()?;
        let (seq, func_var) = VarNd::parse(seq)?;
        let mut params = Vec::new();
        let mut first_param = true;
        let (mut mseq, _) = seq.eat(Token::LParen)?;
        while mseq.get(0) != Some(Token::RParen) {
            if !first_param {
                let (seq, _) = mseq.eat(Token::Comma)?;
                mseq = seq;
            }
            first_param = false;
            let (seq, ty) = mseq.eat_type()?;
            if let Some((seq, vn)) = VarNd::parse(seq.clone()) {
                params.push((ty, Some(vn)));
                mseq = seq;
            } else {
                params.push((ty, None));
                mseq = seq;
            }
        }
        mseq = mseq.advance(1);
        if let Some((seq, block)) = BlockNd::parse(mseq.clone()) {
            mseq = seq;
            let nd = FuncNd::new(ret_ty, func_var, params, Some(block));
            if nd.check() {
                Some((mseq, nd))
            } else {
                None
            }
        } else {
            let (mseq, _) = mseq.eat(Token::Semicolon)?;
            let nd = FuncNd::new(ret_ty, func_var, params, None);
            Some((mseq, nd))
        }
    }
}

impl Parser for FuncCallNd {
    fn parse(seq: Sequence) -> SeqPack<Self> {
        let (seq, func_var) = VarNd::parse(seq)?;
        let mut params = Vec::new();
        let mut first_param = true;
        let (mut mseq, _) = seq.eat(Token::LParen)?;
        while mseq.get(0) != Some(Token::RParen) {
            if !first_param {
                let (seq, _) = mseq.eat(Token::Comma)?;
                mseq = seq;
            }
            first_param = false;
            let (seq, ex) = ExprNd::parse(mseq)?;
            params.push(ex);
            mseq = seq;
        }
        Some((mseq.advance(1), FuncCallNd::new(func_var, params)))
    }
}

impl Parser for GItemNd {
    fn parse(seq: Sequence) -> SeqPack<Self> {
        if let Some((seq, n)) = DeclareNd::parse(seq.clone()) {
            return Some((seq, GItemNd::Declare(n)));
        }
        if let Some((seq, n)) = FuncNd::parse(seq.clone()) {
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
            let (seq, n) = GItemNd::parse(mseq)?;
            items.push(n);
            mseq = seq;
        }
        Some((mseq.advance(1), RootNd::new(items)))
    }
}
