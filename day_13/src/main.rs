use std::collections::HashMap;

use intcode_vm::{ProgramState, ProgramElement};
use util::vec2::Vec2;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CellContents {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl From<ProgramElement> for CellContents {
    fn from(num: ProgramElement) -> Self {
        match num {
            0 => Self::Empty,
            1 => Self::Wall,
            2 => Self::Block,
            3 => Self::Paddle,
            4 => Self::Ball,
            _ => panic!("Unrecognized cell type number: {}", num),
        }
    }
}

struct GameMessage {
    pos: Vec2,
    contents: CellContents,
}

impl From<(ProgramElement, ProgramElement, ProgramElement)> for GameMessage {
    fn from(nums: (ProgramElement, ProgramElement, ProgramElement)) -> Self {
        let x = nums.0 as i32;
        let y = nums.1 as i32;
        let contents = nums.2.into();

        Self {
            pos: Vec2 {
                x, y
            },
            contents,
        }
    }
}

struct Game {
    board: HashMap<Vec2, CellContents>,
    controller: ProgramState,
}

impl Game {
    fn new() -> Self {
        let board = HashMap::new();
        let controller = ProgramState::load_program_file(
            std::path::Path::new("./input.txt"));

        Self {
            board,
            controller,
        }
    }

    fn process_msg(&mut self, msg: GameMessage) {
        self.board.insert(msg.pos, msg.contents);
    }

    fn run_to_completion(&mut self) {
        self.controller.run_to_completion();
        while self.controller.outputs.len() >= 3 {
            let msg_nums = (
                self.controller.outputs.pop_front().unwrap(),
                self.controller.outputs.pop_front().unwrap(),
                self.controller.outputs.pop_front().unwrap(),
            );
            self.process_msg(msg_nums.into());
        }
    }
}

fn main() {
    let mut game = Game::new();
    game.run_to_completion();

    let block_count: usize = game.board
        .values()
        .filter(|v| **v == CellContents::Block)
        .map(|_| 1)
        .sum();

    dbg!(block_count);
}
