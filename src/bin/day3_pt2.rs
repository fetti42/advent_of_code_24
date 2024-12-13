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

    let mut running_sum = 0;
    // read input
    let re = Regex::new(r"mul\((?<num1>\d\d?\d?),(?<num2>\d\d?\d?)\)").unwrap();

    if let Ok(lines) = read_lines("../inputs/day3_input.txt") {
        println!("reading file");

        for line in lines.flatten() {
            let muls: Vec<(i32, i32)> = re.captures_iter(&line).map(|caps| {
                let num_1 = caps.name("num1").unwrap().as_str().parse::<i32>().unwrap();
                let num_2 = caps.name("num2").unwrap().as_str().parse::<i32>().unwrap(); 
                (num_1, num_2)
            }).collect();
            println!("num 1: {}, num 2: {}", muls[0].0, muls[0].1);

            for each in muls {
                running_sum += each.0 * each.1;
            }

        }
        println!("Sum is: {}", running_sum);
    }

}