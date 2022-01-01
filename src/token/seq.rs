use super::token::Token;
use std::rc::Rc;

pub struct Seq {
    tokens: Vec<Token>,
}

impl Seq {
    pub fn new() -> Self {
        Seq { tokens: Vec::new() }
    }

    pub fn add(&mut self, t: Token) {
        self.tokens.push(t);
    }

    pub fn get(&self, idx: usize) -> Option<&Token> {
        self.tokens.get(idx)
    }
}

#[derive(Clone)]
pub struct SeqSlice {
    seq: Rc<Seq>,
    cur: usize,
}

impl SeqSlice {
    pub fn new(seq: Seq) -> Self {
        SeqSlice {
            seq: Rc::new(seq),
            cur: 0,
        }
    }

    #[allow(dead_code)]
    pub fn get(&self, idx: usize) -> Option<&Token> {
        self.seq.get(idx + self.cur)
    }

    #[allow(dead_code)]
    pub fn advance(&self, off: usize) -> Self {
        let mut other = self.clone();
        other.cur += off;
        other
    }
}
