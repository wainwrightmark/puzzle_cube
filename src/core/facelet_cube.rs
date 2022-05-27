use crate::core::prelude::*;
use serde_with::*;
use strum::EnumCount;
use strum_macros::*;
use array_const_fn_init::array_const_fn_init;


#[serde_as]
#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, Clone)]
pub struct FaceletCube {
    #[serde_as(as = "[_; 54]")]
    pub facelets: [FaceColor; 54],
}


const fn get_solved_face(i: usize) -> FaceColor {
    match i / 6 {
        0 => FaceColor::Up,
        1 => FaceColor::Right,
        2 => FaceColor::Front,
        3 => FaceColor::Down,
        4 => FaceColor::Left,
        _=> FaceColor::Back
    }
}

impl FaceletCube {
    pub const SOLVED: FaceletCube = FaceletCube{
        facelets: array_const_fn_init![get_solved_face; 54]
        
    };


}

impl Default for FaceletCube {
    fn default() -> Self {
        FaceletCube::SOLVED
    }
}
