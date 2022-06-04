use crate::core::prelude::*;
use crate::state::prelude::*;
use itertools::Itertools;
use num::ToPrimitive;
use serde::*;
use std::default;
use std::rc::Rc;
use yewdux::prelude::*;

#[derive(PartialEq, Eq, Store, Clone, Default, Serialize, Deserialize)]
#[store(storage = "local")] // can also be "session"
pub struct CubeState {
    pub cube: SomeCube,
}

impl CubeState{

pub fn is_cubie(&self)->bool{
    matches!(self.cube, SomeCube::Cubie { cube:_ })
}

    pub fn try_get_edge_position(&self, edge: &EdgePosition) -> Option<(EdgePosition, EdgeOrientation)>
    {
        if let SomeCube::Cubie { cube } = self.cube.clone() {
            let position_index = cube
                .edge_positions
                .into_iter()
                .position(|x| x == *edge)
                .unwrap();
            let position = EdgePosition::from_repr(position_index as u8).unwrap();

            let orientation = cube.edge_orientations[position_index];

            Some((position, orientation))
        } else {
            None
        }
    }
    
    pub fn try_get_corner_position(&self, corner: &CornerPosition) -> Option<(CornerPosition, CornerOrientation)>
    {
        if let SomeCube::Cubie { cube } = self.cube.clone() {
            let position_index = cube
                .corner_positions
                .into_iter()
                .position(|x| x == *corner)
                .unwrap();
            let position = CornerPosition::from_repr(position_index as u8).unwrap();

            let orientation = cube.corner_orientations[position_index];

            Some((position, orientation))
        } else {
            None
        }
    }
}

#[derive(PartialEq, Eq, Store, Clone, Serialize, Deserialize)]
pub enum SomeCube {
    Cubie {
        cube: Rc<CubieCube>,
    },
    Facelet {
        cube: Rc<FaceletCube>,
        color: Option<FaceColor>,
    },
}

impl Default for SomeCube {
    fn default() -> Self {
        SomeCube::Cubie {
            cube: CubieCube::default().into(),
        }
    }
}



pub struct SetPaintColorMsg{
    pub color: FaceColor
}

impl Reducer<CubeState> for SetPaintColorMsg {
    fn apply(&self, state: Rc<CubeState>) -> Rc<CubeState> {
        match state.cube.clone() {
            SomeCube::Cubie { cube } => state,
            SomeCube::Facelet { cube, color:_ } => {
                CubeState {
                    cube: SomeCube::Facelet {
                        cube,
                        color: Some(self.color),
                    },
                }
                .into()
            }
        }
    }
}

pub struct ClickedMsg {
    pub position: FaceletPosition,
}

impl Reducer<CubeState> for ClickedMsg {
    fn apply(&self, state: Rc<CubeState>) -> Rc<CubeState> {
        match state.cube.clone() {
            SomeCube::Cubie { cube } => state,
            SomeCube::Facelet { cube, color } => {
                if self.position.get_horizontal_position() == HorizontalPosition::Middle
                    && self.position.get_vertical_position() == VerticalPosition::Middle
                {
                    CubeState {
                        cube: SomeCube::Facelet {
                            cube,
                            color: Some(self.position.get_face()),
                        },
                    }
                    .into()
                } else {
                    let mut new_cube = (*cube).clone();
                    new_cube.facelets[self.position as usize] = color;

                    CubeState {
                        cube: SomeCube::Facelet {
                            cube: new_cube.into(),
                            color,
                        },
                    }
                    .into()
                }
            }
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum BasicControlMsg {
    Switch,
    Reset,
    Shuffle,
    Invert,
    Clear,
}

impl Reducer<CubeState> for BasicControlMsg {
    fn apply(&self, state: Rc<CubeState>) -> Rc<CubeState> {
        match self {
            BasicControlMsg::Switch => match state.cube.clone() {
                SomeCube::Cubie { cube } => CubeState {
                    cube: SomeCube::Facelet {
                        cube: FaceletCube::from((*cube).clone()).into(),
                        color: None,
                    },
                }
                .into(),
                SomeCube::Facelet { cube, color } => {
                    if let Ok(cubiecube) = CubieCube::try_from((*cube).clone()) {
                        CubeState {
                            cube: SomeCube::Cubie {
                                cube: Rc::from(cubiecube),
                            },
                        }
                        .into()
                    } else {
                        state
                    }
                }
            },
            BasicControlMsg::Reset => match state.cube.clone() {
                SomeCube::Cubie { cube } => CubeState {
                    cube: SomeCube::Cubie {
                        cube: CubieCube::default().into(),
                    },
                }
                .into(),
                SomeCube::Facelet { cube, color } => CubeState {
                    cube: SomeCube::Facelet {
                        cube: FaceletCube::default().into(),
                        color: None,
                    },
                }
                .into(),
            },
            BasicControlMsg::Shuffle => {
                let seed: u64 = rand::random();

                let new_cube = CubieCube::random_cube(seed);

                match state.cube.clone() {
                    SomeCube::Cubie { cube: _ } => CubeState {
                        cube: SomeCube::Cubie {
                            cube: new_cube.into(),
                        },
                    }
                    .into(),
                    SomeCube::Facelet { cube: _, color } => CubeState {
                        cube: SomeCube::Facelet {
                            cube: Rc::from(FaceletCube::from(new_cube)),
                            color: None,
                        },
                    }
                    .into(),
                }
            }
            BasicControlMsg::Invert => match state.cube.clone() {
                SomeCube::Cubie { cube } => CubeState {
                    cube: SomeCube::Cubie {
                        cube: cube.invert().into(),
                    },
                }
                .into(),
                SomeCube::Facelet { cube, color } => state,
            },
            BasicControlMsg::Clear => match state.cube.clone() {
                SomeCube::Cubie { cube: _ } => state,
                SomeCube::Facelet { cube: _, color } => CubeState {
                    cube: SomeCube::Facelet {
                        cube: Rc::from(FaceletCube::CLEARED),
                        color: None,
                    },
                }
                .into(),
            },
        }
    }
}



pub struct MoveMsg {
    pub cube: CubieCube,
}

impl Reducer<CubeState> for MoveMsg {
    fn apply(&self, state: Rc<CubeState>) -> Rc<CubeState> {
        match state.cube.clone() {
            SomeCube::Cubie { cube } => CubeState {
                cube: SomeCube::Cubie {
                    cube: cube.multiply(&self.cube).into(),
                },
            }
            .into(),
            SomeCube::Facelet { cube, color } => state,
        }
    }
}
