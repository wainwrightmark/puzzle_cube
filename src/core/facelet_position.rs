use crate::core::prelude::*;
use strum_macros::*;

impl From<(FaceColor, HorizontalPosition, VerticalPosition)> for FaceletPosition {
    fn from(g: (FaceColor, HorizontalPosition, VerticalPosition)) -> Self {
        let s = (g.0 as u8 * 9) + (g.1 as u8) + (g.2 as u8 * 3);
        FaceletPosition::from_repr(s).unwrap()
    }
}

impl FaceletPosition {
    pub const fn get_face(self) -> FaceColor {
        let fc = self as u8 / 9;
        FaceColor::from_repr(fc).unwrap()
    }

    pub const fn get_horizontal_position(self) -> HorizontalPosition {
        let hp = (self as u8) % 3;
        HorizontalPosition::from_repr(hp).unwrap()
    }

    pub const fn get_vertical_position(self) -> VerticalPosition {
        let vp = (self as u8 % 9) / 3;
        VerticalPosition::from_repr(vp).unwrap()
    }
}

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
)]
#[repr(u8)]
pub enum FaceletPosition {
    U1 = 0,

    U2 = 1,

    U3 = 2,

    U4 = 3,

    U5 = 4,

    U6 = 5,

    U7 = 6,

    U8 = 7,

    U9 = 8,

    R1 = 9,

    R2 = 10,

    R3 = 11,

    R4 = 12,

    R5 = 13,

    R6 = 14,

    R7 = 15,

    R8 = 16,

    R9 = 17,

    F1 = 18,

    F2 = 19,

    F3 = 20,

    F4 = 21,

    F5 = 22,

    F6 = 23,

    F7 = 24,

    F8 = 25,

    F9 = 26,

    D1 = 27,

    D2 = 28,

    D3 = 29,

    D4 = 30,

    D5 = 31,

    D6 = 32,

    D7 = 33,

    D8 = 34,

    D9 = 35,

    L1 = 36,

    L2 = 37,

    L3 = 38,

    L4 = 39,

    L5 = 40,

    L6 = 41,

    L7 = 42,

    L8 = 43,

    L9 = 44,

    B1 = 45,

    B2 = 46,

    B3 = 47,

    B4 = 48,

    B5 = 49,

    B6 = 50,

    B7 = 51,
    B8 = 52,
    B9 = 53,
}
