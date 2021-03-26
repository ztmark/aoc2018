use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom, Read};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    println!("hello world");

    let mut sum = 0;
    if let Ok(f) = File::open("src/day1/input.txt") {
        let mut reader = BufReader::new(f);
        for line in reader.lines() {
            if let Ok(l) = line {
                sum += l.parse::<i32>()?;
            }
        }
    }
    println!("sum={}", sum);

    println!("part2 = {}", part2());

    Ok(())
}

fn calc() {
    let mut sum = 0;
    if let Ok(f) = File::open("src/day1/input.txt") {
        let mut reader = BufReader::new(f);
        for line in reader.lines() {
            if let Ok(l) = line {
                sum += l.parse::<i32>().unwrap_or(0);
            }
        }
    }
    println!("sum={}", sum);
}

fn part2() -> i32 {
    let mut seen = HashSet::new();
    let mut sum = 0;
    seen.insert(sum);
    if let Ok(mut f) = File::open("src/day1/input.txt") {
        let mut data = String::new();
        if let Ok(_) = f.read_to_string(&mut data) {
            loop {
                for line in data.lines() {
                    sum += line.parse::<i32>().unwrap_or(0);
                    if seen.contains(&sum) {
                        return sum;
                    }
                    seen.insert(sum);
                }
            }
        }
    }
    return sum;
}