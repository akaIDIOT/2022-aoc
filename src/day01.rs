use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("input/day01") {
        let mut sum = 0;
        let mut max = 0;

        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(calories) = line {
                if calories.is_empty() {
                    if sum > max {
                        max = sum;
                    }
                    sum = 0;
                }
                else {
                    sum += calories.parse::<i32>().unwrap();
                }
            }
        }

        println!("Elf named Max carries {} calories.", max);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
