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

fn search_forward_backward(word: &str, text: &str) -> usize {
    let mut running_sum = 0;
    let rev_word: String = word.chars().rev().collect();
    let bar = times_in_string(&word, &text);
    let baz = times_in_string(&rev_word, &text);
    running_sum += bar;
    running_sum += baz;
    return running_sum;
}

fn main () {

    // read input

    let mut running_sum = 0;
    let mut rows: Vec<String> = Vec::new();
    let word = &"XMAS".to_string();

    if let Ok(lines) = read_lines("../inputs/day4_input.txt") {
        println!("reading file");

        for line in lines.flatten() {
            rows.push(line);
        }

        // search the rows
        for each in &rows {
            let bar = search_forward_backward(&word, &each);
            running_sum += bar;
        }

        //make columns
        let mut columns: Vec<String> = rows[0].chars().map(|x| x.to_string()).collect();
        for row in &rows[1..] {
            for i in 0..row.len() {
                columns[i] += &row.chars().nth(i).unwrap().to_string();
            }
        }

        // search columns
        for each in &columns {
            let bar = search_forward_backward(&word, &each);
            running_sum += bar;
        }

        // make diagonals
        let mut r_diagonals: Vec<String> = Vec::new();
        let mut l_diagonals: Vec<String> = Vec::new();

        // find row diagonals
        let length = rows[0].len();
        let height = rows.len();

        for i in 0..length {
            let mut r_row_diagonal = String::new();
            let mut l_row_diagonal = String::new();
            for j in 0..length-i {
                if i+j == height + 1 {
                    break;
                }
                r_row_diagonal.push(rows[j].chars().nth(i+j).unwrap());
                let rev: String = rows[j].chars().rev().collect();
                l_row_diagonal.push(rev.chars().nth(i+j).unwrap());

            }
            r_diagonals.push(r_row_diagonal);
            l_diagonals.push(l_row_diagonal);
        }

        for i in 1..height { 
            let mut r_col_diagonal = String::new();
            let mut l_col_diagonal = String::new();
            for j in 0..height-i {
                if i+j == length+1 {
                    break;
                }
                r_col_diagonal.push(rows[i+j].chars().nth(j).unwrap());
                let rev: String = rows[i+j].chars().rev().collect();
                l_col_diagonal.push(rev.chars().nth(j).unwrap());
            }
            r_diagonals.push(r_col_diagonal);
            l_diagonals.push(l_col_diagonal);
            
        }

        // count r_diagonals
        for each in &r_diagonals {
            let bar = search_forward_backward(&word, &each);
            running_sum += bar;
        }

        // count l_diagonals
        for each in &l_diagonals {
            let bar = search_forward_backward(&word, &each);
            running_sum += bar;
        }
    }

    println!("{} appears {} times", word, running_sum);

}