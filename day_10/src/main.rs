use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::collections::HashSet;


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

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
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

    let mut best_loc = (None, 0);
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

        if score > best_loc.1 {
            best_loc = (Some(*root), score);
        }
    }


    dbg!(&best_loc);
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