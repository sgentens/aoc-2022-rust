use std::env;
use std::fs;


fn main() {
    let file_path = "/vcs/personal/aoc/2022/rust/inputs/day1.txt"
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}