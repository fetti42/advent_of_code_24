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

fn times_in_string(word: &str, text: &str) -> usize {
    let re = Regex::new(&word).unwrap();
    let matches: Vec<String> = re.find_iter(&text).map(|m| m.as_str().to_string()).collect();
    return matches.len();
}

fn main () {

    // read input

    let mut running_sum = 0;
    let mut rows: Vec<String> = Vec::new();
    let word = &"XMAS".to_string();
    //let mut temp: String;

    if let Ok(lines) = read_lines("../inputs/day4_easier.txt") {
        println!("reading file");

        for line in lines.flatten() {
            rows.push(line);
        }

        // search the rows forward
        for each in &rows {
            let bar = times_in_string(&word, &each);
            running_sum += bar;
        }

        // search the rows backwards
        for each in &rows {
            let foo: String = each.chars().rev().collect();
            let bar = times_in_string(&word, &foo);
            running_sum += bar;
        }

        //make columns
        let mut columns: Vec<String> = rows[0].chars().map(|x| x.to_string()).collect();
        for row in &rows[1..] {
            for i in 0..row.len() {
                columns[i] += &row.chars().nth(i).unwrap().to_string();
            }
        }
        println!("{}", columns[0]);

        // search columns
        // forward
        for each in &columns {
            let bar = times_in_string(&word, &each);
            running_sum += bar;
        }

        // backwards
        for each in &columns {
            let foo: String = each.chars().rev().collect();
            let bar = times_in_string(&word, &foo);
            running_sum += bar;
        }

        // make diagonals
        let mut r_diagonals: Vec<String> = Vec::new();

        println!("{}", running_sum);

        // find row diagonals
        for i in 0..rows[0].len() {
            let mut row_diagonal = String::new();
            for j in 0..rows[0].len()-i {
                row_diagonal.push(rows[j].chars().nth(i+j).unwrap());
            }
            r_diagonals.push(row_diagonal);

        }

        for i in 0..columns[0].len() { // needs to account for it not being square
            let mut col_diagonal = String::new();
            for j in 0..columns[0].len()-i {
                col_diagonal.push(rows[j].chars().nth(i+j).unwrap());
            }
            println!("{}", col_diagonal);
            r_diagonals.push(col_diagonal);
        }

        for each in &r_diagonals {
            let bar = times_in_string(&word, &each);
            running_sum += bar;
        }

        // backwards
        for each in &r_diagonals {
            let foo: String = each.chars().rev().collect();
            let bar = times_in_string(&word, &foo);
            running_sum += bar;
        }


        let mut l_diagonals: Vec<String>;


    }

    println!("{} appears {} times", word, running_sum);

}