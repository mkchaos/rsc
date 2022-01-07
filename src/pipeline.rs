

use std::fs;

fn load_code_from_file(path: &str) -> String {
    fs::read_to_string(path).expect("No file")
}