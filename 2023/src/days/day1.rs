use std::{fs, path::PathBuf};

pub fn solve(input_path: &PathBuf) -> (String, String) {
    let _input = fs::read_to_string(input_path).expect("Should have been able to read the file");

    ("let's go".to_owned(), "let's go".to_owned())
}
