use std::fs::File;
use std::io::{prelude::*, BufReader};

type ProgramElement = isize;

enum ParameterMode {
    Position,
    Immediate,
}

impl From<u8> for ParameterMode {
    fn from(code: u8) -> Self {
        match code {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            code => panic!("Unrecognized parameter mode code: {}", code)
        }
    }
}

struct Parameter {
    mode: ParameterMode,
    contents: ProgramElement,
}

impl Parameter {
    fn read(&self, state: &ProgramState) -> ProgramElement {
        match self.mode {
            ParameterMode::Position => state.values[self.contents as usize],
            ParameterMode::Immediate => self.contents,
        }
    }

    fn write(&self, state: &mut ProgramState, value: ProgramElement) {
        match self.mode {
            ParameterMode::Position => state.values[self.contents as usize] = value,
            ParameterMode::Immediate => panic!("Attempting to write to an immediate mode parameter"),
        }
    }
}

enum OpCode {
    Add,
    Multiply,
    Terminate,
}

impl OpCode {
    fn from_element(element: &ProgramElement) -> Self {
        match element % 100 {
            1 => OpCode::Add,
            2 => OpCode::Multiply,
            99 => OpCode::Terminate,
            code => panic!("Unrecognized opcode: {}", code)
        }
    }

    fn length(&self) -> usize {
        match self {
            OpCode::Add => 4,
            OpCode::Multiply => 4,
            OpCode::Terminate => 1,
        }
    }
}


struct Instruction {
    opcode: OpCode,
    parameters: [Option<Parameter>; 4]
}

impl Instruction {
    fn fetch_and_decode(state: &ProgramState) -> Self {
        let opcode = OpCode::from_element(&state.values[state.program_counter]);

        let mut parameters = [None, None, None, None];
        let mut parameter_modes = state.values[state.program_counter] / 100;

        for i in 1..opcode.length() {
            let mode = ((parameter_modes % 10) as u8).into();
            parameter_modes /= 10;
            let contents = state.values[state.program_counter + i].clone();
            parameters[i - 1] = Some(Parameter {
                mode,
                contents,
            });
        }

        Self {
            opcode,
            parameters,
        }
    }

    fn read_param(&self, idx: usize, state: &ProgramState) -> ProgramElement {
        self.parameters[idx].as_ref().unwrap().read(state)
    }

    fn write_param(&self, idx: usize, state: &mut ProgramState, value: ProgramElement) {
        self.parameters[idx].as_ref().unwrap().write(state, value)
    }

    fn execute(&self, state: &mut ProgramState) {
        match self.opcode {
            OpCode::Add => {
                let a = self.read_param(0, state);
                let b = self.read_param(1, state);
                self.write_param(2, state, a + b);
            }
            OpCode::Multiply => {
                let a = self.read_param(0, state);
                let b = self.read_param(1, state);
                self.write_param(2, state, a * b);
            }
            OpCode::Terminate => state.terminated = true,
        }
    }
}


struct ProgramState {
    values: Vec<ProgramElement>,
    program_counter: usize,
    terminated: bool,
}

impl ProgramState {
    fn new(values: Vec<ProgramElement>) -> Self {
        debug_assert!(values.len() > 0);

        Self {
            values,
            program_counter: 0,
            terminated: false,
        }
    }

    fn progress_state(&mut self) {
        let instr = Instruction::fetch_and_decode(self);
        instr.execute(self);
        self.program_counter += instr.opcode.length();
    }

    fn run_to_completion(&mut self) {
        while !self.terminated {
            self.progress_state();
        }
    }
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
    fn test_nontrivial() {
        let mut program = ProgramState::new(vec![1,1,1,4,99,5,6,0,99]);
        program.run_to_completion();
        assert_eq!(program.values, vec![30,1,1,4,2,5,6,0,99]);
    }
}

fn main() {
    let file = File::open("./input.txt").expect("Failed to open input file");
    let reader = BufReader::new(file);

    let values: Vec<_> = reader
        .split(b',')
        .map(|el| el.expect("Failed to read bytes from file"))
        .map(|el| String::from_utf8(el).expect("Bytes between a comma weren't UTF8"))
        .map(|el| el.trim().to_string())
        .map(|el| el.parse::<ProgramElement>().expect(&format!("Failed to parse {} as u64", el)))
        .collect();

    let target = 19690720;

    'outer: for noun in 0..100 {
        for verb in 0..100 {
            let mut program = ProgramState::new(values.clone());
            program.values[1] = noun;
            program.values[2] = verb;
            program.run_to_completion();

            if program.values[0] == target {
                println!("Found solution: {}", 100 * noun + verb);
                break 'outer;
            }
        }
    }
}