use std::fs::File;
use std::io::{prelude::*, BufReader};

const INSTR_ADD: usize = 1;
const INSTR_MUL: usize = 2;
const INSTR_TERM: usize = 99;

trait Offset {
    fn offset(&self, offset: isize) -> Self;
}

impl Offset for usize {
    fn offset(&self, offset: isize) -> usize {
        if offset > 0 {
            self + (offset as usize)
        } else {
            self - (-offset as usize)
        }
    }
}

struct ProgramState {
    values: Vec<usize>,
    program_counter: usize,
}

impl ProgramState {
    fn new(values: Vec<usize>) -> Self {
        debug_assert!(values.len() > 0);

        Self {
            values,
            program_counter: 0,
        }
    }

    /// Returns the value at a given offset from the program counter
    fn read_rel(&self, offset: isize) -> usize {
        self.values[self.program_counter.offset(offset)]
    }

    /// Returns the value at the address stored at the given offset from the program counter
    fn read_rel_ptr(&self, offset: isize) -> usize {
        let idx = self.read_rel(offset);
        self.values[idx]
    }

    fn terminated(&self) -> bool {
        self.read_rel(0) == INSTR_TERM
    }

    fn progress_state(&mut self) {
        if self.terminated() {
            return;
        }

        match self.read_rel(0) {
            INSTR_ADD => {
                let a = self.read_rel_ptr(1);
                let b = self.read_rel_ptr(2);
                let c_idx = self.read_rel(3);
                self.values[c_idx] = a + b;
            },
            INSTR_MUL => {
                let a = self.read_rel_ptr(1);
                let b = self.read_rel_ptr(2);
                let c_idx = self.read_rel(3);
                self.values[c_idx] = a * b;
            },
            other => panic!(format!("Processing unknown opcode: \"{}\"", other)),
        }

        self.program_counter += 4;
    }

    fn run_to_completion(&mut self) {
        while !self.terminated() {
            self.progress_state();
        }
    }
}

fn main() {
    let file = File::open("./input.txt").expect("Failed to open input file");
    let reader = BufReader::new(file);

    let mut values: Vec<usize> = reader
        .split(b',')
        .map(|el| el.expect("Failed to read bytes from file"))
        .map(|el| String::from_utf8(el).expect("Bytes between a comma weren't UTF8"))
        .map(|el| el.trim().to_string())
        .map(|el| el.parse::<usize>().expect(&format!("Failed to parse {} as u64", el)))
        .collect();

    // Perform the mutations required by the puzzle
    values[1] = 12;
    values[2] = 2;

    let mut program = ProgramState::new(values);
    program.run_to_completion();

    println!("Final value in position 0: {}", program.values[0]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut program = ProgramState::new(vec![1, 0, 0, 0, 99]);
        program.run_to_completion();
        assert_eq!(program.values, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn test_mul() {
        let mut program = ProgramState::new(vec![2, 3, 0, 3, 99]);
        program.run_to_completion();
        assert_eq!(program.values, vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn test_larger() {
        let mut program = ProgramState::new(vec![1,1,1,4,99,5,6,0,99]);
        program.run_to_completion();
        assert_eq!(program.values, vec![30,1,1,4,2,5,6,0,99]);
    }
}