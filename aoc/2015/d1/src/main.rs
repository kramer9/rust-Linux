// https://adventofcode.com/2015/day/1

use std::fs;

fn main() {
    let file_path: &str = "/home/argus/Drives/2TBa/Projects/rust-Linux/aoc/2015/d1/src/input.txt";
    // --snip--
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}