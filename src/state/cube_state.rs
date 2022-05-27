use crate::core::prelude::*;
use crate::state::prelude::*;
use itertools::Itertools;
use num::ToPrimitive;
use serde::*;
use std::rc::Rc;
use yewdux::prelude::*;

#[derive(PartialEq, Eq, Store, Clone, Default, Serialize, Deserialize)]
#[store(storage = "local")] // can also be "session"
pub struct CubeState{
    pub cube : CubieCube
}


pub struct RandomizeMsg{}

impl Reducer<CubeState> for RandomizeMsg {
    fn apply(&self, state: Rc<CubeState>) -> Rc<CubeState> {
        let seed: u64 = rand::random();

        let cube = CubieCube::random_cube(seed);

        CubeState{cube}.into()
    }
}

pub struct ResetMsg{}

impl Reducer<CubeState> for ResetMsg {
    fn apply(&self, state: Rc<CubeState>) -> Rc<CubeState> {    
        let cube = CubieCube::default();

        CubeState{cube}.into()
    }
}

pub struct InvertMsg{}

impl Reducer<CubeState> for InvertMsg {
    fn apply(&self, state: Rc<CubeState>) -> Rc<CubeState> {    
        let cube = state.cube.clone().invert();

        CubeState{cube}.into()
    }
}

pub struct MoveMsg{pub my_move: Move}

impl Reducer<CubeState> for MoveMsg{
    fn apply(&self, state: Rc<CubeState>) -> Rc<CubeState> {        
         let cube =self.my_move.apply(&state.cube);
        CubeState{cube}.into()

    }
}