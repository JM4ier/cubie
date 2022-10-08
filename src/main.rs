use cartesian::*;
use rand::{distributions::Standard, prelude::Distribution};

#[macro_use]
mod testing;

type Coord = i8;
type Pos = [Coord; 3];

trait ToVector3 {
    fn to_vec3(&self) -> Vector3;
}

impl ToVector3 for Pos {
    #[inline]
    fn to_vec3(&self) -> Vector3 {
        Vector3::new(self[0] as f32, self[1] as f32, self[2] as f32)
    }
}

type Axis = u8;
type Polarity = bool;
const POS: Polarity = true;
const NEG: Polarity = false;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Face {
    axis: Axis,
    pol: Polarity,
}

impl Face {
    #[inline]
    fn normal(&self) -> Vector3 {
        let mut vec = [0; 3];
        vec[self.axis as usize] = if self.pol == POS { 1 } else { -1 };
        vec.to_vec3()
    }
    #[inline]
    fn area(&self) -> Vector3 {
        let mut vec = [0; 3];
        let a = self.axis as usize;
        vec[a] = 1;
        vec[(a + 1) % 3] = 10;
        vec[(a + 2) % 3] = 10;
        vec.to_vec3() * 0.1
    }
}

impl Face {
    fn white() -> Self {
        Self { axis: 0, pol: POS }
    }
    fn blue() -> Self {
        Self { axis: 1, pol: POS }
    }
    fn orange() -> Self {
        Self { axis: 2, pol: POS }
    }
    fn green() -> Self {
        Self::blue().invert()
    }
    fn yellow() -> Self {
        Self::white().invert()
    }
    fn pink() -> Self {
        Self::orange().invert()
    }
    fn all() -> Vec<Self> {
        let mut v = vec![];
        for (axis, pol) in cartesian!(0..3, [NEG, POS]) {
            v.push(Self {axis, pol});
        }
        v
    }
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
    fn invert(mut self) -> Self {
        self.pol = !self.pol;
        self
    }
    fn rotate(self, face: Face, clockwise: bool) -> Self {
        if !clockwise {
            return self.rotate(face.invert(), true);
        }
        if self.axis == face.axis {
            self
        } else {
            let axis = third_axis(self.axis, face.axis);
            let pol_mod = face.axis == (self.axis + 2) % 3;
            let pol = self.pol ^ pol_mod;
            Self { axis, pol }
        }
    }
}

#[test]
fn rotation_test() {
    for (axis1, axis2) in cartesian!(0..3, 0..3) {
        let face1 = Face {
            axis: axis1,
            pol: POS,
        };
        let face2 = Face {
            axis: axis2,
            pol: POS,
        };
        let rot1 = face1.rotate(face2, POS);
        let rot2 = rot1.rotate(face2, POS);

        assert_eq!(face1.axis == face2.axis, rot1 == face1);
        assert_eq!(rot2.axis, face1.axis);
        assert_eq!(
            rot2.pol == face1.pol,
            face1.axis == face2.axis,
            "{face1:?}  {face2:?}"
        );
    }
}

param_test!(
    rotation_is_not_same_as_counterrotation(
        face: Face::all(),
        rot: Face::all(),
    ) {
        if face.axis == rot.axis {
            return;
        }
        let rot1 = face.rotate(*rot, true);
        let rot2 = face.rotate(*rot, false);
        assert!(rot1 != rot2, "{face:?} {rot:?}");
    }
);

param_test!(
    rotation_test_2(
        data: [
            (Face::white(), Face::blue(), Face::pink()),
            (Face::white(), Face::pink(), Face::green()),
            (Face::white(), Face::green(), Face::orange()),
            (Face::white(), Face::orange(), Face::blue()),
            (Face::orange(), Face::orange(), Face::orange()),
            (Face::orange(), Face::blue(), Face::white()),
            (Face::orange(), Face::white(), Face::green()),
        ],
    ) {
        let (axis, face, expected) = data;
        let actual = face.rotate(*axis, true);
        assert_eq!(actual, *expected);
    }
);

param_test!(
    rotation_noop_4(
        face1: Face::all(),
        face2: Face::all(),
        clockwise: [false, true],
    ) {
        let mut face3 = *face1;
        for _ in 0..4 {
            face3 = face3.rotate(*face2, *clockwise);
        }
        assert_eq!(*face1, face3);
    }
);

param_test!(
    aligned_axis_invariant_rotation(
        face1: Face::all(),
        face2: Face::all(),
        clockwise: [false, true],
    ) {
        if face1.axis != face2.axis {
            return;
        }
        assert_eq!(*face1, face1.rotate(*face2, *clockwise));
    }
);

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
    fn rotate(self, face: Face, clockwise: Polarity) -> Self {
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
    fn get_color(&self, face: Face) -> u8 {
        if self.rot.white == face {
            0 // white
        } else if self.rot.white.axis == face.axis {
            5 // yellow
        } else if self.rot.blue == face {
            1 // blue
        } else if self.rot.blue.axis == face.axis {
            4 // green
        } else {
            let third_face = self.rot.blue.rotate(self.rot.white, true);

            // todo does this make any sense at all
            if third_face == face {
                2
            } else {
                3
            }
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

param_test!(
    cube_inverse_rot_is_ident(
        turn: Face::all(),
    ) {
        let mut cube = Cube::new();
        cube.rotate(*turn, true);
        cube.rotate(*turn, false);
        assert_eq!(Cube::new(), cube, "turn is {turn:?}");
    }
);

use raylib::prelude::*;

fn main() {
    let mut cube = Cube::new();
    for _ in 0..0 {
        let clockwise = rand::random();
        let face = rand::random();
        cube.rotate(face, clockwise);
        cube.rotate(face, !clockwise);
    }
    println!("{cube:#?}");

    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("I <3 El Tony Mate")
        .build();

    let cam = Camera::orthographic(Vector3::one() * 5.0, Vector3::zero(), Vector3::up(), 6.0);

    rl.set_target_fps(60);

    let key_map = {
        use KeyboardKey::*;
        [
            (KEY_W, Face::white()),
            (KEY_B, Face::blue()),
            (KEY_Y, Face::yellow()),
            (KEY_G, Face::green()),
            (KEY_P, Face::pink()),
            (KEY_O, Face::orange()),
        ]
    };

    while !rl.window_should_close() {
        let ccw = rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT)
            || rl.is_key_down(KeyboardKey::KEY_RIGHT_SHIFT);

        for (key, rot) in key_map.iter() {
            if rl.is_key_pressed(*key) {
                cube.rotate(*rot, !ccw);
            }
        }

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::RAYWHITE);

        //cam.position.rotate(Vector4::new(0.0, 0.0001, 0.0, 1.0));

        let mut d = d.begin_mode3D(cam);
        let big_size = 2.75;
        let smol_size = 0.6;
        d.draw_cube(Vector3::zero(), big_size, big_size, big_size, Color::BLACK);

        let colors = [
            Color::WHITE,
            Color::BLUE,
            Color::PINK,
            Color::ORANGE,
            Color::GREEN,
            Color::YELLOW,
        ];

        for cubie in cube.cubies.iter() {
            d.draw_cube(
                cubie.pos.to_vec3(),
                smol_size,
                smol_size,
                smol_size,
                Color::GRAY,
            );
            for axis in 0..3 {
                for pol in [NEG, POS] {
                    let face = Face { axis, pol };

                    let scale = 0.7;

                    let offset = face.normal() * 0.5 * scale;
                    let area = face.area() * scale;
                    let position = cubie.pos.to_vec3() + offset;

                    d.draw_cube_v(position, area, colors[cubie.get_color(face) as usize]);
                }
            }
        }
    }
}
