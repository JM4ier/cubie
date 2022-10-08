#![cfg(test)]

use crate::*;

param_test!(
    third_axis_test(
        x: [0,1,2],
        y: [0,1,2],
    ) {
        let x = *x;
        let y = *y;
        if x == y {
            return;
        }
        let z = third_axis(x, y);
        assert!(z != x);
        assert!(z != y);
        assert!(z < 3);
    }
);

param_test!(
    rotation_test(
        axis1: [0,1,2],
        axis2: [0,1,2],
    ) {
        let face1 = Face {
            axis: *axis1,
            pol: POS,
        };
        let face2 = Face {
            axis: *axis2,
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
);

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
    rotation_injectivity(
        face1: Face::all(),
        face2: Face::all(),
        axis: Face::all(),
    ) {
        if face1 == face2 {
            return;
        }
        assert!(face1.rotate(*axis, true) != face2.rotate(*axis, true));
    }
);

param_test!(
    opposite_rot_is_inverse1(
        face: Face::all(),
        rot: Face::all(),
    ) {
        let face1 = face.rotate(*rot, true).rotate(rot.invert(), true);
        assert_eq!(*face, face1);
    }
);

param_test!(
    opposite_rot_is_inverse2(
        face: Face::all(),
        rot: Face::all(),
    ) {
        let face1 = face.rotate(*rot, true);
        let face2 = face.rotate(rot.invert(), false);
        assert_eq!(face1, face2);
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
