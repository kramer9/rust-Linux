#![allow(clippy::manual_flatten)]
#[allow(unused_imports)] //turns off the clippy warning about unused imports
// https://adventofcode.com/2015/day/3

/*
1 (read in input) - its one big long line
*/

fn calculate_sum(a: i32, b: i32) -> i32 {
    a + b
}

use std::{env, fs}; //need to import this to support querying for the path where we are running
use std::path::Path; //need for the read_lines function
use std::io::{self, BufRead}; //need for the read_lines function
use std::fs::File; //need for the open of the input file


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let input = File::open(filename)?;
    Ok(io::BufReader::new(input).lines())
}

fn main() {
    // get input
    let current_dir = env::current_dir().unwrap();
    println!("I am working in {}", current_dir.display()); //https://doc.rust-lang.org/std/path/struct.Path.html#method.display
    let input_file: String = format!("{}/src/inputp1.txt", current_dir.display()); //set the input file location, using current dir to support work from different platforms
    println!("In file {}", input_file);

if let Ok(input) = read_lines(input_file) {
        // Consumes the iterator, returns an (Optional) String
        // key points, input is NOT the entire input file in memory, its a pointer
        // lines reads in the first who line into a string
        for lines in input {
            if let Ok(line) = lines { //pull the first line in the list
//                println!("{}",line);
                for c in line.chars() { //iterate through the chars in the line
                    println!("d {}",c);
                }
            }
        }
    }
   

}
