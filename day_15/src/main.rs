use std::path::Path;

use intcode_vm::{ProgramState};
use util::geometry::{CardDir, Rotation};
use util::vec2::Vec2;

#[derive(PartialEq, Eq)]
enum RobotResponse {
    Moved,
    HitWall,
    FoundOxygen,
}

struct Robot { 
    controller: ProgramState,
}

impl Robot {
    fn new() -> Self {
        let controller = ProgramState::load_program_file(Path::new("./input.txt"));

        Self {
            controller
        }
    }

    fn explore(&mut self, direction: CardDir) -> RobotResponse {
        let input = match direction {
            CardDir::Up => 1,
            CardDir::Down => 2,
            CardDir::Left => 3,
            CardDir::Right => 4,
        };
        
        self.controller.inputs.push_back(input);
        self.controller.run_to_next_input();

        let output = self.controller.outputs.pop_front()
            .expect("Robot gave no response to movement command");

        match output {
            0 => RobotResponse::HitWall,
            1 => RobotResponse::Moved,
            2 => RobotResponse::FoundOxygen,
            _ => panic!("Robot returned unrecognized output code: {}", output),
        }
    }
}

#[derive(Debug)]
struct DfsStackElement {
    position: Vec2,
    from_dir: Option<CardDir>,
    last_search_dir: Option<CardDir>,
}

fn part_1() -> usize {
    let mut robot = Robot::new();
    let mut dfs_stack = Vec::new();
    let mut shortest_path_to_oxygen = None;

    dfs_stack.push(DfsStackElement {
        position: Vec2::new(0, 0),
        from_dir: None,
        last_search_dir: None,
    });

    loop {
        let search_dir = {
            let head = dfs_stack.last().unwrap();
            match head.last_search_dir {
                Some(dir) => dir.turn(Rotation::Clockwise),
                None => match head.from_dir {
                    Some(dir) => dir.turn(Rotation::Clockwise),
                    None => CardDir::Up,
                },
            }
        };

        // If this search repeats the very first search, break as there is no more searching to do
        if dfs_stack.len() == 1 &&
            search_dir == CardDir::Up &&
            dfs_stack.last().unwrap().last_search_dir.is_some() {
                break;
        }

        let explore_result = robot.explore(search_dir);

        let mut head = dfs_stack.last_mut().unwrap();
        head.last_search_dir = Some(search_dir);

        match explore_result {
            RobotResponse::HitWall => (),
            RobotResponse::Moved | RobotResponse::FoundOxygen => {
                if Some(search_dir) == head.from_dir {
                    dfs_stack.pop();
                } else {
                    let new_stage = DfsStackElement {
                        position: head.position + search_dir.vec(),
                        from_dir: Some(search_dir.opposite()),
                        last_search_dir: None,
                    };
                    dfs_stack.push(new_stage);
                }
            },
        }

        if explore_result == RobotResponse::FoundOxygen {
            shortest_path_to_oxygen = Some(match shortest_path_to_oxygen {
                Some(d) => std::cmp::min(d, dfs_stack.len() - 1),
                None => dfs_stack.len() - 1,
            });
        }
    }

    shortest_path_to_oxygen.expect("Didn't find any path to oxygen")
}

fn main() {
    dbg!(part_1());
}