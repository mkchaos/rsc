use super::token::Token;

pub struct Seq {
    tokens: Vec<Token>,
    cur: usize,
}

impl Seq {
    pub fn new() -> Self {
        Seq {
            tokens: Vec::new(),
            cur: 0,
        }
    }

    pub fn add(&mut self, t: Token) {
        self.tokens.push(t);
    }

    #[allow(dead_code)]
    pub fn get(&self, idx: usize) -> Option<&Token> {
        self.tokens.get(self.cur + idx)
    }

    #[allow(dead_code)]
    pub fn mov(&mut self, off: isize) {
        self.cur = (self.cur as isize + off) as usize;
    }
}
