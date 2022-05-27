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

#[derive(
    Debug,
    Display,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
    Copy,
    Clone,
    FromRepr,
    EnumIter,
    EnumCount,
)]
#[repr(u8)]
pub enum Move {
    U1 = 0,
    U2 = 1,
    U3 = 2,
    R1 = 3,
    R2 = 4,
    R3 = 5,
    F1 = 6,
    F2 = 7,
    F3 = 8,
    D1 = 9,
    D2 = 10,
    D3 = 11,
    L1 = 12,
    L2 = 13,
    L3 = 14,
    B1 = 15,
    B2 = 16,
    B3 = 17,
}

impl Move {
    pub const fn get_color(self) -> FaceColor {
        let color: u8 = self as u8 / 3;
        FaceColor::from_repr(color).unwrap()
    }

    pub const fn into_move_number(self) -> MoveNumber {
        let number: u8 = (self as u8 % 3) + 1;
        MoveNumber::from_repr(number).unwrap()
    }

    pub const fn from((color, number): (FaceColor, MoveNumber)) -> Self {
        let c: u8 = color as u8;
        let n: u8 = number as u8;
        let m = (c * 3) + n - 1;
        Move::from_repr(m).unwrap()
    }

    pub const PHASE2MOVES: [Move; 10] = [
        Move::U1,
        Move::U2,
        Move::U3,
        Move::R2,
        Move::F2,
        Move::D1,
        Move::D2,
        Move::D3,
        Move::L2,
        Move::B2,
    ];

    pub const fn inverse(self) -> Self {
        let m = (self as u8 / 3) * 3;
        let n = 2 - self as u8 % 3;

        let r = m + n;
        Move::from_repr(r).unwrap()
    }

    ///Can m1 precede m2 in a solution
    pub const fn can_precede(self, m2: Move) -> bool {
        self.get_color() as u8 != m2.get_color() as u8
    }
}

impl Move {
    //pub const BASIC_CUBES: [CubieCube; 18] = [CubieCube::default(); 18];

    pub const fn get_move_cube(self) -> CubieCube {
        match self {
            Move::U1 => UP_DEFAULT_CUBE,
            Move::D1 => DOWN_DEFAULT_CUBE,
            Move::L1 => LEFT_DEFAULT_CUBE,
            Move::R1 => RIGHT_DEFAULT_CUBE,
            Move::B1 => BACK_DEFAULT_CUBE,
            Move::F1 => FRONT_DEFAULT_CUBE,
            _ => UP_DEFAULT_CUBE,
        }
    }

    pub fn apply(self, cube: &CubieCube)-> CubieCube{
        cube.clone().multiply(&self.get_move_cube())
    }
}

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
