use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const ITEMS: &str = " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    if let Ok(lines) = read_lines("input/day03") {
        let mut priorities = 0;
        let mut group: Vec<HashSet<char>> = vec![];
        let mut badged_priorities = 0;

        for line in lines {
            if let Ok(rucksack) = line {
                let compartments = rucksack.split_at(rucksack.len() / 2);
                let front = HashSet::<_>::from_iter(compartments.0.chars());
                let back = HashSet::<_>::from_iter(compartments.1.chars());

                priorities += front.intersection(&back).map(|c| {
                    ITEMS.find(*c).unwrap() as i32
                }).sum::<i32>();

                group.push(HashSet::<_>::from_iter(rucksack.chars()));
                if group.len() == 3 {
                    // yeah, i dno, i'm not asking the questions here
                    let intersect = &(&group[0] & &group[1]) & &group[2];
                    badged_priorities += intersect.iter().map(|c| {
                        ITEMS.find(*c).unwrap() as i32
                    }).sum::<i32>();
                    group = vec![];
                }
            }
        }

        println!("Sum of priorities: {}", priorities);
        println!("Sum of badged priorities: {}", badged_priorities);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
