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

fn times_in_string(word: &String, text: &String) -> usize {
    let re = Regex::new(&word).unwrap();
    let matches: Vec<String> = re.find_iter(&text).map(|m| m.as_str().to_string()).collect();
    return matches.len();

}



fn main () {

    // read input

    let mut running_sum = 0;
    let mut text: Vec<String> = Vec::new();
    let mut foo: String;
    let word = &"XMAS".to_string();
    //let mut temp: String;

    if let Ok(lines) = read_lines("../inputs/day4_test.txt") {
        println!("reading file");

        //search forward on horizontals
        for line in lines.flatten() {
            //foo = line.as_string();
            text.push(line);
            //println!("{}",line);
            //let temp = line.clone().to_owned(); 
            //running_sum += times_in_string(word, &foo.as_str());
        }

        for each in &text {
            let bar = times_in_string(word, each);
            running_sum += bar;
            println!("{} appears {} times forwards", word, bar);
        }

        for each in &text {
            let bar = times_in_string(word, each.as_str().chars().rev().collect());
            running_sum += bar;
            println!("{} appears {} times backwards", word, bar);

        }

        //searching backward on horizontals
        //for line in lines {
        //    let foo = line.unwrap().clone().chars().rev().collect::<String>();
        //    running_sum += times_in_string(word, &foo);
       // }
    }

    println!("{} appears {} times", word, running_sum);

}