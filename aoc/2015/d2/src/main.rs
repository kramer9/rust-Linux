#![allow(clippy::manual_flatten)]
// https://adventofcode.com/2015/day/2

/*
1 (read in input) - They have a list of the dimensions (length l, width w, and height h) of each present

*/

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    //hardcoding the variable rather then accepting it via command line
    let file_path: &str = "/home/argus/Drives/2TBa/Projects/rust-Linux/aoc/2015/d2/src/inputp1.txt";
    let mut l: i32 = 0b0;
    let mut w: i32 = 0b0;
    let mut h: i32 = 0b0;

    println!("In file {}", file_path);

// need to understand it - https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
// vec is vector, basically an array
    if let Ok(presents) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String
        for present in presents {
            if let Ok(dimension) = present {
                println!("{}", dimension);
                let split_dimension: Vec<i32> = dimension
                    .trim()                             /* returns a new string based on limiter? */
                    .split("x")                         /* deliminiter */
                    .map(|val| val.parse().unwrap())    /*map is a function bring applied to split_dimension, pulling the parsed value*/
                    .collect();                         /* just collects the values, does not modify them */
                let l: i32 = split_dimension[0];
                let w: i32 = split_dimension[1];
                let h: i32 = split_dimension[2];
                println!("length {} width {} height {}", l, w, h);
 //               remember 3 variable above are toast post loop
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
let file = File::open(filename)?;
Ok(io::BufReader::new(file).lines())
}