#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Value {
    Int(i32),
    Bool(bool),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Type {
    Void,
    Int,
    Bool,
}

pub fn get_value_type(v: Value) -> Type {
    match v {
        Value::Int(_) => Type::Int,
        Value::Bool(_) => Type::Bool,
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Single
    Add,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Assign,
    Semicolon,
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Comma,

    // keywords
    If,
    Else,
    While,
    Return,
    Continue,
    Break,

    // Special
    Type(Type),
    Value(Value),
    Name(String),
}

const RESERVED_SINGLE_CHAR_TOKENS: [(char, Token); 14] = [
    ('+', Token::Add),
    ('-', Token::Minus),
    ('*', Token::Multiply),
    ('/', Token::Divide),
    ('%', Token::Modulo),
    ('=', Token::Assign),
    ('[', Token::LBracket),
    (']', Token::RBracket),
    ('(', Token::LParen),
    (')', Token::RParen),
    ('{', Token::LBrace),
    ('}', Token::RBrace),
    (';', Token::Semicolon),
    (',', Token::Comma),
];

const RESERVED_KEYWORDS: [(&'static str, Token); 11] = [
    ("if", Token::If),
    ("else", Token::Else),
    ("while", Token::While),
    ("return", Token::Return),
    ("break", Token::Break),
    ("continue", Token::Continue),
    ("void", Token::Type(Type::Void)),
    ("int", Token::Type(Type::Int)),
    ("bool", Token::Type(Type::Bool)),
    ("true", Token::Value(Value::Bool(true))),
    ("false", Token::Value(Value::Bool(false))),
];

// Only positive & No overflow
fn get_num(word: &str) -> Option<Token> {
    let mut num = 0i32;
    for c in word.chars() {
        if !c.is_ascii_digit() {
            return None;
        }
        let d = (c as u8) - ('0' as u8);
        num = num * 10 + d as i32;
    }
    Some(Token::Value(Value::Int(num)))
}

fn get_name(word: &str) -> Option<Token> {
    let mut first = true;
    for c in word.chars() {
        if first && c.is_ascii_digit() {
            return None;
        }
        if !c.is_ascii_alphabetic() && c != '_' {
            return None;
        }
        first = false;
    }
    Some(Token::Name(word.to_owned()))
}

pub fn get_token_from_word(word: &str) -> Option<Token> {
    for it in RESERVED_KEYWORDS.iter() {
        if it.0 == word {
            return Some(it.1.clone());
        }
    }
    if let Some(t) = get_num(word) {
        Some(t)
    } else if let Some(t) = get_name(word) {
        Some(t)
    } else {
        None
    }
}

pub fn get_token_from_char(ch: char) -> Option<Token> {
    for it in RESERVED_SINGLE_CHAR_TOKENS.iter() {
        if it.0 == ch {
            return Some(it.1.clone());
        }
    }
    None
}
