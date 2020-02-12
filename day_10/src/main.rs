#![feature(slice_partition_dedup)]

use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::collections::HashSet;

use util::math::gcd;


enum CellContents {
    Empty,
    Asteroid,
}

impl CellContents {
    fn from_char(c: char) -> Self {
        match c {
            '.' => CellContents::Empty,
            '#' => CellContents::Asteroid,
            other => panic!("Unrecognized asteroid map char: {}", other),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

impl std::ops::Sub for Coord {
    type Output = Coord;

    fn sub(self, other: Coord) -> Self::Output {
        Coord {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Coord {
    fn new(x: i32, y: i32) -> Self {
        Self {
            x, y
        }
    }

    /// For a Coord of the form {N*x, N*y}, returns the tuple ({x, y}, N) where N >= 0.
    fn simplify(self) -> (Self, i32) {
        let n = gcd(self.y, self.x).abs();

        if n == 0 {
            (Coord {
                x: 0,
                y: 0,
            }, 0)
        } else {
            (Coord {
                x: self.x / n,
                y: self.y / n,
            }, n)
        }
    }

    /// Clockwise angle in radians from straight up.
    fn angle(&self) -> f32 {
        // atan2 returns from the range [-pi, +pi] radians from (1, 0)
        // Additionally, the y coordinate in the puzzle is backwards, ie, +ve y is down.
        let raw = (-self.y as f32).atan2(self.x as f32);
        let against_vertical = std::f32::consts::FRAC_PI_2 - raw;

        // Normalize the angle to the range [0, 2*pi]
        let two_pi = 2f32 * std::f32::consts::PI;
        let normalized = (against_vertical + two_pi).rem_euclid(two_pi);

        normalized
    }
}

struct AsteroidField {
    locs: Vec<Coord>,
}

impl AsteroidField {
    fn load_from_str(data: &str) -> Self {
        let mut locs = Vec::new();
        for (y, row_str) in data.lines().enumerate() {
            for (x, c) in row_str.chars().enumerate() {
                match CellContents::from_char(c) {
                    CellContents::Empty => (),
                    CellContents::Asteroid => locs.push(Coord::new(x as i32, y as i32)),
                }
            }
        }

        Self {
            locs: locs,
        }
    }

    fn load_from_file(path: &Path) -> Self {
        let mut file = File::open(path)
            .expect("Failed to open asteroid field file");

        let mut data = String::new();
        file.read_to_string(&mut data)
            .expect("Failed to read asteroid field file");

        Self::load_from_str(&data)
    }
}

fn main() {
    let field = AsteroidField::load_from_file(Path::new("./input.txt"));

    let mut best: Option<(Coord, usize)> = None;
    for root in field.locs.iter() {
        let score = field.locs
            .iter()
            .filter(|other| *other != root)
            .map(|other| {
                let (base, _n) = (*other - *root).simplify();
                base
            })
            .collect::<HashSet<_>>()
            .len();

        match best {
            Some((_, curr_best_score)) if curr_best_score > score => (),
            _ => best = Some((*root, score)),
        }
    }

    dbg!(&best);
    let station_loc = best.unwrap().0;

    let mut targets = field.locs
        .iter()
        .filter(|target| **target != station_loc)
        .map(|target| {
            let (base, n) = (*target - station_loc).simplify();
            (target, base, n)
        })
        .collect::<Vec<_>>();

    targets.sort_by_key(|(_, _, n)| *n);
    targets.sort_by(|(_, a, _), (_, b, _)| a.angle().partial_cmp(&b.angle()).unwrap());
    loop {
        let (uniques, duplicates) = targets.partition_dedup_by_key(|(_, a, _)| *a);

        if  duplicates.len() == 0 ||
            duplicates.iter().all(|(_, base, _)| *base == uniques.last().unwrap().1)
        {
            break;
        }
    }

    assert!(targets.len() >= 200);
    dbg!(&targets[199]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coord_simplify_positive() {
        let c = Coord::new(4, 6);
        let (simplified, n) = c.simplify();
        assert_eq!(simplified, Coord::new(2, 3));
        assert_eq!(n, 2);
    }

    #[test]
    fn test_coord_simplify_negative() {
        let c = Coord::new(-10, -20);
        let (simplified, n) = c.simplify();
        assert_eq!(simplified, Coord::new(-1, -2));
        assert_eq!(n, 10);
    }

    #[test]
    fn test_coord_simplify_mixed_1() {
        let c = Coord::new(5, -15);
        let (simplified, n) = c.simplify();
        assert_eq!(simplified, Coord::new(1, -3));
        assert_eq!(n, 5);
    }

    #[test]
    fn test_coord_simplify_mixed_2() {
        let c = Coord::new(-5, 15);
        let (simplified, n) = c.simplify();
        assert_eq!(simplified, Coord::new(-1, 3));
        assert_eq!(n, 5);
    }

    #[test]
    fn test_coord_simplify_zero_x() {
        let c = Coord::new(0, 5);
        let (simplified, n) = c.simplify();
        assert_eq!(simplified, Coord::new(0, 1));
        assert_eq!(n, 5);

        let c = Coord::new(0, -5);
        let (simplified, n) = c.simplify();
        assert_eq!(simplified, Coord::new(0, -1));
        assert_eq!(n, 5);
    }

    #[test]
    fn test_coord_simplify_zero_y() {
        let c = Coord::new(5, 0);
        let (simplified, n) = c.simplify();
        assert_eq!(simplified, Coord::new(1, 0));
        assert_eq!(n, 5);

        let c = Coord::new(-5, 0);
        let (simplified, n) = c.simplify();
        assert_eq!(simplified, Coord::new(-1, 0));
        assert_eq!(n, 5);
    }
}