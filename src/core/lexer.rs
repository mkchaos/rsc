use super::types::{ErrKind, Token, get_token_from_char, get_token_from_word};

#[allow(dead_code)]
pub fn lexer(code: &str) -> Result<Vec<Token>, ErrKind> {
    let mut word: String = String::new();
    let mut tokens: Vec<Token> = Vec::new();
    macro_rules! try_push_word {
        () => {
            if word.len() > 0 {
                if let Some(t) = get_token_from_word(&word) {
                    // seq.add(t);
                    tokens.push(t);
                } else {
                    return Err(ErrKind::LexErr);
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
    use super::super::types::Type;
    use super::*;

    #[test]
    fn test_lexer() {
        let code = "int foo() {}";
        match lexer(code) {
            Ok(tks) => {
                // let seq = Sequence::new(tks);
                assert_eq!(tks[0], Token::Type(Type::Int));
                assert_eq!(tks[1], Token::Name("foo".to_owned()));
                assert_eq!(tks[2], Token::LParen);
                assert_eq!(tks[3], Token::RParen);
                assert_eq!(tks[4], Token::LBrace);
                assert_eq!(tks[5], Token::RBrace);
            }
            Err(_) => {
                assert!(false, "Not Ok");
            }
        }
    }
}
