use intcode_vm::ProgramState;

fn main() {
    let source_path = std::path::Path::new("./input.txt");
    let mut program = ProgramState::load_program_file(&source_path);
    program.inputs = vec![5].into();
    program.run_to_completion();
    println!("Program outputs = {:?}", program.outputs);
}