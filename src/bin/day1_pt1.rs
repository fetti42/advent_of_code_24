use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// file I/O - read in file

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}




fn main () {

    let mut first_list = Vec::new();
    let mut sec_list = Vec::new();

    if let Ok(lines) = read_lines("../inputs/day1_input.txt") {
        println!("reading file");
        for line in lines.flatten() {
            let temp: Vec<&str> = line.split(' ').collect();
            first_list.push(temp.first().unwrap().parse::<u32>().unwrap());
            sec_list.push(temp.last().unwrap().parse::<u32>().unwrap());
        }
    }
    println!("List len: {}",first_list.len());

}