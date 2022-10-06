use cartesian::*;
use rand::{distributions::Standard, prelude::Distribution};

type Coord = i8;
type Pos = [Coord; 3];

type Axis = u8;
type Polarity = bool;
const POS: Polarity = true;
const NEG: Polarity = false;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Face {
    axis: Axis,
    pol: Polarity,
}

#[inline]
fn third_axis(x: u8, y: u8) -> u8 {
    x ^ y ^ 3
}

#[test]
fn axis_test() {
    for (x, y) in cartesian!(0..3, 0..3) {
        if x == y {
            continue;
        }
        let z = third_axis(x, y);
        assert!(z != x);
        assert!(z != y);
        assert!(z < 3);
    }
}

impl Face {
    fn rotate(self, face: Face, clockwise: bool) -> Self {
        if self.axis == face.axis {
            self
        } else {
            let axis = third_axis(self.axis, face.axis);
            let pol = self.pol ^ face.pol ^ clockwise; // I hope this is somewhat correct TODO
            Self { axis, pol }
        }
    }
}

impl Distribution<Face> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Face {
        Face {
            axis: rng.gen_range(0..3),
            pol: rng.gen(),
        }
    }
}

trait Trig {
    fn sin(&self) -> i8;
    fn cos(&self) -> i8;
}

impl Trig for Polarity {
    fn sin(&self) -> i8 {
        match self {
            true => 1,
            false => -1,
        }
    }
    fn cos(&self) -> i8 {
        0
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Ori {
    white: Face,
    blue: Face,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Cubie {
    pos: Pos,
    rot: Ori,
}

impl Cubie {
    fn rotate(self, face: Face, clockwise: bool) -> Self {
        if self.pos[face.axis as usize] == face.pol.sin() {
            let sin = clockwise.sin();
            let x = ((face.axis + 1) % 3) as usize;
            let y = ((face.axis + 2) % 3) as usize;

            let mut pos = self.pos;
            pos[x] = -self.pos[y] * sin;
            pos[y] = self.pos[x] * sin;

            let rot = Ori {
                white: self.rot.white.rotate(face, clockwise),
                blue: self.rot.blue.rotate(face, clockwise),
            };

            Self { pos, rot }
        } else {
            self
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Cube {
    cubies: Vec<Cubie>,
}

impl Cube {
    fn new() -> Self {
        let mut cubies = vec![];
        for (x, y, z) in cartesian!(-1..=1, -1..=1, -1..=1) {
            let pos = [x, y, z];
            let rot = Ori {
                white: Face { axis: 0, pol: POS },
                blue: Face { axis: 1, pol: POS },
            };
            let baby = Cubie { pos, rot };
            cubies.push(baby);
        }
        Self { cubies }
    }
    fn rotate(&mut self, face: Face, clockwise: bool) {
        self.cubies
            .iter_mut()
            .for_each(|cubie| *cubie = cubie.rotate(face, clockwise))
    }
}

fn main() {
    let mut cube = Cube::new();
    for _ in 0..50 {
        cube.rotate(rand::random(), rand::random());
    }
    println!("{cube:#?}");
}
