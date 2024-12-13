use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// file I/O - read in file

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn safe (report: &Vec<i32>) -> bool {
    //check the order


    //check the intervals
}

fn main () {

    let mut safe_count = 0;
    let mut reports: Vec<Vec<i32>> = Vec::new();

    // read in numbers
    if let Ok(lines) = read_lines("../inputs/day2_input.txt") {
        println!("reading file");
        for line in lines.flatten() {
            let foo: Vec<i32> = line.split(' ').map(|x| x.parse::<i32>().unwrap()).collect();
            reports.push(foo);
        }

        for report in reports {
            let mut foo = report.clone();
            foo.sort();
            if foo != report {
                foo.reverse();
                if foo != report {
                    //println!{"{:?}", report};
                    continue;
                }
            }

            let mut curr_val = &foo[0];
            let mut bad = false;

            for val in &foo[1..] {
                if (curr_val - val).abs() < 1 {
                    bad = true;
                    break;
                }
                if (curr_val - val).abs() > 3 {
                    bad = true;
                    break;
                }
                curr_val = val;
            }
            if bad {
                continue;
            }
            safe_count += 1;
        }
    }

    println!("Safe reports: {}", safe_count);
}
