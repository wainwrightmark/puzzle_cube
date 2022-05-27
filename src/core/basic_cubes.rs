use crate::core::prelude::CornerPosition::*;
use crate::core::prelude::EdgePosition::*;
use crate::core::prelude::FaceColor::*;
use crate::core::prelude::FaceletPosition::*;
use crate::core::prelude::*;
use crate::web::prelude::Cube;
use array_const_fn_init::array_const_fn_init;
use strum::EnumCount;
use strum_macros::*;

type CO = CornerOrientation;
type EO = EdgeOrientation;

pub const MOVE_CUBES: [CubieCube; Move::COUNT]
=[
UP_DEFAULT_CUBE,
UP_DEFAULT_CUBE.multiply(&UP_DEFAULT_CUBE),
UP_DEFAULT_CUBE.multiply(&UP_DEFAULT_CUBE).multiply(&UP_DEFAULT_CUBE),
RIGHT_DEFAULT_CUBE,
RIGHT_DEFAULT_CUBE.multiply(&RIGHT_DEFAULT_CUBE),
RIGHT_DEFAULT_CUBE.multiply(&RIGHT_DEFAULT_CUBE).multiply(&RIGHT_DEFAULT_CUBE),
FRONT_DEFAULT_CUBE,
FRONT_DEFAULT_CUBE.multiply(&FRONT_DEFAULT_CUBE),
FRONT_DEFAULT_CUBE.multiply(&FRONT_DEFAULT_CUBE).multiply(&FRONT_DEFAULT_CUBE),
DOWN_DEFAULT_CUBE,
DOWN_DEFAULT_CUBE.multiply(&DOWN_DEFAULT_CUBE),
DOWN_DEFAULT_CUBE.multiply(&DOWN_DEFAULT_CUBE).multiply(&DOWN_DEFAULT_CUBE),
LEFT_DEFAULT_CUBE,
LEFT_DEFAULT_CUBE.multiply(&LEFT_DEFAULT_CUBE),
LEFT_DEFAULT_CUBE.multiply(&LEFT_DEFAULT_CUBE).multiply(&LEFT_DEFAULT_CUBE),
BACK_DEFAULT_CUBE,
BACK_DEFAULT_CUBE.multiply(&BACK_DEFAULT_CUBE),
BACK_DEFAULT_CUBE.multiply(&BACK_DEFAULT_CUBE).multiply(&BACK_DEFAULT_CUBE)
];


pub const UP_DEFAULT_CUBE: CubieCube = CubieCube {
    edge_positions: [Ub, Ur, Uf, Ul, Dr, Df, Dl, Db, Fr, Fl, Bl, Br],
    corner_positions: [Ubr, Urf, Ufl, Ulb, Dfr, Dlf, Dbl, Drb],
    edge_orientations: [EO::Zero; 12],
    corner_orientations: [CO::Zero; 8],
};



pub const RIGHT_DEFAULT_CUBE: CubieCube = CubieCube {
    edge_positions: [Fr, Uf, Ul, Ub, Br, Df, Dl, Db, Dr, Fl, Bl, Ur],
    corner_positions: [Dfr,
    Ufl,
    Ulb,
    Urf,
    Drb,
    Dlf,
    Dbl,
    Ubr],
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