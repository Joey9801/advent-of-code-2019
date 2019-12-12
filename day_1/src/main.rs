use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn fuel_required(mass: u64) -> u64 {
    (mass / 3) - 2
}

fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);

    let sum: u64 = reader.lines()
        .map(|l| l.expect("Failed to read a line"))
        .map(|l| l.trim().parse::<u64>().expect(&format!("{} wasn't a valid u64", l)))
        .map(fuel_required)
        .sum();

    println!("Sum of fuel required: {}", sum);

    Ok(())
}