mod seq;
mod token;

pub use seq::{Sequence, SeqPack};
use token::{get_token_from_char, get_token_from_word};
pub use token::{Token, Type, Value, get_value_type};

#[derive(Debug, PartialEq, Clone)]
pub struct LexErr(String);

#[allow(dead_code)]
pub fn lexer(code: &str) -> Result<Vec<Token>, LexErr> {
    let mut word: String = String::new();
    let mut tokens: Vec<Token> = Vec::new();
    macro_rules! try_push_word {
        () => {
            if word.len() > 0 {
                if let Some(t) = get_token_from_word(&word) {
                    // seq.add(t);
                    tokens.push(t);
                } else {
                    return Err(LexErr(word.clone()));
                }
                word.clear();
            }
        };
    }
    for c in code.chars() {
        if c.is_ascii_whitespace() {
            try_push_word!();
        } else if let Some(t) = get_token_from_char(c) {
            try_push_word!();
            tokens.push(t);
        } else {
            word.push(c);
        }
    }
    try_push_word!();
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lexer() {
        let code = "int foo() {}";
        match lexer(code) {
            Ok(tks) => {
                let seq = Sequence::new(tks);
                assert_eq!(seq.get(0), Some(Token::Type(Type::Int)));
                assert_eq!(seq.get(1), Some(Token::Name("foo".to_owned())));
                assert_eq!(seq.get(2), Some(Token::LParen));
                assert_eq!(seq.get(3), Some(Token::RParen));
                assert_eq!(seq.get(4), Some(Token::LBrace));
                assert_eq!(seq.get(5), Some(Token::RBrace));
            }
            Err(_) => {
                assert!(false, "Not Ok");
            }
        }
    }
}
