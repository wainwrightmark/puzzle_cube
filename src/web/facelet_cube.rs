use std::rc::Rc;

use crate::core::prelude::*;
use crate::state::prelude::*;
use crate::web::prelude::*;
use chrono::format::format;
use itertools::Itertools;
use strum::IntoEnumIterator;
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(PartialEq, Eq, Properties)]
pub struct FaceletCubeProperties{
    pub cube: Rc<FaceletCube>
}

#[function_component(FaceletCubeView)]
pub fn facelet_cube(properties: &FaceletCubeProperties) -> Html {
    let view_type = use_selector(|v:&ViewState|v.view_type);
    let stuff =
    FaceletPosition::iter()
    .map(|position| {
        let color =properties.cube.facelets[position as usize];
        face(color, position,*view_type )
    }).collect::<Html>();
    html!({stuff})
}