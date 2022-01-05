use crate::core::{FactorNd, SeqPack, Sequence, Token, Value};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter, PartialEq, Eq, Clone, Copy)]
pub enum Op {
    // 1
    Paren,
    // 2
    UnaryMinus,
    Not,
    // 3
    Multiply,
    Divide,
    Modulo,
    // 4
    Add,
    Minus,
    // 6
    GreaterEq,
    GreaterThan,
    LessEq,
    LessThan,
    // 7
    Eq,
    NotEq,
    // 8
    And,
    // 9
    Or,
}

#[derive(Debug, Clone)]
pub enum CalcItem {
    Op(Op),
    Factor(FactorNd),
}

fn eat_factor(_seq: Sequence) -> SeqPack<FactorNd> {
    None
}

fn op_priority(op: Op) -> u8 {
    match op {
        Op::Paren => 1,
        Op::UnaryMinus => 2,
        Op::Not => 2,
        Op::Multiply => 3,
        Op::Divide => 3,
        Op::Modulo => 3,
        Op::Add => 4,
        Op::Minus => 4,
        Op::GreaterEq => 6,
        Op::GreaterThan => 6,
        Op::LessEq => 6,
        Op::LessThan => 6,
        Op::Eq => 7,
        Op::NotEq => 7,
        Op::And => 8,
        Op::Or => 9,
    }
}

fn bin_op_tokens(op: Op) -> Vec<Token> {
    match op {
        Op::Multiply => vec![Token::Multiply],
        Op::Divide => vec![Token::Divide],
        Op::Modulo => vec![Token::Modulo],
        Op::Add => vec![Token::Add],
        Op::Minus => vec![Token::Minus],
        Op::GreaterEq => vec![Token::Greater, Token::Eq],
        Op::GreaterThan => vec![Token::Greater],
        Op::LessEq => vec![Token::Less, Token::Eq],
        Op::LessThan => vec![Token::Less],
        Op::Eq => vec![Token::Eq, Token::Eq],
        Op::NotEq => vec![Token::Not, Token::Eq],
        Op::And => vec![Token::And, Token::And],
        Op::Or => vec![Token::Or, Token::Or],
        _ => {
            panic!("not binary op: {:?}", op);
        }
    }
}

fn eat_op(seq: Sequence, op: Op) -> SeqPack<Vec<CalcItem>> {
    let priority = op_priority(op);
    match op {
        Op::Paren => {
            let (seq, _) = seq.eat(Token::LParen)?;
            let (seq, st) = get_calc_stack(seq)?;
            let (seq, _) = seq.eat(Token::RParen)?;
            Some((seq, st))
        }
        Op::UnaryMinus => {
            let (seq, _) = seq.eat(Token::Minus)?;
            let (seq, v) = seq.eat_value()?;
            match v {
                Value::Int(x) => {
                    Some((seq, vec![CalcItem::Factor(FactorNd::Value(Value::Int(-x)))]))
                }
                _ => None,
            }
        }
        Op::Not => {
            let (seq, _) = seq.eat(Token::Not)?;
            let (seq, mut st) = _get_calc_stack(seq, priority)?;
            st.push(CalcItem::Op(Op::Not));
            Some((seq, st))
        }
        Op::Multiply
        | Op::Divide
        | Op::Modulo
        | Op::Add
        | Op::Minus
        | Op::GreaterEq
        | Op::GreaterThan
        | Op::LessEq
        | Op::LessThan
        | Op::Eq
        | Op::NotEq
        | Op::And
        | Op::Or => {
            let mut stack = Vec::new();
            let (seq, n) = eat_factor(seq)?;
            let (seq, _) = seq.eats(&bin_op_tokens(op))?;
            let (seq, st) = _get_calc_stack(seq, priority)?;
            stack.push(CalcItem::Factor(n));
            stack.extend(st);
            stack.push(CalcItem::Op(op));
            Some((seq, stack))
        }
    }
}

fn _get_calc_stack(seq: Sequence, priority: u8) -> SeqPack<Vec<CalcItem>> {
    for op in Op::iter() {
        if op_priority(op) >= priority {
            continue;
        }
        let sp = eat_op(seq.clone(), op);
        if sp.is_some() {
            return sp;
        }
    }
    let (seq, n) = eat_factor(seq)?;
    Some((seq, vec![CalcItem::Factor(n)]))
}

#[allow(dead_code)]
pub fn get_calc_stack(seq: Sequence) -> SeqPack<Vec<CalcItem>> {
    _get_calc_stack(seq, !0)
}
