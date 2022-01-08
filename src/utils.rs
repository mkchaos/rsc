use std::fs;
use std::str;

fn preprocess_code_line(code: String) -> String {
    let bs = code.as_bytes();
    for i in 1..bs.len() {
        if bs[i] == b'/' && bs[i-1] == b'/' {
            let s = match str::from_utf8(&bs[..i-1]) {
                Ok(v) => return v.to_owned(),
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };
        }
    }
    code
}

fn preprocess_code(code: String) -> String {
    let mut vec = Vec::new();
    for s in code.split('\n') {
        vec.push(preprocess_code_line(s.to_owned()));
    }
    vec.connect("\n")
}

#[allow(dead_code)]
pub fn load_code_from_file(path: &str) -> String {
    preprocess_code(fs::read_to_string(path).expect("No file"))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_preprocess_code() {
        let code = "12345//123\n12345";
        let res = preprocess_code(code.to_owned());
        assert_eq!("12345\n12345", res);
    }
}
