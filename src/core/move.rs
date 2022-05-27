use crate::core::prelude::*;
use crate::core::prelude::FaceletPosition::*;
use crate::core::prelude::FaceColor::*;
use strum::EnumCount;
use strum_macros::*;
use array_const_fn_init::array_const_fn_init;


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