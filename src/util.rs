use std::fs::{self, read_to_string};

pub fn open_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Could not open file {file_path}")
}

pub fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}
