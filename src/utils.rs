use std::fs;

#[allow(dead_code)]
pub fn load_code_from_file(path: &str) -> String {
    fs::read_to_string(path).expect("No file")
}
