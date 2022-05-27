use crate::core::prelude::*;
use crate::core::prelude::FaceletPosition::*;
use crate::core::prelude::FaceColor::*;
use strum::EnumCount;
use strum::IntoEnumIterator;
use strum_macros::*;
use array_const_fn_init::array_const_fn_init;

pub const NPERM4: usize = 24;
pub const NCHOOSE84: usize = 70;
pub const NMOVE: usize = 18;
pub const NTWIST: usize = 2187;
pub const NFLIP: usize = 2048;
pub const NSLICESORTED: usize = 11880;
pub const NSLICE: usize = NSLICESORTED / NPERM4;
pub const NFLIPSLICECLASS: usize = 64430;
pub const NUEDGESPHASE2: usize = 1680;
pub const NCORNERS: usize = 40320;
pub const NCORNERSCLASS: usize = 2768;
pub const NUDEDGES: usize = 40320;
pub const NSYM: usize = 48;
pub const NSYMD4H: usize = 16;



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

impl From<HorizontalPosition> for usize{
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

impl From<VerticalPosition> for usize{
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

impl From<EdgeOrientation> for usize{
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

impl From<CornerOrientation> for usize{
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
    pub const fn get_x(self)-> usize{
        match self {
            Up => 1,
            Right => 2,
            Front => 1,
            Down => 1,
            Left => 0,
            Back => 3,
        }
    }

    pub const fn get_y(self)-> usize{
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

impl From<FaceColor> for usize{
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

impl From<MoveNumber> for usize{
    fn from(ep: MoveNumber) -> Self {
        ep as usize
    }
}
