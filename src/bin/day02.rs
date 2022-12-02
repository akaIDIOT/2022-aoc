extern crate core;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn beats(opponent: &str, response: &str) -> i32 {
    return match (opponent, response) {
        ("A", "X") => 3, // rock == rock
        ("A", "Y") => 6, // rock < paper
        ("B", "Y") => 3, // paper == paper
        ("B", "Z") => 6, // paper < scissors
        ("C", "Z") => 3, // scissors == scissors
        ("C", "X") => 6, // scissors < rock
        _ => 0,          // response loses
    }
}

fn main() {
    if let Ok(lines) = read_lines("input/day02") {
        let mut score = 0;
        for line in lines {
            if let Ok(round) = line {
                if let Some((opponent, response)) = round.split_once(' ') {
                    score += match response {
                        "X" => 1,
                        "Y" => 2,
                        "Z" => 3,
                        _ => panic!("unrecognized response: {}", response),
                    };
                    score += beats(opponent, response);
                }
            }
        }

        println!("Total score for encrypted strategy: {}", score);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
