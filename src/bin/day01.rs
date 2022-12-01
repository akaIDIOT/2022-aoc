use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("input/day01") {
        let mut sum = 0;
        let mut elves: Vec<i32> = vec![];

        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(calories) = line {
                if calories.is_empty() {
                    elves.push(sum);
                    sum = 0;
                }
                else {
                    sum += calories.parse::<i32>().unwrap();
                }
            }
        }

        elves.sort();

        println!("Elf named Max carries {} calories.",
                 elves.last().unwrap());
        println!("Elves Maxwell, Maxxie and Max together carry {} calories.",
                 elves.iter().rev().take(3).sum::<i32>());
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
