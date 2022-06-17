use crate::core::prelude::FaceColor::*;

use strum_macros::*;

pub const NPERM4: usize = 4 * 3 * 2;
pub const NCHOOSE84: usize = 70; //8 choose 4
pub const NTWIST: usize = 2187; // 3 ** 7;
pub const NFLIP: usize = 2048; // 2 ** 11;
pub const NSLICESORTED: usize = 11880; //12 choose 4 * 4 factorial
pub const NSLICE: usize = 495; //12 choose 4
pub const NFLIPSLICECLASS: usize = 64430;
pub const NUEDGESPHASE2: usize = 1680; //8 choose 4 * 24
pub const NCORNERS: usize = 40320; // 8 factorial;
pub const NCORNERSCLASS: usize = 2768;
pub const NUDEDGES: usize = 8 * 7 * 6 * 5 * 4 * 3 * 2; //8 factorial
pub const NSYM: usize = 48; // 2 * 3 * 4 * 2;
pub const NSYMD4H: usize = 24; // 2 * 3 * 4;

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
pub enum HorizontalPosition {
    Left = 0,
    Middle = 1,
    Right = 2,
}

impl From<HorizontalPosition> for usize {
    fn from(ep: HorizontalPosition) -> Self {
        ep as usize
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
    EnumCount,
)]
#[repr(u8)]
pub enum VerticalPosition {
    Top = 0,
    Middle = 1,
    Bottom = 2,
}

impl From<VerticalPosition> for usize {
    fn from(ep: VerticalPosition) -> Self {
        ep as usize
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
    EnumCount,
)]
#[repr(u8)]
pub enum EdgeOrientation {
    Zero = 0,
    One = 1,
}

impl From<EdgeOrientation> for usize {
    fn from(ep: EdgeOrientation) -> Self {
        ep as usize
    }
}

impl From<usize> for EdgeOrientation {
    fn from(x: usize) -> Self {
        EdgeOrientation::from_repr(x as u8).unwrap()
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
    EnumCount,
)]
#[repr(u8)]
pub enum CornerOrientation {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
}

impl From<CornerOrientation> for usize {
    fn from(ep: CornerOrientation) -> Self {
        ep as usize
    }
}

impl From<usize> for CornerOrientation {
    fn from(x: usize) -> Self {
        CornerOrientation::from_repr(x as u8).unwrap()
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
    EnumCount,
    Hash,
)]
#[repr(u8)]
pub enum FaceColor {
    Up = 0,
    Right = 1,
    Front = 2,
    Down = 3,
    Left = 4,
    Back = 5,
}

impl FaceColor {
    pub fn get_color_string(self) -> String {
        match self {
            Up => "yellow",
            Right => "green",
            Front => "red",
            Down => "white",
            Left => "blue",
            Back => "orange",
        }
        .to_string()
    }

    pub const fn get_x(self) -> usize {
        match self {
            Up => 1,
            Right => 2,
            Front => 1,
            Down => 1,
            Left => 0,
            Back => 3,
        }
    }

    pub const fn get_y(self) -> usize {
        match self {
            Up => 0,
            Right => 1,
            Front => 1,
            Down => 2,
            Left => 1,
            Back => 1,
        }
    }
}

impl From<FaceColor> for usize {
    fn from(ep: FaceColor) -> Self {
        ep as usize
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
    EnumCount,
)]
#[repr(u8)]
pub enum MoveNumber {
    One = 1,
    Two = 2,
    Three = 3,
}

impl From<MoveNumber> for usize {
    fn from(ep: MoveNumber) -> Self {
        ep as usize
    }
}
