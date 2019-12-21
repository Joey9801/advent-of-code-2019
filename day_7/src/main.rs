use intcode_vm::{ProgramElement, ProgramState};
use permutohedron;

fn test_phase_settings(
    phase_settings: &[ProgramElement],
    program: &ProgramState,
) -> ProgramElement {
    let mut signal = 0;
    for &phase in phase_settings {
        let mut instance = program.clone();
        instance.inputs.push_back(phase);
        instance.inputs.push_back(signal);
        instance.run_to_completion();
        signal = *instance.outputs.first().expect("Amplifier didn't give an output value");
    }

    signal
}

fn main() {
    let program = ProgramState::load_program_file(std::path::Path::new("./input.txt"));

    let mut phases = (0..5).collect::<Vec<isize>>();
    let mut phase_settings = permutohedron::Heap::new(&mut phases);

    let (signal, max_phase_setting) = phase_settings
        .map(|phase_setting| (test_phase_settings(&phase_setting[..], &program), phase_setting))
        .max_by_key(|(signal, _phase_setting)| *signal)
        .unwrap();

    println!("Max signal: {}, phase_settings: {:?}", signal, max_phase_setting);

}