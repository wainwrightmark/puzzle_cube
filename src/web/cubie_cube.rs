use crate::core::prelude::*;
use crate::state::prelude::*;
use crate::web::prelude::*;

use strum::IntoEnumIterator;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(CubieCubeView)]
pub fn cubie_cube() -> Html {
    let centres = FaceColor::iter()
        .map(|face| html!(<Centre {face} />))
        .collect::<Html>();
    let edges = EdgePosition::iter()
        .map(|edge| html!(<Edge {edge} />))
        .collect::<Html>();
    let corners = CornerPosition::DEFAULT_ARRAY
        .into_iter()
        .map(|corner| html!(<Corner {corner} />))
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
}

#[function_component(Edge)]
fn edge(properties: &EdgeProperties) -> Html {
    let view = use_selector(|s: &ViewState| s.view_type);
    let edge = properties.edge;
    let option = *use_selector_with_deps(
        |s: &CubeState, edge| s.try_get_edge_position(edge),
        properties.edge,
    )
    .as_ref();

    if let Some((position, orientation)) = option {
        let position0 = position.get_location(0, orientation);
        let color0 = Some(edge.get_color(0));

        let position1 = position.get_location(1, orientation);
        let color1 = Some(edge.get_color(1));

        html!(
            <>
            {face(color0, position0, *view)}
            {face(color1, position1, *view)}
            </>
        )
    } else {
        Html::default()
    }
}

#[derive(PartialEq, Eq, Properties)]
pub struct CornerProperties {
    pub corner: CornerPosition,
}

#[function_component(Corner)]
fn corner(properties: &CornerProperties) -> Html {
    let view = use_selector(|s: &ViewState| s.view_type);
    let _some_cube = use_selector(|s: &CubeState| s.cube.clone());
    let corner = properties.corner;
    let option = *use_selector_with_deps(
        |s: &CubeState, corner| s.try_get_corner_position(corner),
        properties.corner,
    )
    .as_ref();

    if let Some((position, orientation)) = option {
        let position0 = position.get_location(0, orientation);
        let color0 = Some(corner.get_color(0));

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
    } else {
        Html::default()
    }
}

#[derive(PartialEq, Eq, Properties)]
pub struct CenterProperties {
    pub face: FaceColor,
}

#[function_component(Centre)]
fn centre(properties: &CenterProperties) -> Html {
    let view = use_selector(|s: &ViewState| s.view_type);
    let is_cubie = *use_selector(|x: &CubeState| x.is_cubie());
    if is_cubie {
        let facelet_position = FaceletPosition::from((
            properties.face,
            HorizontalPosition::Middle,
            VerticalPosition::Middle,
        ));
        face(Some(properties.face), facelet_position, *view)
    } else {
        Html::default()
    }
}
