use cartesian::*;

type Coord = i8;
type Pos = [Coord; 3];

type Axis = u8;
type Polarity = bool;
const POS: Polarity = true;
const NEG: Polarity = false;

#[derive(Copy, Clone, PartialEq, Eq)]
struct Face {
    axis: Axis,
    pol: Polarity,
}

#[inline]
/// 0 -> 0
/// 1 -> 2
/// 2 -> 1
/// 3 -> 3
fn swap(a: u8) -> u8 {
    3 & (216 >> (a << 1))
}

impl Face {
    fn rotate(self, face: Face, clockwise: bool) -> Self {
        if self.axis == face.axis {
            self
        } else {
            let axis = swap(self.axis + face.axis);
            let pol = self.pol ^ face.pol ^ clockwise; // I hope this is somewhat correct TODO
            Self {axis, pol}
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

#[derive(Copy, Clone, PartialEq, Eq)]
struct Ori {
    white: Face,
    blue: Face,
}

#[derive(Copy, Clone, PartialEq, Eq)]
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

#[derive(Clone, PartialEq, Eq)]
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
}

fn main() {
    for i in 0..4 {
        let k = swap(i);
        println!("{i} -> {k}");
    }
}
