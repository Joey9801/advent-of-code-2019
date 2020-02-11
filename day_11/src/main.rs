use std::collections::HashSet;


enum Rotation {
    Clockwise,
    CounterClockwise,
}

#[derive(Clone, Copy, Debug)]
enum CardDir {
    Up,
    Down,
    Left,
    Right,
}

impl CardDir {
    fn turn(self, rot: Rotation) -> Self {
        let dirnum: i32 = match &self {
            CardDir::Up => 0,
            CardDir::Left => 1,
            CardDir::Down => 2,
            CardDir::Right => 3,
        };
        let rotnum: i32 = match rot {
            Rotation::Clockwise => 1,
            Rotation::CounterClockwise => -1,
        };

        match (dirnum + rotnum).rem_euclid(4) {
            0 => CardDir::Up,
            1 => CardDir::Left,
            2 => CardDir::Down,
            3 => CardDir::Right,
            wat => unreachable!("i32.rem_euclid(4) returned {}, which isn't in {{0, 1, 2, 3}}", wat),
        }
    }
}

#[derive(Debug)]
enum Color {
    Black,
    White,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn advance(self, dir: CardDir) -> Self {
        let (x, y) = match dir {
            CardDir::Up    => (self.x, self.y + 1),
            CardDir::Down  => (self.x, self.y - 1),
            CardDir::Left  => (self.x + 1, self.y),
            CardDir::Right => (self.x - 1, self.y),
        };

        Self {
            x, y
        }
    }
}

#[derive(Debug)]
struct Board {
    white_cells: HashSet<Coord>,
    painted_ever: HashSet<Coord>,
}

impl Board {
    fn new() -> Self {
        // Board starts out entirely black
        Self {
            white_cells: HashSet::new(),
            painted_ever: HashSet::new(),
        }
    }

    fn get_color_of(&self, coord: Coord) -> Color {
        if self.white_cells.contains(&coord) {
            Color::White
        } else {
            Color::Black
        }
    }

    fn set_color_of(&mut self, coord: Coord, color: Color) {
        self.painted_ever.insert(coord);

        match color {
            Color::White => self.white_cells.insert(coord),
            Color::Black => self.white_cells.remove(&coord),
        };
    }
}

#[derive(Debug)]
struct Robot {
    pos: Coord,
    dir: CardDir,
    board: Board,
    controller: intcode_vm::ProgramState,
}

impl Robot {
    fn new() -> Self {
        let pos = Coord { x: 0, y: 0 };
        let dir = CardDir::Up;
        let board = Board::new();
        let controller = intcode_vm::ProgramState::load_program_file(
            std::path::Path::new("./input.txt")
        );

        Self {
            pos,
            dir,
            board,
            controller,
        }
    }

    fn is_done(&self) -> bool {
        self.controller.terminated
    }

    fn step(&mut self) {
        let sensor_reading = match self.board.get_color_of(self.pos) {
            Color::White => 1,
            Color::Black => 0,
        };

        self.controller.inputs.push_back(sensor_reading);
        self.controller.run_to_next_input();
        let color_command = self.controller.outputs.pop_front();
        let movement_command = self.controller.outputs.pop_front();

        match color_command {
            Some(0) => self.board.set_color_of(self.pos, Color::Black),
            Some(1) => self.board.set_color_of(self.pos, Color::White),
            Some(other) => panic!("Unrecognized color painting command code: {}", other),
            None => (),
        }

        match movement_command {
            Some(0) => {
                self.dir = self.dir.turn(Rotation::CounterClockwise);
                self.pos = self.pos.advance(self.dir);
            },
            Some(1) => {
                self.dir = self.dir.turn(Rotation::Clockwise);
                self.pos = self.pos.advance(self.dir);
            },
            Some(wat) => panic!("Unrecognized movement command code: {}", wat),
            None => (),
        }
    }
}

fn main() {
    let mut robot = Robot::new();
    while !robot.is_done() {
        robot.step();
    }

    dbg!(robot.board.painted_ever.len());
}