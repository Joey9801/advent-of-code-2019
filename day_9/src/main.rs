use intcode_vm::ProgramState;

fn main() {
    let mut program = ProgramState::load_program_file(std::path::Path::new("./input.txt"));
    program.inputs.push_back(2);
    program.run_to_completion();
    dbg!(&program.outputs);
}
