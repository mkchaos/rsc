use super::super::parser::Parser;
use super::err::ErrKind;
use super::nodes::FactorNd;
use super::seq::{SeqPack, Sequence};
use super::token::{Token, Type};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use std::collections::HashMap;

#[derive(Debug, EnumIter, PartialEq, Eq, Clone, Copy)]
pub enum Op {
    Paren,
    UnaryMinus,
    Not,
    Multiply,
    Divide,
    Modulo,
    Add,
    Minus,
    GreaterEq,
    GreaterThan,
    LessEq,
    LessThan,
    Eq,
    NotEq,
    And,
    Or,
}

// op (param num, num of ops)
const LEVEL_OPS: [(u8, u8); 8] = [
    (1, 1),
    (1, 2),
    (2, 3),
    (2, 2),
    (2, 4),
    (2, 2),
    (2, 1),
    (2, 1),
];

fn get_level(op: Op) -> u8 {
    let mut off = 0;
    for (idx, o) in Op::iter().enumerate() {
        if op == o {
            off = idx;
            break;
        }
    }
    for (idx, (_, n)) in LEVEL_OPS.iter().enumerate() {
        let n = *n as usize;
        if off < n {
            return idx as u8;
        }
        off -= n;
    }
    panic!("should not reach here");
}

fn max_level() -> u8 {
    LEVEL_OPS.len() as u8 - 1
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

#[derive(Debug, Clone)]
pub enum CalcItem {
    Op(Op),
    Factor(FactorNd),
}

fn eat_op(
    seq: Sequence,
    op: Op,
    mp: &mut HashMap<(u8, u8), SeqPack<Vec<CalcItem>>>,
) -> SeqPack<Vec<CalcItem>> {
    let level = get_level(op);
    match op {
        Op::Paren => {
            let (seq, _) = seq.eat(Token::LParen)?;
            let (seq, st) = _get_calc_stack(seq, max_level(), mp)?;
            let (seq, _) = seq.eat(Token::RParen)?;
            Some((seq, st))
        }
        Op::UnaryMinus => {
            let (seq, _) = seq.eat(Token::Minus)?;
            let (seq, mut st) = _get_calc_stack(seq, level - 1, mp)?;
            st.push(CalcItem::Op(Op::UnaryMinus));
            Some((seq, st))
        }
        Op::Not => {
            let (seq, _) = seq.eat(Token::Not)?;
            let (seq, mut st) = _get_calc_stack(seq, level, mp)?;
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
            let (seq, st) = _get_calc_stack(seq, level - 1, mp)?;
            stack.extend(st);
            let (seq, _) = seq.eats(&bin_op_tokens(op))?;
            let (seq, st) = _get_calc_stack(seq, level, mp)?;
            stack.extend(st);
            stack.push(CalcItem::Op(op));
            Some((seq, stack))
        }
    }
}

fn _get_calc_stack(
    seq: Sequence,
    level: u8,
    mp: &mut HashMap<(u8, u8), SeqPack<Vec<CalcItem>>>,
) -> SeqPack<Vec<CalcItem>> {
    let ky = (seq.len() as u8, level);
    if mp.contains_key(&ky) {
        return mp[&ky].clone();
    }
    mp.insert(ky, None);
    for op in Op::iter() {
        if get_level(op) != level {
            continue;
        }
        if let Some((seq, st)) = eat_op(seq.clone(), op, mp) {
            let res = Some((seq, st));
            mp.insert(ky, res.clone());
            return res;
        }
    }

    let res = if level == 0 {
        let (seq, factor) = FactorNd::parse(seq)?;
        Some((seq, vec![CalcItem::Factor(factor)]))
    } else {
        _get_calc_stack(seq, level - 1, mp)
    };
    mp.insert(ky, res.clone());
    res
}

#[allow(dead_code)]
pub fn get_calc_stack(seq: Sequence) -> SeqPack<Vec<CalcItem>> {
    let mut mp = HashMap::new();
    _get_calc_stack(seq, max_level(), &mut mp)
}

#[allow(dead_code)]
pub fn get_op_param_num(op: Op) -> usize {
    let level = get_level(op);
    LEVEL_OPS[level as usize].0 as usize
}

pub fn calc_op_1(op: Op, a: i32) -> Result<i32, ErrKind> {
    match op {
        Op::Paren => Ok(a),
        Op::UnaryMinus => Ok(-a),
        Op::Not => {
            if a != 0 {
                Ok(0)
            } else {
                Ok(1)
            }
        }
        _ => {
            panic!("{:?} is not op_1", op);
        }
    }
}

pub fn calc_op_2(op: Op, a: i32, b: i32) -> Result<i32, ErrKind> {
    match op {
        Op::Multiply => Ok(a * b),
        Op::Divide => {
            if b == 0 {
                Err(ErrKind::DivideZero)
            } else {
                Ok(a / b)
            }
        }
        Op::Modulo => {
            if b == 0 {
                Err(ErrKind::DivideZero)
            } else {
                Ok(a % b)
            }
        }
        Op::Add => Ok(a + b),
        Op::Minus => Ok(a - b),
        Op::GreaterEq => {
            if a >= b {
                Ok(1)
            } else {
                Ok(0)
            }
        }
        Op::GreaterThan => {
            if a > b {
                Ok(1)
            } else {
                Ok(0)
            }
        }
        Op::LessEq => {
            if a <= b {
                Ok(1)
            } else {
                Ok(0)
            }
        }
        Op::LessThan => {
            if a < b {
                Ok(1)
            } else {
                Ok(0)
            }
        }
        Op::Eq => {
            if a == b {
                Ok(1)
            } else {
                Ok(0)
            }
        }
        Op::NotEq => {
            if a != b {
                Ok(1)
            } else {
                Ok(0)
            }
        }
        Op::And => {
            if a != 0 && b != 0 {
                Ok(1)
            } else {
                Ok(0)
            }
        }
        Op::Or => {
            if a != 0 || b != 0 {
                Ok(1)
            } else {
                Ok(0)
            }
        }
        _ => {
            panic!("{:?} is not op_2", op);
        }
    }
}

pub fn retrieve_type(st: &[CalcItem]) -> Result<Type, ErrKind> {
    Err(ErrKind::TypeErr)
}
