use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::{HashMap, VecDeque};

pub type ProgramElement = isize;

enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

impl From<u8> for ParameterMode {
    fn from(code: u8) -> Self {
        match code {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            2 => ParameterMode::Relative,
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
            ParameterMode::Position => state.mem[self.contents as usize],
            ParameterMode::Immediate => self.contents,
            ParameterMode::Relative => state.mem[(state.relative_base + self.contents) as usize]
        }
    }

    fn write(&self, state: &mut ProgramState, value: ProgramElement) {
        match self.mode {
            ParameterMode::Position => state.mem[self.contents as usize] = value,
            ParameterMode::Immediate => panic!("Attempting to write to an immediate mode parameter"),
            ParameterMode::Relative => state.mem[(state.relative_base + self.contents) as usize] = value,
        }
    }
}

enum OpCode {
    Add,
    Multiply,
    ReadInput,
    WriteOutput,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    AdjustRelativeBase,
    Terminate,
}

impl OpCode {
    fn from_element(element: &ProgramElement) -> Self {
        match element % 100 {
            1 => OpCode::Add,
            2 => OpCode::Multiply,
            3 => OpCode::ReadInput,
            4 => OpCode::WriteOutput,
            5 => OpCode::JumpIfTrue,
            6 => OpCode::JumpIfFalse,
            7 => OpCode::LessThan,
            8 => OpCode::Equals,
            9 => OpCode::AdjustRelativeBase,
            99 => OpCode::Terminate,
            code => panic!("Unrecognized opcode: {}", code)
        }
    }

    fn length(&self) -> usize {
        match self {
            OpCode::Add => 4,
            OpCode::Multiply => 4,
            OpCode::ReadInput => 2,
            OpCode::WriteOutput => 2,
            OpCode::JumpIfTrue => 3,
            OpCode::JumpIfFalse => 3,
            OpCode::LessThan => 4,
            OpCode::Equals => 4,
            OpCode::AdjustRelativeBase => 2,
            OpCode::Terminate => 1,
        }
    }
}

#[derive(Debug)]
pub enum ExecuteError {
    NoInput
}

struct Instruction {
    opcode: OpCode,
    parameters: [Option<Parameter>; 4]
}

impl Instruction {
    fn fetch_and_decode(state: &ProgramState) -> Self {
        let opcode = OpCode::from_element(&state.mem[state.program_counter]);

        let mut parameters = [None, None, None, None];
        let mut parameter_modes = state.mem[state.program_counter] / 100;

        for i in 1..opcode.length() {
            let mode = ((parameter_modes % 10) as u8).into();
            parameter_modes /= 10;
            let contents = state.mem[state.program_counter + i].clone();
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

    fn execute(&self, state: &mut ProgramState) -> Result<(), ExecuteError> {
        let mut jumped = false;
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
            OpCode::ReadInput => {
                let input = state.inputs
                    .pop_front()
                    .ok_or(ExecuteError::NoInput)?;

                self.write_param(0, state, input);
            }
            OpCode::WriteOutput => state.outputs.push(self.read_param(0, state)),
            OpCode::JumpIfTrue => {
                let test = self.read_param(0, state);
                if test != 0 {
                    let target = self.read_param(1, state) as usize;
                    state.program_counter = target;
                    jumped = true;
                }
            }
            OpCode::JumpIfFalse => {
                let test = self.read_param(0, state);
                if test == 0 {
                    let target = self.read_param(1, state) as usize;
                    state.program_counter = target;
                    jumped = true;
                }
            }
            OpCode::LessThan => {
                let a = self.read_param(0, state);
                let b = self.read_param(1, state);
                self.write_param(2, state, if a < b { 1 } else { 0 });
            }
            OpCode::Equals => {
                let a = self.read_param(0, state);
                let b = self.read_param(1, state);
                self.write_param(2, state, if a == b { 1 } else { 0 });
            }
            OpCode::AdjustRelativeBase => state.relative_base += self.read_param(0, state),
            OpCode::Terminate => state.terminated = true,
        }

        if !jumped {
            state.program_counter += self.opcode.length();
        }

        Ok(())
    }
}

const PAGE_SIZE: usize = 1024;

struct PagedMemory<T: Default + Copy> {
    /// Maps page index to storage for that page, where page index is floor(addr / PAGE_SIZE)
    pages: HashMap<usize, [T; PAGE_SIZE]>,
}

impl<T: Default + Copy> PagedMemory<T> {
    pub fn new() -> Self {
        PagedMemory {
            pages: HashMap::new(),
        }
    }

    pub fn read_addr(&self, addr: usize) -> T {
        let index = addr / PAGE_SIZE;
        let offset = addr % PAGE_SIZE;
        match self.pages.get(&index) {
            Some(page) => page[offset],
            None => T::default(),
        }
    }

    pub fn write_addr(&mut self, addr: usize, value: T) {
        let index = addr / PAGE_SIZE;
        let offset = addr % PAGE_SIZE;

        let page = self.pages.entry(index).or_insert([T::default(); PAGE_SIZE]);
        page[offset] = value;
    }
}


#[derive(Clone)]
pub struct ProgramState {
    pub mem: Vec<ProgramElement>,
    pub inputs: VecDeque<ProgramElement>,
    pub outputs: Vec<ProgramElement>,
    pub program_counter: usize,
    pub relative_base: ProgramElement,
    pub terminated: bool,
}

impl ProgramState {
    /// Loads a comma-separated program source file, leaves the input queue empty.
    pub fn load_program_file(path: &std::path::Path) -> Self {
        let file = File::open(path).expect("Failed to open program source");
        let reader = BufReader::new(file);

        let initial_mem: Vec<_> = reader
            .split(b',')
            .map(|el| el.expect("Failed to read bytes from file"))
            .map(|el| String::from_utf8(el).expect("Bytes between a comma weren't UTF8"))
            .map(|el| el.trim().to_string())
            .map(|el| el.parse::<ProgramElement>().expect(&format!("Failed to parse {} as u64", el)))
            .collect();

        Self {
            mem: initial_mem,
            inputs: Vec::new().into(),
            outputs: Vec::new(),
            program_counter: 0,
            relative_base: 0,
            terminated: false,
        }
    }

    pub fn new(mem: Vec<ProgramElement>, inputs: VecDeque<ProgramElement>) -> Self {
        debug_assert!(mem.len() > 0);

        Self {
            mem,
            inputs,
            outputs: Vec::new(),
            program_counter: 0,
            relative_base: 0,
            terminated: false,
        }
    }

    pub fn progress_state(&mut self) -> Result<(), ExecuteError> {
        let instr = Instruction::fetch_and_decode(self);
        instr.execute(self)
    }

    pub fn run_to_next_input(&mut self) {
        while !self.terminated {
            match self.progress_state() {
                Ok(()) => (),
                Err(ExecuteError::NoInput) => break,
            }
        }
    }

    pub fn run_to_completion(&mut self) {
        while !self.terminated {
            self.progress_state().expect("Hit execution error while running to completion");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paged_memory() {
        let mut mem = PagedMemory::<i32>::new();
        assert_eq!(mem.read_addr(1234 as usize), 0);
        mem.write_addr(1234 as usize, 42);
        assert_eq!(mem.read_addr(1234 as usize), 42);
    }

    #[test]
    fn test_add() {
        let mut program = ProgramState::new(vec![1, 0, 0, 0, 99], VecDeque::new());
        program.run_to_completion();
        assert_eq!(program.mem, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn test_mul() {
        let mut program = ProgramState::new(vec![2, 3, 0, 3, 99], VecDeque::new());
        program.run_to_completion();
        assert_eq!(program.mem, vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn test_nontrivial() {
        let mut program = ProgramState::new(vec![1,1,1,4,99,5,6,0,99], VecDeque::new());
        program.run_to_completion();
        assert_eq!(program.mem, vec![30,1,1,4,2,5,6,0,99]);
    }

    #[test]
    fn test_jump_if_true() {
        fn run(input: ProgramElement) -> ProgramElement {
            let mut inputs = VecDeque::new();
            inputs.push_back(input);
            // Problem statement claims that this program outputs 0 if the input is 0, or
            // 1 if it was non-zero
            let mut program = ProgramState::new(
                vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9],
                inputs
            );
            program.run_to_completion();
            program.outputs[0]
        }

        assert_eq!(run(0), 0);
        assert_eq!(run(4), 1);
    }
}
