use crate::vec2::Vec2;

pub enum Rotation {
    Clockwise,
    CounterClockwise,
}

/// Represents one of the four cardinal directions
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CardDir {
    Up,
    Down,
    Left,
    Right,
}

impl CardDir {
    pub fn turn(&self, rot: Rotation) -> Self {
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

    pub fn opposite(&self) -> Self {
        match self {
            CardDir::Up => CardDir::Down,
            CardDir::Down => CardDir::Up,
            CardDir::Right => CardDir::Left,
            CardDir::Left => CardDir::Right,
        }
    }

    pub fn vec(self) -> Vec2 {
        match self {
            CardDir::Up => Vec2::new(0, 1),
            CardDir::Down => Vec2::new(0, -1),
            CardDir::Right => Vec2::new(1, 0),
            CardDir::Left => Vec2::new(-1, 0),
        }
    }
}