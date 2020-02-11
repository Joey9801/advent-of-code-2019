mod vec3;

use vec3::Vec3;

#[derive(Clone)]
struct Moon {
    pos: Vec3,
    vel: Vec3,
}

impl Moon {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self {
            pos: Vec3::new(x, y, z),
            vel: Vec3::new(0, 0, 0),
        }
    }

    fn energy(&self) -> i32 {
        self.pos.l1_norm() * self.vel.l1_norm()
    }
}

impl std::fmt::Display for Moon {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "pos = {:^13} vel = {:^13}", self.pos, self.vel)
    }
}

struct System {
    moons: Vec<Moon>,
}

impl System {
    fn new() -> Self {
        Self {
            moons: Vec::new(),
        }
    }

    // Example 1 from the problem statement
    //     <x=-1, y=0, z=2>
    //     <x=2, y=-10, z=-7>
    //     <x=4, y=-8, z=8>
    //     <x=3, y=5, z=-1>
    fn example_1() -> Self {
        let mut s = Self::new();

        s.moons.push(Moon::new(-1, 0, 2));
        s.moons.push(Moon::new(2, -10, -7));
        s.moons.push(Moon::new(4, -8, 8));
        s.moons.push(Moon::new(3, 5, -1));

        s
    }

    /// Real puzzle input, from ./input.txt
    ///     <x=-2, y=9, z=-5>
    ///     <x=16, y=19, z=9>
    ///     <x=0, y=3, z=6>
    ///     <x=11, y=0, z=11>
    fn puzzle_input() -> Self {
        let mut s = Self::new();

        s.moons.push(Moon::new(-2, 9, -5));
        s.moons.push(Moon::new(16, 19, 9));
        s.moons.push(Moon::new(0, 3, 6));
        s.moons.push(Moon::new(11, 0, 11));

        s
    }

    fn step(&mut self) {
        for a in 0..self.moons.len() {
            for b in (a + 1)..self.moons.len() {
                let force = (self.moons[b].pos - self.moons[a].pos).signum();
                self.moons[a].vel += force;
                self.moons[b].vel -= force;
            }
        }

        for moon in self.moons.iter_mut() {
            moon.pos += moon.vel;
        }
    }

    fn energy(&self) -> i32 {
        self.moons.iter()
            .map(|m| m.energy())
            .sum()
    }
}

fn main() {
    let mut system = System::puzzle_input();
    for _step in 0..1000 {
        system.step();
    }

    println!("Step 1000, energy = {}", system.energy());
}