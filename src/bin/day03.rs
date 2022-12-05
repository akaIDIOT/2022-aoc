use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const ITEMS: &str = " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    if let Ok(lines) = read_lines("input/day03") {
        let mut sum = 0;
        for line in lines {
            if let Ok(rucksack) = line {
                let compartments = rucksack.split_at(rucksack.len() / 2);
                let front = HashSet::<_>::from_iter(compartments.0.chars());
                let back = HashSet::<_>::from_iter(compartments.1.chars());

                sum += front.intersection(&back).map(|c| {
                    ITEMS.find(*c).unwrap() as i32
                }).sum::<i32>();
            }
        }

        println!("Sum of priorities: {}", sum);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
