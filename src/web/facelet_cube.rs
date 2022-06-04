use std::rc::Rc;

use crate::core::prelude::*;
use crate::state::prelude::*;
use crate::web::prelude::*;
use chrono::format::format;
use itertools::Itertools;
use strum::IntoEnumIterator;
use yew::prelude::*;
use yewdux::prelude::*;




#[function_component(FaceletCubeView)]
pub fn facelet_cube() -> Html {
    let cube_state = use_store_value::<CubeState>().as_ref().clone();
    let view_type = use_selector(|v:&ViewState|v.view_type);

if let SomeCube::Facelet { cube, color: _ } = cube_state.cube{
    
    FaceletPosition::iter()
    .map(|position| {
        let color =cube.facelets[position as usize];
        face(color, position,*view_type )
    }).collect::<Html>()
}
else {
    Html::default()
}
    
}