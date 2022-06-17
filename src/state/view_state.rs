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
}

#[derive(PartialEq, Eq, Copy, Clone, Default, Serialize, Deserialize)]
pub enum ViewType {
    #[default]
    FlatMap,
    Compact3D,
    Exploded3D,
}

#[derive(PartialEq, Copy, Clone, Default, Serialize, Deserialize)]
pub struct TransformTranslate {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl TransformTranslate {
    pub fn get_text(self) -> String {
        format!("translate3d({}vw,{}vw,{}vw)", self.x, self.y, self.z)
    }
}

impl TransformRotate {
    pub fn get_text(self) -> String {
        if self.x != 0 {
            if self.y != 0 {
                format!("rotateX({}deg) rotateY({}deg)", self.x, self.y)
            } else {
                format!("rotateX({}deg)", self.x)
            }
        } else if self.y != 0 {
            format!("rotateY({}deg)", self.y)
        } else {
            "".to_string()
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Default, Serialize, Deserialize)]
pub struct TransformRotate {
    pub x: i32,
    pub y: i32,
}

impl ViewType {
    pub fn get_initial_transform(&self) -> String {
        match self {
            ViewType::FlatMap => "".to_string(),
            ViewType::Compact3D => {
                " rotateX(-30deg) rotateY(-45deg) rotateZ(0deg) translate3d(40vw, 5vw, 0)"
                    .to_string()
            }
            ViewType::Exploded3D => {
                " rotateX(-30deg) rotateY(-45deg) rotateZ(0deg) translate3d(40vw, 5vw, 0)"
                    .to_string()
            }
        }
    }

    pub fn get_face_transform(&self, face: FaceColor) -> (TransformRotate, TransformTranslate) {
        match self {
            ViewType::FlatMap => {
                let hf = face.get_x() as f64;
                let vf = face.get_y() as f64;
                let x: f64 = ((FACELETSIZE + FACELETSPACING) * hf * 3.0) + (hf * FACESPACING);
                let y: f64 = ((FACELETSIZE + FACELETSPACING) * vf * 3.0) + (vf * FACESPACING);

                (
                    TransformRotate::default(),
                    TransformTranslate {
                        x,
                        y,
                        ..Default::default()
                    },
                )
            }
            ViewType::Compact3D => {
                let d = (FACELETSIZE) * 1.5;

                match face {
                    FaceColor::Up => (
                        TransformRotate { x: 90, y: 0 },
                        TransformTranslate {
                            x: 0.0,
                            y: 0.0,
                            z: d,
                        },
                    ),
                    FaceColor::Left => (
                        TransformRotate { x: 0, y: -90 },
                        TransformTranslate {
                            x: 0.0,
                            y: 0.0,
                            z: d,
                        },
                    ),
                    FaceColor::Front => (
                        TransformRotate { x: 0, y: 0 },
                        TransformTranslate {
                            x: 0.0,
                            y: 0.0,
                            z: d,
                        },
                    ),
                    FaceColor::Right => (
                        TransformRotate { x: 0, y: 90 },
                        TransformTranslate {
                            x: 0.0,
                            y: 0.0,
                            z: d,
                        },
                    ),
                    FaceColor::Back => (
                        TransformRotate { x: 0, y: 180 },
                        TransformTranslate {
                            x: 0.0,
                            y: 0.0,
                            z: d,
                        },
                    ),
                    FaceColor::Down => (
                        TransformRotate { x: -90, y: 0 },
                        TransformTranslate {
                            x: 0.0,
                            y: 0.0,
                            z: d,
                        },
                    ),
                }
            }
            ViewType::Exploded3D => {
                let d1 = (FACELETSIZE) * 1.5;
                let d2 = (FACELETSIZE) * 6.0;
                match face {
                    FaceColor::Up => (
                        TransformRotate { x: 90, y: 0 },
                        TransformTranslate {
                            x: 0.0,
                            y: 0.0,
                            z: d1,
                        },
                    ),
                    FaceColor::Left => (
                        TransformRotate { x: 0, y: -90 },
                        TransformTranslate {
                            x: 0.0,
                            y: 0.0,
                            z: d2,
                        },
                    ),
                    FaceColor::Front => (
                        TransformRotate { x: 0, y: 0 },
                        TransformTranslate {
                            x: 0.0,
                            y: 0.0,
                            z: d1,
                        },
                    ),
                    FaceColor::Right => (
                        TransformRotate { x: 0, y: 90 },
                        TransformTranslate {
                            x: 0.0,
                            y: 0.0,
                            z: d1,
                        },
                    ),
                    FaceColor::Back => (
                        TransformRotate { x: 0, y: 180 },
                        TransformTranslate {
                            x: 0.0,
                            y: 0.0,
                            z: d2,
                        },
                    ),
                    FaceColor::Down => (
                        TransformRotate { x: -90, y: 0 },
                        TransformTranslate {
                            x: 0.0,
                            y: 0.0,
                            z: d2,
                        },
                    ),
                }
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
