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

fn required_move<'o>(opponent: &'o str, result: &str) -> &'o str {
    return match (opponent, result) {
        ("A", "X") => "C",    // lose: scissors
        ("A", "Z") => "B",    // win: paper
        ("B", "X") => "A",    // lose: rock
        ("B", "Z") => "C",    // win: scissors
        ("C", "X") => "B",    // loes: paper
        ("C", "Z") => "A",    // win: rock
        (_, "Y") => opponent, // draw: copy opponent's move
        _ => panic!("unrecognized result: {}", result),
    }
}

fn main() {
    if let Ok(lines) = read_lines("input/day02") {
        let mut score1 = 0;
        let mut score2 = 0;
        for line in lines {
            if let Ok(round) = line {
                if let Some((opponent, response)) = round.split_once(' ') {
                    score1 += match response {
                        "X" => 1,
                        "Y" => 2,
                        "Z" => 3,
                        _ => panic!("unrecognized response: {}", response),
                    };
                    score1 += beats(opponent, response);

                    let me = required_move(opponent, response);
                    score2 += match me {
                        "A" => 1,
                        "B" => 2,
                        "C" => 3,
                        _ => panic!("unrecognized move: {}", me),
                    };
                    score2 += match response {
                        "X" => 0,
                        "Y" => 3,
                        "Z" => 6,
                        _ => panic!("unrecognized result: {}", response),
                    }
                }
            }
        }

        println!("Total score for encrypted strategy: {}", score1);
        println!("Total score for better strategy: {}", score2);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
