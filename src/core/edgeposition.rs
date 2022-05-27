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
pub enum EdgePosition {
    Ur = 0,
    Uf = 1,
    Ul = 2,
    Ub = 3,
    Dr = 4,
    Df = 5,
    Dl = 6,
    Db = 7,
    Fr = 8,
    Fl = 9,
    Bl = 10,
    Br = 11,
}

impl From<EdgePosition> for usize{
    fn from(ep: EdgePosition) -> Self {
        ep as usize
    }
}


const fn get_edge_position(i: usize) -> EdgePosition {
    EdgePosition::from_repr(0).unwrap()
}


impl EdgePosition {
    pub const fn default_array() -> [Self; 12] {
        array_const_fn_init![get_edge_position; 12]
    }

    ///The positions of each edge facelet
pub const EDGEFACELETS:[[FaceletPosition;2];12] =[
    [U6, R2],
    [U8, F2],
    [U4, L2],
    [U2, B2],
    
    [D6, R8],
    [D2, F8],
    [D4, L8],
    [D8, B8],
    
    [F6, R4],
    [F4, L6],
    [B6, L4],
    [B4, R6],
    ];
    
    
    
    ///The colors of each of the edge pieces
    pub const EDGECOLORS : [[FaceColor;2];12] =
    [
        [Up, Right],
        [Up, Front],
        [Up, Left],
        [Up, Back],
    
        [Down, Right],
        [Down, Front],
        [Down, Left],
        [Down, Back],
    
        [Front, Right],
        [Front, Left],
        [Back, Left],
        [Back, Right],
    ];
}