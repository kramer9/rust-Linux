#![allow(clippy::manual_flatten)]
// https://adventofcode.com/2015/day/2

/*
1 (read in input) - They have a list of the dimensions (length l, width w, and height h) of each present
2 - break apart l/w/h
3 - find the surface area of the box, which is 2*l*w + 2*w*h + 2*h*l
4 - find the area of the smallest side
5 - add it to 3
6 - total square feet of wrapping paper
part 2
1 - is this basically just the lowest 2 numbers of the l, w, h list twice?
1.1 - sort l,w, h
2 - requires a bow made out of ribbon as well; the feet of ribbon required for the perfect bow is equal to the cubic feet of volume of the present. (l*w*h)
*/

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{env, fs}; //need to import this to support querying for the path where we are running

fn main() {
    //hardcoding the variable rather then accepting it via command line
//    let file_path: &str = "/home/argus/Drives/2TBa/Projects/rust-Linux/aoc/2015/d2/src/inputp1.txt";
    let mut l: i32 = 0b0;
    let mut w: i32 = 0b0;
    let mut h: i32 = 0b0;
    let mut rt: i32 = 0b0; //have to initialize rt up here, cuz you add it on itself within the loop
    let mut rt_ribbon: i32 = 0b0; //have to initialize rt up here, cuz you add it on itself within the loop

    let current_dir = env::current_dir().unwrap();
    println!("{}", current_dir.display()); //https://doc.rust-lang.org/std/path/struct.Path.html#method.display

    let file_path: String = format!("{}/src/inputp1.txt", current_dir.display());
    //let file_path: &str = "{current_dir}/src/inputp1.txt";
    println!("In file {}", file_path);

// need to understand it - https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
// vec is vector, basically an array
    if let Ok(presents) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String
        for present in presents {
            if let Ok(dimension) = present {
//                println!("{}", dimension);
                let split_dimension: Vec<i32> = dimension
                    .trim()                             /* returns a new string based on limiter? */
                    .split("x")                         /* deliminiter */
                    .map(|val| val.parse().unwrap())    /*map is a function bring applied to split_dimension, pulling the parsed value*/
                    .collect();                         /* just collects the values, does not modify them */
                let l: i32 = split_dimension[0];
                let w: i32 = split_dimension[1];
                let h: i32 = split_dimension[2];
                let sa: i32 = ((2*l*w) + (2*w*h) + (2*h*l));
                let min: i32 = std::cmp::min((l*w), (w*h)); //note dont use the *2 the *2 is for TWO sides, you want the SA for a single single
                let min: i32 = std::cmp::min(min, (h*l));
                rt = rt + sa + min;  //key point !!! notice this is not a let, the use of let resets it to zero every time

//sort l, w, h to pull only the bottom two as part of the calculation for the ribbon

                let mut array = [l, w, h]; //https://rosettacode.org/wiki/Sort_three_variables#Rust
                array.sort(); //remember its variable.function (sort is a built in function that we are passing variable into)
 //               println!("original: {}", dimension);
 //               println!("Sorted: {:?}", array);
 //               println!("Smallest: {} Medium: {}", array[0], array[1]);
                rt_ribbon = rt_ribbon + ((array[0]*2) + (array[1] * 2) + (l * w * h));
 //               println!("Ribbon total: {}", rt_ribbon);
                //|\ The primary use for the let keyword is in let statements, which are used to introduce a new set of variables into the current scope, as given by a pattern.
 //               println!("The minimum {}", min);
 //               println!("length {} width {} height {} sa {} rt {}", l, w, h, sa, rt);
 //               remember 3 variable above are toast post loop
            }
        }
    }
   
    println!("Part 1 total {}", rt);
    println!("Part 2 total {}", rt_ribbon);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
let file = File::open(filename)?;
Ok(io::BufReader::new(file).lines())
}