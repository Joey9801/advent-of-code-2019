use intcode_vm::{ProgramElement, ProgramState};
use permutohedron;

fn test_phase_settings(
    phase_settings: &[ProgramElement],
    program: &ProgramState,
) -> ProgramElement {
    let mut amps = Vec::new();
    for phase_setting in phase_settings {
        let mut amp = program.clone();
        amp.inputs.push_back(*phase_setting);
        amps.push(amp);
    }

    let mut signal = 0;
    let mut idx = 0;
    while !amps.last().unwrap().terminated {
        amps[idx].inputs.push_back(signal);
        amps[idx].run_to_next_input();
        signal = *amps[idx].outputs.back().unwrap();

        idx = (idx + 1) % amps.len();
    }

    signal
}

fn main() {
    let program = ProgramState::load_program_file(std::path::Path::new("./input.txt"));

    let mut phases = (5..10).collect::<Vec<isize>>();
    let phase_settings = permutohedron::Heap::new(&mut phases);

    let (signal, max_phase_setting) = phase_settings
        .map(|phase_setting| (test_phase_settings(&phase_setting[..], &program), phase_setting))
        .max_by_key(|(signal, _phase_setting)| *signal)
        .unwrap();

    println!("Max signal: {}, phase_settings: {:?}", signal, max_phase_setting);

}