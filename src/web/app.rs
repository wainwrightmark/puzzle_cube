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
    <ButtonsControl/>


            </div>
        }
}

#[function_component(ButtonsControl)]
pub fn buttons_control() -> Html {
    let is_cubie = *use_selector(|x: &CubeState| x.is_cubie());

    if is_cubie {
        let move_buttons = Move::iter()
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
            </div>

            <div class="row">
                {move_buttons}

            </div>
            <SymButtons/>
            <ViewButtons/>
        </div>

        )
    } else {
        let paint_buttons = FaceColor::iter()
        .map(|color| {
            html!(<PaintButton {color} />)
        })
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


#[function_component(SymButtons)]
pub fn sym_buttons() -> Html {
    html!(
        <div class="row">
                    <MoveButton cube={F2_SYMMETRY} name={"Sym: F2"} />
                    <MoveButton cube={U4_SYMMETRY} name={"Sym: U4"} />
                    <MoveButton cube={URF3_SYMMETRY} name={"Sym: URF3"} />
                    <MoveButton cube={MIRROR_LR2_SYMMETRY} name={"Sym: LR2"} />

                </div>
    )
}

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

    let onclick: Callback<MouseEvent> = Dispatch::new().apply_callback(move |_| SetPaintColorMsg{color});

    let style = format!("background: {}", color.get_color_string());

    let class = if selected{
        "size-2 col btn-small selected paint_button"
    }else{
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
    let onclick: Option<Callback<MouseEvent>> =
        Some(Dispatch::new().apply_callback(move |_| MoveMsg { cube: cube.clone() }));

    html!(<button {onclick} class="size-2 col btn-small"> {properties.name.clone()}  </button>)
}
