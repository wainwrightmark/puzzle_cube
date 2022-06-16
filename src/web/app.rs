use std::rc::Rc;

use crate::core::prelude::*;
use crate::state::{self, prelude::*};
use crate::web::prelude::*;
use chrono::format::format;
use itertools::Itertools;
use strum::IntoEnumIterator;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let view_box = format!("0 0 {SVG_WIDTH} {SVG_HEIGHT}");
    let width = format!("{SVG_WIDTH}");
    let height = format!("{SVG_HEIGHT}");

    html! {

            <div class="paper container margin-bottom-large">
            <svg viewBox={view_box} class="cubesvg" >
            <rect x="0" y="0" {width} {height} fill="white"  />
            //<g style={"transform: rotateX(-30deg) rotateY(-45deg) rotateZ(0deg);"}>
            <CubieCubeView />
            <FaceletCubeView />
            //</g>
            </svg>
            <br/>
            <SolutionView/>
    <ButtonsControl/>


            </div>
        }
}

#[function_component(ButtonsControl)]
pub fn buttons_control() -> Html {
    let is_cubie = *use_selector(|x: &CubeState| x.is_cubie());

    if is_cubie {
        let move_buttons = Move::MOVESBYNUMBER
        .into_iter()
            .map(|my_move| {
                let cube = my_move.get_cube().clone();
                let name = format!("{}", my_move);
                html!(<MoveButton {cube} {name} />)
            })
            .collect::<Html>();




        html!(
            <div id="buttons">
            <div class="row">
            <FunctionButton name={"Reset".to_string()} msg={BasicControlMsg::Reset} />
            <FunctionButton name={"Shuffle".to_string()} msg={BasicControlMsg::Shuffle} />
            <FunctionButton name={"Invert".to_string()} msg={BasicControlMsg::Invert} />
            <FunctionButton name={"Paint".to_string()} msg={BasicControlMsg::Switch} />
            <SolveGenerateButton />

            </div>

            <div class="row">
                {move_buttons}

            </div>
            
            <ViewButtons/>
        </div>

        )
    } else {
        let paint_buttons = FaceColor::iter()
            .map(|color| html!(<PaintButton {color} />))
            .collect::<Html>();

        html!(
            <div id="buttons">
            <div class="row">
            <FunctionButton name={"Reset".to_string()} msg={BasicControlMsg::Reset} />
            <FunctionButton name={"Clear".to_string()} msg={BasicControlMsg::Clear} />
            <FunctionButton name={"Shuffle".to_string()} msg={BasicControlMsg::Shuffle} />
            <FunctionButton name={"Freeze".to_string()} msg={BasicControlMsg::Switch} />
            </div>

            <div class="row">
            {paint_buttons}
            </div>

            <ViewButtons/>
        </div>

        )
    }
}

#[function_component(SolveGenerateButton)]
pub fn solve_or_generate_button() -> Html{    
    let is_data_generated = use_selector(|x: &DataState| x.is_generated()).as_ref().clone();
    let is_solved = use_selector(|x: &CubeState| x.solution.is_some()).as_ref().clone();

    let generate: Callback<MouseEvent> = Dispatch::new().apply_callback(|_| GenerateMsg {      
    });
    let solve: Callback<MouseEvent> = Dispatch::new().apply_callback(|_| SolveMsg {
    });

    if is_data_generated{
        html!(<button class="size-2 col btn-small"  onclick={solve} disabled={is_solved} > {"Solve"} </button>)
    }
    else{
        html!(        
            <button class="size-2 col btn-small"  onclick={generate} > {"Generate Solve Data"} </button>
        )
    }

    

}

#[function_component(SolutionView)]
pub fn solution_view() -> Html{
    let solution = use_selector(|x: &CubeState| x.solution.clone()).as_ref().clone();

    match solution {
    Some(vector) => {
        let len = vector.len();
        let txt = vector.into_iter().map(|x|x.to_string()) .join(" ") + format!(" ({})", &len).as_str();
        html!(<code>{txt} </code>)
    },
    None => {
        let onclick: Option<Callback<MouseEvent>> =
        Some(Dispatch::new().apply_callback(move |_| SolveMsg{}));

    html!(<code> </code>)
    }
}
}

// #[function_component(SymButtons)]
// pub fn sym_buttons() -> Html {
//     html!(
//         <div class="row">
//                     <MoveButton cube={F2_SYMMETRY} name={"Sym: F2"} />
//                     <MoveButton cube={U4_SYMMETRY} name={"Sym: U4"} />
//                     <MoveButton cube={URF3_SYMMETRY} name={"Sym: URF3"} />
//                     <MoveButton cube={MIRROR_LR2_SYMMETRY} name={"Sym: LR2"} />

//                 </div>
//     )
// }

#[function_component(PaintButtons)]
pub fn paint_buttons() -> Html {
    html!(

        <div class="row">
                    <MoveButton cube={F2_SYMMETRY} name={"Sym: F2"} />
                    <MoveButton cube={U4_SYMMETRY} name={"Sym: U4"} />
                    <MoveButton cube={URF3_SYMMETRY} name={"Sym: URF3"} />
                    <MoveButton cube={MIRROR_LR2_SYMMETRY} name={"Sym: LR2"} />

                </div>
    )
}

#[derive(PartialEq, Eq, Properties)]
pub struct PaintButtonProperties {
    color: FaceColor,
}

#[function_component(PaintButton)]
pub fn paint_button(properties: &PaintButtonProperties) -> Html {
    let color = properties.color;
    let selected = *use_selector_with_deps(
        |state: &CubeState, c| match (*state).cube {
            SomeCube::Cubie { cube: _ } => false,
            SomeCube::Facelet { cube: _, color } => color == Some(*c),
        },
        color,
    );

    let onclick: Callback<MouseEvent> =
        Dispatch::new().apply_callback(move |_| SetPaintColorMsg { color });

    let style = format!("background: {}", color.get_color_string());

    let class = if selected {
        "size-2 col btn-small selected paint_button"
    } else {
        "size-2 col btn-small paint_button"
    };

    html!(
        <button {onclick} {class} {style} />
    )
}

#[function_component(ViewButtons)]
pub fn view_buttons() -> Html {
    let flat: Callback<MouseEvent> = Dispatch::new().apply_callback(|_| ChangeViewMsg {
        view_type: ViewType::FlatMap,
    });
    let compact: Callback<MouseEvent> = Dispatch::new().apply_callback(|_| ChangeViewMsg {
        view_type: ViewType::Compact3D,
    });
    let explode: Callback<MouseEvent> = Dispatch::new().apply_callback(|_| ChangeViewMsg {
        view_type: ViewType::Exploded3D,
    });

    html!(
        <div class="row">
        <button onclick={flat} > {"Flat"} </button>
        <button onclick={compact} > {"Compact"} </button>
        <button onclick={explode} > {"Explode"} </button>
                </div>
    )
}

#[derive(PartialEq, Eq, Properties)]
pub struct FunctionButtonProperties {
    pub name: String,
    pub msg: BasicControlMsg,
}

#[function_component(FunctionButton)]
pub fn function_button(properties: &FunctionButtonProperties) -> Html {
    let msg = properties.msg;
    let onclick: Callback<MouseEvent> = Dispatch::new().apply_callback(move |_| msg);

    html!(<button {onclick} class="size-2 col btn-small" > {properties.name.clone()} </button>)
}

#[derive(PartialEq, Eq, Properties)]
pub struct MoveButtonProperties {
    pub cube: CubieCube,
    pub name: String,
}

#[function_component(MoveButton)]
fn move_button(properties: &MoveButtonProperties) -> Html {
    let cube = properties.cube.clone();

    let is_highlighted = use_selector_with_deps(|state: &CubeState, c| match &state.solution {
    Some(moves) => match moves.first(){
    Some(m) => m.get_cube() == c,
    None => false,
},
    None => false,
} , cube.clone());

    let onclick: Option<Callback<MouseEvent>> =
        Some(Dispatch::new().apply_callback(move |_| MoveMsg { cube: cube.clone() }));
        let extra_class = if *is_highlighted {Some("btn-success")} else {None};

        let class = classes!("size-2", "col", "btn-small", extra_class );

    html!(<button {onclick} {class}> {properties.name.clone()}  </button>)
}
