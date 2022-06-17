use crate::core::prelude::*;
use crate::state::prelude::*;

use serde::*;
use std::rc::Rc;
use yewdux::{storage, prelude::*};

#[derive(PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
pub struct ViewState {
    pub view_type: ViewType,
}

impl Store for ViewState {
    fn new() -> Self {
        init_listener(storage::StorageListener::<Self>::new(storage::Area::Local));

        storage::load(storage::Area::Local)
            .expect("Unable to load state")
            .unwrap_or_default()
    }

    fn changed(&self, other: &Self) -> bool {
        self != other
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Default, Serialize, Deserialize)]
pub enum ViewType {
    #[default]
    FlatMap,
    Compact3D,
    Exploded3D,
}

impl ViewType {
    pub fn get_initial_transform(&self) -> Vec<Transform> {
        match self {
            ViewType::FlatMap => Default::default(),
            _ => vec![Transform::RotateX(-30.0), Transform::RotateY(-45.0), Transform::Translate { x: 750.0, y: 40.0 }]            
        }
    }

    pub fn get_face_transform(&self, face: FaceColor) ->Vec<Transform> {
        match self {
            ViewType::FlatMap => {

                let hf = face.get_x() as f32;
                let vf = face.get_y() as f32;
                let x: f32 = ((FACELETSIZE + FACELETSPACING) * hf * 3.0) + (hf * FACESPACING);
                let y: f32 = ((FACELETSIZE + FACELETSPACING) * vf * 3.0) + (vf * FACESPACING);

                vec![Transform::Translate{x,y}]
                
            }
            ViewType::Compact3D => {
                let d = FACELETSIZE * 1.5;
                let tuple =
                match face {
                    FaceColor::Up => (
                        Transform::RotateY(-90.0),
                        Transform::Translate {
                            x: 0.0,
                            y: -d,
                        },
                        Transform::RotateY(90.0),
                        Transform::RotateX(90.0),                                                                
                    ),
                    FaceColor::Left => (
                        Transform::RotateZ(-90.0),
                        Transform::Translate {
                            x: 0.0,
                            y: -d,
                        },
                        Transform::RotateZ(90.0),
                        Transform::RotateY(-90.0),
                    ),
                    FaceColor::Front => (
                        Transform::RotateX(-90.0),
                        Transform::Translate {
                            x: 0.0,
                            y: -d,
                        },
                        Transform::RotateX(90.0),
                        Transform::RotateY(0.0),
                    ),
                    FaceColor::Right => (
                        Transform::RotateZ(-90.0),
                        Transform::Translate {
                            x: 0.0,
                            y: d,
                        },
                        Transform::RotateZ(90.0),
                        Transform::RotateY(90.0),
                    ),
                    FaceColor::Back => (
                        Transform::RotateX(-90.0),
                        Transform::Translate {
                            x: 0.0,
                            y: d,
                        },
                        Transform::RotateX(90.0),
                        Transform::RotateY(180.0),
                    ),
                    FaceColor::Down => (
                        Transform::RotateY(-90.0),
                        Transform::Translate {
                            x: 0.0,
                            y: d,
                        },
                        Transform::RotateY(90.0),
                        Transform::RotateX(-90.0),
                    ),
                };
                vec![tuple.0, tuple.1, tuple.2, tuple.3]
                
            }
            ViewType::Exploded3D => {
                let d1 = FACELETSIZE * 1.5;
                let d2 = FACELETSIZE * 6.0;
                let tuple =
                match face {
                    FaceColor::Up => (
                        Transform::RotateY(-90.0),
                        Transform::Translate {
                            x: 0.0,
                            y: -d1,
                        },
                        Transform::RotateY(90.0),
                        Transform::RotateX(90.0),                                                                
                    ),
                    FaceColor::Left => (
                        Transform::RotateZ(-90.0),
                        Transform::Translate {
                            x: 0.0,
                            y: -d2,
                        },
                        Transform::RotateZ(90.0),
                        Transform::RotateY(-90.0),
                    ),
                    FaceColor::Front => (
                        Transform::RotateX(-90.0),
                        Transform::Translate {
                            x: 0.0,
                            y: -d1,
                        },
                        Transform::RotateX(90.0),
                        Transform::RotateY(0.0),
                    ),
                    FaceColor::Right => (
                        Transform::RotateZ(-90.0),
                        Transform::Translate {
                            x: 0.0,
                            y: d1,
                        },
                        Transform::RotateZ(90.0),
                        Transform::RotateY(90.0),
                    ),
                    FaceColor::Back => (
                        Transform::RotateX(-90.0),
                        Transform::Translate {
                            x: 0.0,
                            y: d2,
                        },
                        Transform::RotateX(90.0),
                        Transform::RotateY(180.0),
                    ),
                    FaceColor::Down => (
                        Transform::RotateY(-90.0),
                        Transform::Translate {
                            x: 0.0,
                            y: d2,
                        },
                        Transform::RotateY(90.0),
                        Transform::RotateX(-90.0),
                    ),
                };
                vec![tuple.0, tuple.1, tuple.2, tuple.3]
            }
        }
    }
}

pub struct ChangeViewMsg {
    pub view_type: ViewType,
}

impl Reducer<ViewState> for ChangeViewMsg {
    fn apply(&self, _state: Rc<ViewState>) -> Rc<ViewState> {
        ViewState {
            view_type: self.view_type,
        }
        .into()
    }
}
