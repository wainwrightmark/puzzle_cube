use crate::core::prelude::*;
use crate::state::prelude::*;
use itertools::Itertools;
use serde::*;

use std::rc::Rc;
use yewdux::{storage, prelude::*};

#[derive(PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
pub struct CubeState {
    pub cube: SomeCube,
    pub solution: Option<Vec<Move>>,
}

impl Store for CubeState {
    fn new() -> Self {
        init_listener(storage::StorageListener::<Self>::new(storage::Area::Local));

        storage::load(storage::Area::Local)
            .expect("Unable to load state")
            .unwrap_or_default()
    }
}

impl CubeState {
    pub fn is_cubie(&self) -> bool {
        matches!(self.cube, SomeCube::Cubie { cube: _ })
    }

    pub fn try_get_edge_position(
        &self,
        edge: &EdgePosition,
    ) -> Option<(EdgePosition, EdgeOrientation)> {
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

    pub fn try_get_corner_position(
        &self,
        corner: &CornerPosition,
    ) -> Option<(CornerPosition, CornerOrientation)> {
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

pub struct SetPaintColorMsg {
    pub color: FaceColor,
}

impl Reducer<CubeState> for SetPaintColorMsg {
    fn apply(&self, state: Rc<CubeState>) -> Rc<CubeState> {
        match state.cube.clone() {
            SomeCube::Cubie { cube: _ } => state,
            SomeCube::Facelet { cube, color: _ } => CubeState {
                cube: SomeCube::Facelet {
                    cube,
                    color: Some(self.color),
                },
                solution: None,
            }
            .into(),
        }
    }
}

pub struct ClickedMsg {
    pub position: FaceletPosition,
}

impl Reducer<CubeState> for ClickedMsg {
    fn apply(&self, state: Rc<CubeState>) -> Rc<CubeState> {
        match state.cube.clone() {
            SomeCube::Cubie { cube: _ } => state,
            SomeCube::Facelet { cube, color } => {
                if self.position.get_horizontal_position() == HorizontalPosition::Middle
                    && self.position.get_vertical_position() == VerticalPosition::Middle
                {
                    CubeState {
                        cube: SomeCube::Facelet {
                            cube,
                            color: Some(self.position.get_face()),
                        },
                        solution: None,
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
                        solution: None,
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
                    solution: None,
                }
                .into(),
                SomeCube::Facelet { cube, color: _ } => {
                    if let Ok(cubiecube) = CubieCube::try_from((*cube).clone()) {
                        CubeState {
                            cube: SomeCube::Cubie {
                                cube: Rc::from(cubiecube),
                            },
                            solution: None,
                        }
                        .into()
                    } else {
                        state
                    }
                }
            },
            BasicControlMsg::Reset => match state.cube.clone() {
                SomeCube::Cubie { cube: _ } => CubeState {
                    cube: SomeCube::Cubie {
                        cube: CubieCube::default().into(),
                    },
                    solution: None,
                }
                .into(),
                SomeCube::Facelet { cube: _, color: _ } => CubeState {
                    cube: SomeCube::Facelet {
                        cube: FaceletCube::default().into(),
                        color: None,
                    },
                    solution: None,
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
                        solution: None,
                    }
                    .into(),
                    SomeCube::Facelet { cube: _, color: _ } => CubeState {
                        cube: SomeCube::Facelet {
                            cube: Rc::from(FaceletCube::from(new_cube)),
                            color: None,
                        },
                        solution: None,
                    }
                    .into(),
                }
            }
            BasicControlMsg::Invert => match state.cube.clone() {
                SomeCube::Cubie { cube } => CubeState {
                    cube: SomeCube::Cubie {
                        cube: cube.invert().into(),
                    },
                    solution: None,
                }
                .into(),
                SomeCube::Facelet { cube: _, color: _ } => state,
            },
            BasicControlMsg::Clear => match state.cube.clone() {
                SomeCube::Cubie { cube: _ } => state,
                SomeCube::Facelet { cube: _, color: _ } => CubeState {
                    cube: SomeCube::Facelet {
                        cube: Rc::from(FaceletCube::CLEARED),
                        color: None,
                    },
                    solution: None,
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
            SomeCube::Cubie { cube } => {
                let solution: Option<Vec<Move>> = match &state.solution {
                    Some(vec) => match vec.split_first() {
                        Some((m1, rem)) => {
                            if self.cube.eq(m1.get_cube()) {
                                Some(rem.into_iter().cloned().collect_vec())
                            } else {
                                None
                            }
                        }
                        None => None,
                    },
                    None => None,
                };

                CubeState {
                    cube: SomeCube::Cubie {
                        cube: cube.multiply(&self.cube).into(),
                    },
                    solution,
                }
                .into()
            }

            SomeCube::Facelet { cube: _, color: _ } => state,
        }
    }
}

pub struct SolveMsg {}

impl Reducer<CubeState> for SolveMsg {
    fn apply(&self, state: Rc<CubeState>) -> Rc<CubeState> {
        match state.cube.clone() {
            SomeCube::Cubie { cube } => {
                let data_source = Dispatch::<DataState>::new().get().data.clone();

                match data_source {
                    Some(data) => {
                        let solution = Solver::get_solution(
                            cube.as_ref().clone(),
                            data,
                            SolveSettings::default(),
                        );

                        CubeState {
                            cube: SomeCube::Cubie { cube },
                            solution,
                        }
                        .into()
                    }

                    None => state,
                }
            }
            SomeCube::Facelet { cube: _, color: _ } => state,
        }
    }
}
