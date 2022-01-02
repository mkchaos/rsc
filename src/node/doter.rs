use super::node::*;
use crate::token::Value;

pub trait Doter {
    fn dot(&self, id: usize) -> usize;
}

impl Doter for FactorNd {
    fn dot(&self, id: usize) -> usize {
        let mut id = id;
        match self {
            FactorNd::Value(Value::Int(n)) => println!("{}[label=\"{}\"];", id, n),
            FactorNd::Value(Value::Bool(n)) => println!("{}[label=\"{}\"];", id, n),
            FactorNd::Var(n) => println!("{}[label=\"{}\"];", id, n.name),
            FactorNd::Expr(n) => {
                id = n.dot(id);
            }
        }
        id
    }
}

impl Doter for TermNd {
    fn dot(&self, id: usize) -> usize {
        let ida = self.a.dot(id);
        if self.b.is_some() {
            let mut id = ida + 1;
            let idb = self.b.as_ref().unwrap().0.dot(id);
            id = idb + 1;
            println!("{}->{};", id, idb);
            println!("{}->{};", id, ida);
            println!("{}[label=\"{:?}\"];", id, self.b.as_ref().unwrap().1);
            id
        } else {
            ida
        }
    }
}

impl Doter for ExprNd {
    fn dot(&self, id: usize) -> usize {
        let ida = self.a.dot(id);
        if self.b.is_some() {
            let mut id = ida + 1;
            let idb = self.b.as_ref().unwrap().0.dot(id);
            id = idb + 1;
            println!("{}->{};", id, idb);
            println!("{}->{};", id, ida);
            println!("{}[label=\"{:?}\"];", id, self.b.as_ref().unwrap().1);
            id
        } else {
            ida
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::parser::*;
    use super::*;
    use crate::token::{lexer, Sequence};

    #[test]
    fn print_dot() {
        let code = "1*2*4+3*5+6";
        if let Ok(s) = lexer(code) {
            let seq = Sequence::new(s);
            if let Some((_, n)) = ExprNd::parse(seq) {
                n.dot(0);
            }
        }
    }
}
