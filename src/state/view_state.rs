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





impl ViewType {
    pub fn get_initial_transform(&self) -> Vec<TransformComponent> {
        match self {
            ViewType::FlatMap => Default::default(),
            _ => vec![TransformComponent::Rotate(TransformRotate{x: -30, y:-45, }) ,TransformComponent::Translate( TransformTranslate{x: 40.0,y: 5.0,z: 0.0})]
        }
    }

    pub fn get_face_transform(&self, face: FaceColor) ->Vec<TransformComponent> {
        match self {
            ViewType::FlatMap => {

                let hf = face.get_x() as f64;
                let vf = face.get_y() as f64;
                let x: f64 = ((FACELETSIZE + FACELETSPACING) * hf * 3.0) + (hf * FACESPACING);
                let y: f64 = ((FACELETSIZE + FACELETSPACING) * vf * 3.0) + (vf * FACESPACING);

                vec![TransformComponent::Translate(TransformTranslate {
                    x,
                    y,
                    ..Default::default()
                })]
                
            }
            ViewType::Compact3D => {
                let d = (FACELETSIZE) * 1.5;
                let tuple =
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
                };
                vec![TransformComponent::Rotate(tuple.0),TransformComponent::Translate(tuple.1) ]
            }
            ViewType::Exploded3D => {
                let d1 = (FACELETSIZE) * 1.5;
                let d2 = (FACELETSIZE) * 6.0;
                let tuple =
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
                };
                vec![TransformComponent::Rotate(tuple.0),TransformComponent::Translate(tuple.1) ]
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
