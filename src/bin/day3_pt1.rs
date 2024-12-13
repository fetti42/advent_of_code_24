use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

// file I/O - read in file

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn main () {

    // read input

    if let Ok(lines) = read_lines("../inputs/day1_input.txt") {
        println!("reading file");
        for line in lines.flatten() {
        }
    }

}