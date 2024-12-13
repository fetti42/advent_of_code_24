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

    // read in numbers
    if let Ok(lines) = read_lines("../inputs/day1_input.txt") {
        println!("reading file");
        for line in lines.flatten() {
            let temp: Vec<&str> = line.split(' ').collect();
            first_list.push(temp[0].parse::<i32>().unwrap());
            sec_list.push(temp.last().unwrap().parse::<i32>().unwrap());
        }
    }

    //sort lists
    first_list.sort();
    sec_list.sort();

    //let foo = min(sec_list.len(), 5);
    //println!("{:?}",&sec_list[..foo]);

    let mut similarity = 0;

    for each in &first_list {
        let mut appearances = 0;
        for foo in &sec_list {
            if each == foo {
                appearances += 1;
            }
            else if each > foo {
                continue;
            }
        }
        similarity += appearances * each;
    }

    println!("Similarity: {}", similarity);

}