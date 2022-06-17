use crate::core::prelude::CornerPosition::*;
use crate::core::prelude::EdgePosition::*;

use crate::core::prelude::*;
use array_const_fn_init::array_const_fn_init;
use strum::EnumCount;

type CO = CornerOrientation;
type EO = EdgeOrientation;

pub const MOVE_CUBES: [CubieCube; Move::COUNT] = [
    UP_DEFAULT_CUBE,
    UP_DEFAULT_CUBE.multiply(&UP_DEFAULT_CUBE),
    UP_DEFAULT_CUBE
        .multiply(&UP_DEFAULT_CUBE)
        .multiply(&UP_DEFAULT_CUBE),
    RIGHT_DEFAULT_CUBE,
    RIGHT_DEFAULT_CUBE.multiply(&RIGHT_DEFAULT_CUBE),
    RIGHT_DEFAULT_CUBE
        .multiply(&RIGHT_DEFAULT_CUBE)
        .multiply(&RIGHT_DEFAULT_CUBE),
    FRONT_DEFAULT_CUBE,
    FRONT_DEFAULT_CUBE.multiply(&FRONT_DEFAULT_CUBE),
    FRONT_DEFAULT_CUBE
        .multiply(&FRONT_DEFAULT_CUBE)
        .multiply(&FRONT_DEFAULT_CUBE),
    DOWN_DEFAULT_CUBE,
    DOWN_DEFAULT_CUBE.multiply(&DOWN_DEFAULT_CUBE),
    DOWN_DEFAULT_CUBE
        .multiply(&DOWN_DEFAULT_CUBE)
        .multiply(&DOWN_DEFAULT_CUBE),
    LEFT_DEFAULT_CUBE,
    LEFT_DEFAULT_CUBE.multiply(&LEFT_DEFAULT_CUBE),
    LEFT_DEFAULT_CUBE
        .multiply(&LEFT_DEFAULT_CUBE)
        .multiply(&LEFT_DEFAULT_CUBE),
    BACK_DEFAULT_CUBE,
    BACK_DEFAULT_CUBE.multiply(&BACK_DEFAULT_CUBE),
    BACK_DEFAULT_CUBE
        .multiply(&BACK_DEFAULT_CUBE)
        .multiply(&BACK_DEFAULT_CUBE),
];

/// Cubes representing the basic symmetries and their combinations
pub const SYMMETRY_CUBES: [CubieCube; 48] = array_const_fn_init![get_symmetry_cube; 48];

/// The inverses of the symmetry cubes
pub const SYMMETRY_CUBES_INVERTED: [CubieCube; 48] =
    array_const_fn_init![get_inverse_symmetry_cube; 48];

// const fn get_symmetry_cube(i: usize) -> CubieCube {

//     let mut c = CubieCube::default();
//     let mut index = i;

//     let mut itr = 0;
//     while(itr < index % 2){
//         c = c.multiply(&MIRROR_LR2_SYMMETRY);
//         itr +=1;
//     }
//     index /= 2;

//     itr = 0;
//     while(itr < index % 4){
//         c = c.multiply(&U4_SYMMETRY);
//         itr +=1;
//     }
//     index /= 4;

//     itr = 0;
//     while(itr < index % 2){
//         c = c.multiply(&F2_SYMMETRY);
//         itr +=1;
//     }
//     index /= 2;

//     itr = 0;
//     while(itr < index % 3){
//         c = c.multiply(&URF3_SYMMETRY);
//         itr +=1;
//     }

//     c
// }

const fn get_symmetry_cube(i: usize) -> CubieCube {
    let mut urf3 = i / 16;
    let mut f2 = (i % 16) / 8;
    let mut u4 = (i % 8) / 2;
    let mut lr2 = i % 2;

    let mut c = CubieCube::default();

    while urf3 > 0 {
        c = c.multiply(&URF3_SYMMETRY);
        urf3 -= 1;
    }
    while f2 > 0 {
        c = c.multiply(&F2_SYMMETRY);
        f2 -= 1;
    }
    while u4 > 0 {
        c = c.multiply(&U4_SYMMETRY);
        u4 -= 1;
    }
    while lr2 > 0 {
        c = c.multiply(&MIRROR_LR2_SYMMETRY);
        lr2 -= 1;
    }

    c
}

const fn get_inverse_symmetry_cube(j: usize) -> CubieCube {
    SYMMETRY_CUBES[j].invert()
}

pub const UP_DEFAULT_CUBE: CubieCube = CubieCube {
    edge_positions: [Ub, Ur, Uf, Ul, Dr, Df, Dl, Db, Fr, Fl, Bl, Br],
    corner_positions: [Ubr, Urf, Ufl, Ulb, Dfr, Dlf, Dbl, Drb],
    edge_orientations: [EO::Zero; 12],
    corner_orientations: [CO::Zero; 8],
};

pub const RIGHT_DEFAULT_CUBE: CubieCube = CubieCube {
    edge_positions: [Fr, Uf, Ul, Ub, Br, Df, Dl, Db, Dr, Fl, Bl, Ur],
    corner_positions: [Dfr, Ufl, Ulb, Urf, Drb, Dlf, Dbl, Ubr],
    edge_orientations: [EO::Zero; 12],
    corner_orientations: [
        CO::Two,
        CO::Zero,
        CO::Zero,
        CO::One,
        CO::One,
        CO::Zero,
        CO::Zero,
        CO::Two,
    ],
};

pub const LEFT_DEFAULT_CUBE: CubieCube = CubieCube {
    edge_positions: [Ur, Uf, Bl, Ub, Dr, Df, Fl, Db, Fr, Ul, Dl, Br],
    corner_positions: [Urf, Ulb, Dbl, Ubr, Dfr, Ufl, Dlf, Drb],
    edge_orientations: [EO::Zero; 12],
    corner_orientations: [
        CO::Zero,
        CO::One,
        CO::Two,
        CO::Zero,
        CO::Zero,
        CO::Two,
        CO::One,
        CO::Zero,
    ],
};

pub const DOWN_DEFAULT_CUBE: CubieCube = CubieCube {
    edge_positions: [Ur, Uf, Ul, Ub, Df, Dl, Db, Dr, Fr, Fl, Bl, Br],
    corner_positions: [Urf, Ufl, Ulb, Ubr, Dlf, Dbl, Drb, Dfr],
    edge_orientations: [EO::Zero; 12],
    corner_orientations: [CO::Zero; 8],
};

pub const FRONT_DEFAULT_CUBE: CubieCube = CubieCube {
    edge_positions: [Ur, Fl, Ul, Ub, Dr, Fr, Dl, Db, Uf, Df, Bl, Br],
    corner_positions: [Ufl, Dlf, Ulb, Ubr, Urf, Dfr, Dbl, Drb],
    edge_orientations: [
        EO::Zero,
        EO::One,
        EO::Zero,
        EO::Zero,
        EO::Zero,
        EO::One,
        EO::Zero,
        EO::Zero,
        EO::One,
        EO::One,
        EO::Zero,
        EO::Zero,
    ],
    corner_orientations: [
        CO::One,
        CO::Two,
        CO::Zero,
        CO::Zero,
        CO::Two,
        CO::One,
        CO::Zero,
        CO::Zero,
    ],
};

pub const BACK_DEFAULT_CUBE: CubieCube = CubieCube {
    edge_positions: [Ur, Uf, Ul, Br, Dr, Df, Dl, Bl, Fr, Fl, Ub, Db],
    corner_positions: [Urf, Ufl, Ubr, Drb, Dfr, Dlf, Ulb, Dbl],
    edge_orientations: [
        EO::Zero,
        EO::Zero,
        EO::Zero,
        EO::One,
        EO::Zero,
        EO::Zero,
        EO::Zero,
        EO::One,
        EO::Zero,
        EO::Zero,
        EO::One,
        EO::One,
    ],
    corner_orientations: [
        CO::Zero,
        CO::Zero,
        CO::One,
        CO::Two,
        CO::Zero,
        CO::Zero,
        CO::Two,
        CO::One,
    ],
};

///120° clockwise rotation around the long diagonal URF-DBL
pub const URF3_SYMMETRY: CubieCube = CubieCube {
    edge_positions: [Uf, Fr, Df, Fl, Ub, Br, Db, Bl, Ur, Dr, Dl, Ul],
    corner_positions: [Urf, Dfr, Dlf, Ufl, Ubr, Drb, Dbl, Ulb],
    edge_orientations: [
        EO::One,
        EO::Zero,
        EO::One,
        EO::Zero,
        EO::One,
        EO::Zero,
        EO::One,
        EO::Zero,
        EO::One,
        EO::One,
        EO::One,
        EO::One,
    ],
    corner_orientations: [
        CO::One,
        CO::Two,
        CO::One,
        CO::Two,
        CO::Two,
        CO::One,
        CO::Two,
        CO::One,
    ],
};

/// 180° rotation around the axis through the F and B centers
pub const F2_SYMMETRY: CubieCube = CubieCube {
    edge_positions: [Dl, Df, Dr, Db, Ul, Uf, Ur, Ub, Fl, Fr, Br, Bl],
    corner_positions: [Dlf, Dfr, Drb, Dbl, Ufl, Urf, Ubr, Ulb],
    edge_orientations: [EO::Zero; 12],
    corner_orientations: [CO::Zero; 8],
};

/// 90° clockwise rotation around the axis through the U and D centers
pub const U4_SYMMETRY: CubieCube = CubieCube {
    edge_positions: [Ub, Ur, Uf, Ul, Db, Dr, Df, Dl, Br, Fr, Fl, Bl],
    corner_positions: [Ubr, Urf, Ufl, Ulb, Drb, Dfr, Dlf, Dbl],
    edge_orientations: [
        EO::Zero,
        EO::Zero,
        EO::Zero,
        EO::Zero,
        EO::Zero,
        EO::Zero,
        EO::Zero,
        EO::Zero,
        EO::One,
        EO::One,
        EO::One,
        EO::One,
    ],
    corner_orientations: [CO::Zero; 8],
};

/// <summary>
/// reflection at the plane through the U, D, F, B centers
/// </summary>
pub const MIRROR_LR2_SYMMETRY: CubieCube = CubieCube {
    edge_positions: [Ul, Uf, Ur, Ub, Dl, Df, Dr, Db, Fl, Fr, Br, Bl],
    corner_positions: [Ufl, Urf, Ubr, Ulb, Dlf, Dfr, Drb, Dbl],
    edge_orientations: [EO::Zero; 12],
    corner_orientations: [CO::Three; 8],
};
