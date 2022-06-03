use std::rc::Rc;

use crate::core::prelude::*;
use crate::state::prelude::*;
use crate::web::prelude::*;
use chrono::format::format;
use itertools::Itertools;
use strum::IntoEnumIterator;
use yew::prelude::*;
use yewdux::prelude::*;

pub fn face(color: Option<FaceColor>, facelet_position: FaceletPosition, view: ViewType) -> Html {
    let facelet_translate = get_facelet_transform(facelet_position);
    let (face_rotate, face_translate) = view.get_face_transform(facelet_position.get_face());

let onclick: Callback<MouseEvent> =Dispatch::new().apply_callback(move |_| ClickedMsg {position: facelet_position});

    let color_class = if let Some(c) = color {
        format!("color-{}", c)
    } else {
        "color-unknown".to_string()
    };
    let class = classes!("face", color_class);

    let style = format!(
        "transform: {} {} {};",
        face_translate.get_text(),
        face_rotate.get_text(),
        facelet_translate.get_text()
    );

    html! {
        <rect {class} {style} {onclick} width={FACELETSIZE.to_string()} height={FACELETSIZE.to_string()} rx="1" ></rect>
    }
}

fn get_facelet_transform(facelet_position: FaceletPosition) -> TransformTranslate {
    let hp = (facelet_position.get_horizontal_position() as usize) as f64;
    let x: f64 = hp * (FACELETSIZE + FACELETSPACING);

    let vp = (facelet_position.get_vertical_position() as usize) as f64;
    let y: f64 = vp * (FACELETSIZE + FACELETSPACING);

    TransformTranslate {
        x,
        y,
        ..Default::default()
    }
}
