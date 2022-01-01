use super::node::*;
use crate::token::{Value};

pub trait Doter {
    fn dot(&self, id: usize) -> usize;
}

impl Doter for FactorNd {
    fn dot(&self, id: usize) -> usize {
        match self {
            FactorNd::Value(Value::Int(n)) => println!("{}[label=\"{}\"];", id, n),
            FactorNd::Value(Value::Bool(n)) => println!("{}[label=\"{}\"];", id, n),
        }
        id
    }
}

impl Doter for TermNd {
    fn dot(&self, id: usize) -> usize {
        let ida = self.a.dot(id);
        let mut id = ida + 1;
        if self.b.is_some() {
            let idb = self.b.as_ref().unwrap().0.dot(id);
            id = idb + 1;
            println!("{}->{};", id, idb);
        }
        println!("{}->{};", id, ida);
        println!("{}[label=\"Term\"];", id);
        id
    }
}

impl Doter for ExprNd {
    fn dot(&self, id: usize) -> usize {
        let ida = self.a.dot(id);
        let mut id = ida + 1;
        if self.b.is_some() {
            let idb = self.b.as_ref().unwrap().0.dot(id);
            id = idb + 1;
            println!("{}->{};", id, idb);
        }
        println!("{}->{};", id, ida);
        println!("{}[label=\"Expr\"];", id);
        id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::parser::*;
    use crate::token::lexer;

    #[test]
    fn print_dot() {
        let code = "1*2+3*5";
        if let Ok(s) = lexer(code) {
            if let Ok(Seq(_, n)) = ExprNd::parse(&s) {
                n.dot(0);
            }
        }
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
