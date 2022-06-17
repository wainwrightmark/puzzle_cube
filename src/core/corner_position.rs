use crate::core::prelude::*;
use crate::core::prelude::FaceletPosition::*;
use crate::core::prelude::FaceColor::*;

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
    EnumCount,    
)]
#[repr(u8)]
pub enum CornerPosition {
    Urf = 0,
    Ufl = 1,
    Ulb = 2,
    Ubr = 3,
    Dfr = 4,
    Dlf = 5,
    Dbl = 6,
    Drb = 7,
}

impl From<CornerPosition> for usize{
    fn from(ep: CornerPosition) -> Self {
        ep as usize
    }
}


const fn get_corner_position(i: usize) -> CornerPosition {
    CornerPosition::from_repr(i as u8).unwrap()
}

impl CornerPosition {
    pub const DEFAULT_ARRAY: [Self; 8] = array_const_fn_init![get_corner_position; 8];

    pub fn get_color(self, index : usize)-> FaceColor{
        CornerPosition::CORNERCOLORS[self as usize][index]
    }

    pub fn get_location(self, index : usize, orientation:  CornerOrientation )-> FaceletPosition{
        CornerPosition::CORNERFACELETS[self as usize][(index + (orientation as usize)) % 3]
    }


    ///The positions of each corner facelet
pub const CORNERFACELETS: [[FaceletPosition; 3];8 ] = [
    [U9, R1, F3, ],
    [U7, F1, L3, ],
    [U1, L1, B3, ],
    [U3, B1, R3, ],
    [D3, F9, R7, ],
    [D1, L9, F7, ],
    [D7, B9, L7, ],
    [D9, R9, B7, ],
    ];
    
    ///The colors of each of the corner pieces
    pub const CORNERCOLORS : [[FaceColor;3];8] =[
        [Up,Right,Front],
        [Up,Front,Left],
        [Up,Left,Back],
        [Up, Back, Right],
        [Down, Front, Right],
        [Down,Left, Front],
        [Down,Back,Left],
        [Down,Right,Back],
    ];
}