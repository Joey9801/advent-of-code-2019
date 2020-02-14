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

enum GameMessage {
    BlockUpdate {
        pos: Vec2,
        contents: CellContents,
    },
    ScoreUpdate(i32),
}

impl From<(ProgramElement, ProgramElement, ProgramElement)> for GameMessage {
    fn from(nums: (ProgramElement, ProgramElement, ProgramElement)) -> Self {
        let x = nums.0 as i32;
        let y = nums.1 as i32;

        if x == -1 && y == 0 {
            GameMessage::ScoreUpdate(nums.2 as i32)
        } else {
            let contents = nums.2.into();
            GameMessage::BlockUpdate {
                pos: Vec2 {
                    x, y
                },
                contents,
            }
        }
    }
}

#[derive(Clone)]
struct Game {
    board: HashMap<Vec2, CellContents>,

    // Both ball and paddle only occupy a single cell each frame
    // Option<Vec2>, since the controller could write the old position as empty before writing the new location.
    ball_pos: Option<Vec2>,
    paddle_pos: Option<Vec2>,

    score: Option<i32>,
    controller: ProgramState,
}

impl Game {
    fn new() -> Self {
        let board = HashMap::new();
        let mut controller = ProgramState::load_program_file(
            std::path::Path::new("./input.txt")
        );

        // From part 2 instructions
        controller.mem.write_addr(0, 2);

        let mut new_game = Self {
            board,
            score: None,
            ball_pos: None,
            paddle_pos: None,
            controller,
        };

        
        // Load the initial board (no inputs given)
        new_game.step(None);

        new_game
    }

    fn process_msg(&mut self, msg: GameMessage) {
        match msg {
            GameMessage::BlockUpdate {pos, contents} => {
                match contents {
                    CellContents::Empty => {
                        self.board.remove(&pos);

                        if Some(pos) == self.ball_pos{
                            self.ball_pos = None;
                        }

                        if Some(pos) == self.paddle_pos {
                            self.paddle_pos = None;
                        }
                    },
                    CellContents::Ball => self.ball_pos = Some(pos),
                    CellContents::Paddle => self.paddle_pos = Some(pos),
                    _ => { self.board.insert(pos, contents); },
                };
            }
            GameMessage::ScoreUpdate(score) => self.score = Some(score),
        }
    }

    fn ball(&self) -> Vec2 {
        self.ball_pos.expect("Expect to have a ball position")
    }

    fn paddle(&self) -> Vec2 {
        self.paddle_pos.expect("Expect to have a paddle position")
    }

    fn finished(&self) -> bool {
        self.controller.terminated ||
            self.ball().y > self.paddle().y ||
            self.block_count() == 0
    }

    fn block_count(&self) -> usize {
        self.board
            .values()
            .filter(|v| **v == CellContents::Block)
            .count()
    }

    fn step(&mut self, paddle_input: Option<ProgramElement>) {
        if let Some(input) = paddle_input {
            self.controller.inputs.push_back(input);
        }

        self.controller.run_to_next_input();

        while self.controller.outputs.len() >= 3 {
            let msg_nums = (
                self.controller.outputs.pop_front().unwrap(),
                self.controller.outputs.pop_front().unwrap(),
                self.controller.outputs.pop_front().unwrap(),
            );
            self.process_msg(msg_nums.into());
        }
    }

    fn win_game(&mut self) {
        while !self.finished() {
            let input = (self.ball().x - self.paddle().x).signum();
            self.step(Some(input as ProgramElement));
            // println!("Ball {}, Paddle {}, Score {}, blocks {}",
            //     self.ball(), self.paddle(), self.score.unwrap(), self.block_count());
        }
    }
}

fn main() {
    let mut game = Game::new();
    game.win_game();
    dbg!(&game.score);
}
