//! Define the coordinate system to be one where (1, 1) is a vector pointing up and right.

use std::fs::File;
use std::io::{prelude::*, BufReader};


#[derive(Clone, Copy, Debug)]
struct Vector2 {
    x: i64,
    y: i64,
}

impl Vector2 {
    fn new(x: i64, y: i64) -> Self {
        Self {
            x,
            y
        }
    }

    fn l1_norm(&self) -> u64 {
        self.x.abs() as u64 + self.y.abs() as u64
    }
}

impl std::ops::Sub for Vector2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::AddAssign for Vector2 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug)]
struct Line {
    /// The starting coordinate of this line
    origin: Vector2,

    /// The vector from the start of this wire to its terminus
    ///
    /// Expect that this is zero in precisely one of (x, y)
    span: Vector2,
}

impl Line {
    fn contains_point(&self, point: Vector2) -> bool {
        let in_x = match self.span.x.signum() {
            0 => point.x == self.origin.x,
            1 => point.x > self.origin.x && point.x < (self.origin.x + self.span.x),
            -1 => point.x < self.origin.x && point.x > (self.origin.x + self.span.x),
            _ => unreachable!(),
        };

        let in_y = match self.span.y.signum() {
            0 => point.y == self.origin.y,
            1 => point.y > self.origin.y && point.y < (self.origin.y + self.span.y),
            -1 => point.y < self.origin.y && point.y > (self.origin.y + self.span.y),
            _ => unreachable!(),
        };

        in_x && in_y
    }

    /// The distance from the origin of this line to the given point
    fn distance_to(&self, point: Vector2) -> u64 {
        (self.origin - point).l1_norm()
    }

    /// If this line intersects with the other, the point at which they intersect
    fn intersection_point(&self, other: &Line) -> Option<Vector2> {
        if self.span.x == 0 && other.span.x == 0 {
            // The lines are parallel -> they don't intersect
            return None;
        }

        // The point where the lines would intersect if they were infinitely long
        // Only valid because each line is axis aligned.
        let point = Vector2 {
            x: self.origin.x * self.span.y.abs().signum() + other.origin.x * other.span.y.abs().signum(),
            y: self.origin.y * self.span.x.abs().signum() + other.origin.y * other.span.x.abs().signum(),
        };

        if self.contains_point(point) && other.contains_point(point) {
            Some(point)
        } else {
            None
        }
    }
}

#[derive(Clone, Copy)]
struct WireNode {
    point: Vector2,
    length_before: u64,
}

/// Represents a wire made up multiple line segments
struct Wire {
    /// The line segments in this wire go between the nodes.
    ///
    /// For AoC day 2, part 1, the first node should be (0, 0)
    nodes: Vec<WireNode>,
}

impl Wire {
    fn from_puzzle_input(input: &str) -> Self {
        assert!(input.is_ascii());

        let mut cursor = Vector2::new(0, 0);
        let mut nodes = vec![WireNode {
                point: cursor.clone(),
                length_before: 0,
        }];

        let mut total_len: u64 = 0;
        for instr in input.split(",") {
            let dir = &instr[0..1];
            let len: i64  = instr[1..]
                .parse()
                .expect(&format!("\"{}\" wasn't a valid instruction", instr));

            match dir {
                "U" => cursor += Vector2::new(0, len),
                "D" => cursor += Vector2::new(0, -len),
                "L" => cursor += Vector2::new(-len, 0),
                "R" => cursor += Vector2::new(len, 0),
                other => panic!(format!("Unknown direction '{}'", other)),
            }

            total_len += len.abs() as u64;

            nodes.push(WireNode {
                point: cursor.clone(),
                length_before: total_len,
            });
        }

        Self {
            nodes
        }
    }

    fn iter_lines<'a>(&'a self) -> impl Iterator<Item = (Line, u64)> + 'a {
        self.nodes
            .windows(2)
            .map(|parts| (
                    Line {
                    origin: parts[0].point,
                    span: parts[1].point - parts[0].point,
                },
                parts[0].length_before,
            ))
    }

    fn iter_lengths<'a>(&'a self) -> impl Iterator<Item = u64> + 'a {
        self.nodes.iter().map(|n| n.length_before)
    }
}

fn main()  {
    let file = File::open("./input.txt").expect("Failed to open input.txt");
    let reader = BufReader::new(file);

    let mut wires = reader.lines()
        .map(|l| l.expect("Failed to read line"))
        .map(|l| Wire::from_puzzle_input(&l.trim()));

    let a = wires.next().expect("Expected exactly two wires");
    let b = wires.next().expect("Expected exactly two wires");

    let min_intersection = a.iter_lines().filter_map(|(a_line, a_base_length)| {
            b.iter_lines().filter_map(|(b_line, b_base_length)| {
                a_line.intersection_point(&b_line).map(|point|
                    a_base_length + a_line.distance_to(point) +
                    b_base_length + b_line.distance_to(point)
                )
            })
            .min()
    }).min();

    println!("Minimum intersection: {:?}", min_intersection);
}
