use super::token::{Token, Type, Value};
use std::rc::Rc;

#[derive(Clone)]
pub struct Sequence {
    tokens: Rc<Vec<Token>>,
    cur: usize,
}

pub type SeqPack<T> = Option<(Sequence, T)>;

#[allow(dead_code)]
impl Sequence {
    pub fn new(tks: Vec<Token>) -> Self {
        Sequence {
            tokens: Rc::new(tks),
            cur: 0,
        }
    }

    pub fn eat(&self, t: Token) -> SeqPack<Token> {
        if self.get(0) == Some(t.clone()) {
            Some((self.advance(1), t))
        } else {
            None
        }
    }

    pub fn eat_value(&self) -> SeqPack<Value> {
        if let Some(Token::Value(v)) = self.get(0) {
            Some((self.advance(1), v))
        } else {
            None
        }
    }

    pub fn eat_type(&self) -> SeqPack<Type> {
        if let Some(Token::Type(v)) = self.get(0) {
            Some((self.advance(1), v))
        } else {
            None
        }
    }

    pub fn eat_name(&self) -> SeqPack<String> {
        if let Some(Token::Name(s)) = self.get(0) {
            Some((self.advance(1), s))
        } else {
            None
        }
    }

    pub fn swtich_eat(&self, tks: &[Token]) -> SeqPack<Token> {
        for t in tks.iter() {
            if self.get(0) == Some(t.clone()) {
                return Some((self.clone(), t.clone()));
            }
        }
        None
    }

    pub fn eat_fn<T: Sized, F: Fn(Sequence) -> SeqPack<T>>(
        &self,
        heads: &[Token],
        f: F,
        tails: &[Token],
    ) -> SeqPack<T> {
        let mut seq = self.clone();
        for it in heads.iter() {
            let (s, _) = seq.eat(it.clone())?;
            seq = s;
        }
        let (mut seq, t) = f(seq)?;
        for it in tails.iter() {
            let (s, _) = seq.eat(it.clone())?;
            seq = s;
        }
        Some((seq, t))
    }

    pub fn get(&self, idx: usize) -> Option<Token> {
        if idx >= self.len() {
            None
        } else {
            Some(self.tokens[idx + self.cur].clone())
        }
    }

    pub fn advance(&self, off: usize) -> Self {
        Sequence {
            tokens: self.tokens.clone(),
            cur: self.cur + off,
        }
    }

    pub fn len(&self) -> usize {
        self.tokens.len() - self.cur
    }

    pub fn empty(&self) -> bool {
        self.len() == 0
    }
}

// pub struct Seq {
//     tokens: Vec<Token>,
// }

// impl Seq {
//     pub fn new() -> Self {
//         Seq { tokens: Vec::new() }
//     }

//     pub fn add(&mut self, t: Token) {
//         self.tokens.push(t);
//     }

//     pub fn get(&self, idx: usize) -> Option<Token> {
//         let t = self.tokens.get(idx)?;
//         Some(t.clone())
//     }
// }

// #[derive(Clone)]
// pub struct SeqSlice {
//     seq: Rc<Seq>,
//     cur: usize,
// }

// impl SeqSlice {
//     pub fn new(seq: Seq) -> Self {
//         SeqSlice {
//             seq: Rc::new(seq),
//             cur: 0,
//         }
//     }

//     #[allow(dead_code)]
//     pub fn get(&self, idx: usize) -> Option<Token> {
//         self.seq.get(idx + self.cur)
//     }

//     #[allow(dead_code)]
//     pub fn advance(&self, off: usize) -> Self {
//         let mut other = self.clone();
//         other.cur += off;
//         other
//     }
// }
