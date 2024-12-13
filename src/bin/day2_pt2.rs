use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// file I/O - read in file

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn safe (report: Vec<i32>) -> bool {

    let mut foo = report.clone();
    //check the order
    foo.sort();
    if foo != report {
        foo.reverse();
        if foo != report {
            return false;   
        }
    }

    //check the intervals
    let mut curr_val = &foo[0];

    for val in &foo[1..] {
        if (curr_val - val).abs() < 1 {
            return false;
        }
        
        if (curr_val - val).abs() > 3 {
            return false;
        }
        curr_val = val;
    }

    return true;
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
            
            if safe(report.clone()) {
                safe_count += 1;    
            }

            else { // this is so inefficient
                let length = report.len();
                for i in 0..length {
                    let mut foo = report.clone();
                    foo.remove(i);
                    if safe(foo) {
                        safe_count += 1;
                        break;
                    }
                }
            }
        }
    }

    println!("Safe reports: {}", safe_count);
}
