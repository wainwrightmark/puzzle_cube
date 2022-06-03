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
pub struct CubieCubeProperties{
    pub cube: Rc<CubieCube>,
}

#[function_component(CubieCubeView)]
pub fn cubie_cube(properties: &CubieCubeProperties) -> Html {
    let centres = FaceColor::iter()
        .map(|face| html!(<Centre {face} />))
        .collect::<Html>();
    let edges = EdgePosition::iter()
        .map(|edge| html!(<Edge {edge} cube={properties.cube.clone()}  />))
        .collect::<Html>();
    let corners = CornerPosition::DEFAULT_ARRAY
        .into_iter()
        .map(|corner| html!(<Corner {corner} cube={properties.cube.clone()} />))
        .collect::<Html>();

    html!(
    <>
    {centres}
    {edges}
    {corners}
    </>
        )
}

#[derive(PartialEq, Eq, Properties)]
pub struct EdgeProperties {
    pub edge: EdgePosition,
    pub cube: Rc<CubieCube>
}

#[function_component(Edge)]
fn edge(properties: &EdgeProperties) -> Html {
    let edge = properties.edge;
    let cube = properties.cube.clone();
    let position_index = cube
    .edge_positions
    .into_iter()
    .position(|x| x == edge)
    .unwrap();
let view = use_selector(|s:&ViewState|s.view_type);

    let position = EdgePosition::from_repr(position_index as u8).unwrap();

    let orientation = cube.edge_orientations[position_index];

    let position0 = position.get_location(0, orientation);
    let color0 = Some(edge.get_color(0)) ;

    let position1 = position.get_location(1, orientation);
    let color1 = Some( edge.get_color(1));

    html!(
        <>
             {face(color0, position0, *view)}
             {face(color1, position1, *view)}
        </>
    )
}

#[derive(PartialEq, Eq, Properties)]
pub struct CornerProperties {
    pub corner: CornerPosition,
    pub cube: Rc<CubieCube>
}

#[function_component(Corner)]
fn corner(properties: &CornerProperties) -> Html {
    let view = use_selector(|s:&ViewState|s.view_type);
    let cube = properties.cube.clone();
    let corner = properties.corner;
    let position_index = cube
    .corner_positions
    .into_iter()
    .position(|x| x == corner)
    .unwrap();
    let position = CornerPosition::from_repr(position_index as u8).unwrap();

    let orientation = cube.corner_orientations[position_index];

    let position0 = position.get_location(0, orientation);
    let color0 = Some(corner.get_color(0)) ;

    let position1 = position.get_location(1, orientation);
    let color1 = Some(corner.get_color(1));

    let position2 = position.get_location(2, orientation);
    let color2 = Some(corner.get_color(2));

    html!(
        <>
             {face(color0, position0, *view)}
             {face(color1, position1, *view)}
             {face(color2, position2, *view)}
        </>
    )
}

#[derive(PartialEq, Eq, Properties)]
pub struct CenterProperties {
    pub face: FaceColor,
}

#[function_component(Centre)]
fn centre(properties: &CenterProperties) -> Html {
    let view = use_selector(|s:&ViewState|s.view_type);
    let facelet_position = FaceletPosition::from((
        properties.face,
        HorizontalPosition::Middle,
        VerticalPosition::Middle,
    ));

    face(Some(properties.face), facelet_position, *view)
}
