use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn fuel_required(mass: u64) -> u64 {
    std::cmp::max(mass / 3, 2) - 2
}

fn fuel_required_recursive(mass: u64) -> u64 {
    let mut total = 0;
    let mut extra = fuel_required(mass);
    while extra > 0 {
        total += extra;
        extra = fuel_required(extra);
    }

    total
}

fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);

    let sum: u64 = reader.lines()
        .map(|l| l.expect("Failed to read a line"))
        .map(|l| l.trim().parse::<u64>().expect(&format!("{} wasn't a valid u64", l)))
        .map(fuel_required_recursive)
        .sum();

    println!("Sum of fuel required: {}", sum);

    Ok(())
}