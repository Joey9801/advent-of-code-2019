use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::VecDeque;

use intcode_vm::{ProgramState, ProgramElement};

fn main() {
    let file = File::open("./input.txt").expect("Failed to open input file");
    let reader = BufReader::new(file);

    let initial_mem: Vec<_> = reader
        .split(b',')
        .map(|el| el.expect("Failed to read bytes from file"))
        .map(|el| String::from_utf8(el).expect("Bytes between a comma weren't UTF8"))
        .map(|el| el.trim().to_string())
        .map(|el| el.parse::<ProgramElement>().expect(&format!("Failed to parse {} as u64", el)))
        .collect();

    let mut inputs = VecDeque::new();
    inputs.push_back(5);
    let mut program = ProgramState::new(initial_mem, inputs);
    program.run_to_completion();
    println!("Program outputs = {:?}", program.outputs);
}