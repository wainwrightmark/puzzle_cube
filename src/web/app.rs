use crate::core::prelude::*;
use crate::state::prelude::*;
use crate::web::prelude::*;
use chrono::format::format;
use itertools::Itertools;
use strum::IntoEnumIterator;
use yew::prelude::*;
use yewdux::prelude::*;

const FACELETSIZE: f64 = 10.0;
const FACELETSPACING: f64 = 1.0;
const FACESPACING: f64 = 1.0;
const SVG_WIDTH: f64 = (FACELETSIZE + FACESPACING) * 12.0 + (FACESPACING * 3.0);
const SVG_HEIGHT: f64 = (FACELETSIZE + FACESPACING) * 9.0 + (FACESPACING * 2.0);

#[function_component(App)]
pub fn app() -> Html {
    let cube = //CubieCube::default();

    CubieCube::random_cube(12345);

    let view_box = format!("0 0 {SVG_WIDTH} {SVG_HEIGHT}");
    let width = format!("{SVG_WIDTH}");
    let height = format!("{SVG_HEIGHT}");

    let move_buttons = Move::iter()
        .map(|my_move| html!(<MoveButton  {my_move} />))
        .collect::<Html>();

    html! {

        <div class="paper container margin-bottom-large">
        <svg viewBox={view_box} class="cubesvg" >
        <rect x="0" y="0" {width} {height} fill="white"  />
        <Cube />
        </svg>

        <div id="buttons">
            <div class="row">
            <RandomizeButton/><ResetButton/><InvertButton/>
            </div>
            <div class="row">
        {move_buttons}
        </div>
        </div>

        </div>
    }
}

#[function_component(RandomizeButton)]
pub fn randomize_button() -> Html {
    let onclick: Option<Callback<MouseEvent>> =
        Some(Dispatch::new().apply_callback(|_| RandomizeMsg {}));

    html!(<button {onclick} > {"Random"} </button>)
}

#[function_component(ResetButton)]
pub fn reset_button() -> Html {
    let onclick: Option<Callback<MouseEvent>> =
        Some(Dispatch::new().apply_callback(|_| ResetMsg {}));

    html!(<button {onclick} > {"Reset"} </button>)
}


#[function_component(InvertButton)]
pub fn invert_button() -> Html {
    let onclick: Option<Callback<MouseEvent>> =
        Some(Dispatch::new().apply_callback(|_| InvertMsg {}));

    html!(<button {onclick} > {"Invert"} </button>)
}

#[derive(PartialEq, Eq, Properties)]
pub struct MoveButtonProperties {
    pub my_move: Move,
}

#[function_component(MoveButton)]
fn move_button(properties: &MoveButtonProperties) -> Html {
    let my_move = properties.my_move;
    let onclick: Option<Callback<MouseEvent>> =
        Some(Dispatch::new().apply_callback(move |_| MoveMsg { my_move }));

    html!(<button {onclick} class="size-4 col btn-small"> {my_move.to_string()}  </button>)
}

#[function_component(Cube)]
pub fn cube() -> Html {

    let centres = FaceColor::iter()
        .map(|face| html!(<Centre {face} />))
        .collect::<Html>();
    let edges = EdgePosition::iter()
        .map(|edge| html!(<Edge {edge} />))
        .collect::<Html>();
    let corners = CornerPosition::DEFAULT_ARRAY.into_iter()
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
    let edge = properties.edge;
    let position_index = *use_selector_with_deps(|x: &CubeState, &p|x.cube.edge_positions.into_iter().position(|x| x == p).unwrap(), edge);
    let position = EdgePosition::from_repr(position_index as u8).unwrap();

    let orientation = *use_selector_with_deps(|x: &CubeState, &p|x.cube.edge_orientations[p], position_index);

    let position0 = position.get_location(0, orientation);
    let color0 = edge.get_color(0);

    let position1 = position.get_location(1, orientation);
    let color1 = edge.get_color(1);

    html!(
        <>
             {face(color0, position0)}
             {face(color1, position1)}
        </>
    )
}

#[derive(PartialEq, Eq, Properties)]
pub struct CornerProperties {
    //pub corner: CornerPosition,
    pub corner: CornerPosition,
    //pub orientation: CornerOrientation,
}

#[function_component(Corner)]
fn corner(properties: &CornerProperties) -> Html {

    let corner = properties.corner;
    let position_index = *use_selector_with_deps(|x: &CubeState, &p|x.cube.corner_positions.into_iter().position(|x| x == p).unwrap(), corner);
    let position = CornerPosition::from_repr(position_index as u8).unwrap();

    let orientation = *use_selector_with_deps(|x: &CubeState, &p|x.cube.corner_orientations[p], position_index);

    let position0 = position.get_location(0, orientation);
    let color0 = corner.get_color(0);

    let position1 = position.get_location(1, orientation);
    let color1 = corner.get_color(1);
    
    let position2 = position.get_location(2, orientation);
    let color2 = corner.get_color(2);

    html!(
        <>
             {face(color0, position0)}
             {face(color1, position1)}
             {face(color2, position2)}
        </>
    )


    // let position0 = properties.position.get_location(0, properties.orientation);
    // let sposition0 = properties.corner.get_location(0, properties.orientation);
    // let color0 = properties.corner.get_color(0);

    // let position1 = properties.position.get_location(1, properties.orientation);
    // let sposition1 = properties.corner.get_location(1, properties.orientation);
    // let color1 = properties.corner.get_color(1);

    // let position2 = properties.position.get_location(2, properties.orientation);
    // let sposition2 = properties.corner.get_location(2, properties.orientation);
    // let color2 = properties.corner.get_color(2);

    // html!(
    //     <g id={properties.corner.to_string()} key={properties.corner.to_string()}>
    //         <Face color={color0} facelet_position={position0} solved_position={sposition0} />
    //         <Face color={color1} facelet_position={position1} solved_position={sposition1} />
    //         <Face color={color2} facelet_position={position2} solved_position={sposition2} />
    //     </g>


    // )
}

#[derive(PartialEq, Eq, Properties)]
pub struct CenterProperties {
    pub face: FaceColor,
}

#[function_component(Centre)]
fn centre(properties: &CenterProperties) -> Html {
    let facelet_position = FaceletPosition::from((
        properties.face,
        HorizontalPosition::Middle,
        VerticalPosition::Middle,
    ));

    face(properties.face, facelet_position)    
}



fn face(color: FaceColor,
    facelet_position: FaceletPosition,
) -> Html {
    let hp = (facelet_position.get_horizontal_position() as usize) as f64;
    let hf = facelet_position.get_face().get_x() as f64;

    let x: f64 =
        FACELETSIZE * (hp + (hf * 3.0)) + ((hp + hf * 3.0) * FACELETSPACING) + (hf * FACESPACING);

    let vp = (facelet_position.get_vertical_position() as usize) as f64;
    let vf = facelet_position.get_face().get_y() as f64;

    let y: f64 =
        FACELETSIZE * (vp + (vf * 3.0)) + ((vp + vf * 3.0) * FACELETSPACING) + (vf * FACESPACING);

    let color_class = format!("color-{}", color);
    let class = classes!("face", color_class);

    let style = format!("--xpos: {x}px; --ypos: {y}px;");

    html! {
        <rect {class} {style} width={FACELETSIZE.to_string()} height={FACELETSIZE.to_string()} rx="1" ></rect>
    }
}
