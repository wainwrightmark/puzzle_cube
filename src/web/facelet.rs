use crate::core::prelude::*;
use crate::state::prelude::*;

use yew::prelude::*;
use yewdux::prelude::*;

pub fn face(color: Option<FaceColor>, facelet_position: FaceletPosition, view: ViewType) -> Html {
    let facelet_transform = get_facelet_transform(facelet_position);
    let face_transforms = view.get_face_transform(facelet_position.get_face());

    let onclick: Callback<MouseEvent> = Dispatch::new().apply_callback(move |_| ClickedMsg {
        position: facelet_position,
    });

    let color_class = if let Some(c) = color {
        format!("color-{}", c)
    } else {
        "color-unknown".to_string()
    };
    let class = classes!("face", color_class);
    let initial_transform = view.get_initial_transform();

    let all_transforms = initial_transform
        .into_iter()
        .chain(face_transforms.into_iter())
        .chain(std::iter::once(facelet_transform));

    let combined_transforms = Transform::combine_transforms(all_transforms);
    let style = Transform::get_transform_string(&combined_transforms, &"%".to_string());

    html! {
        <div {class} {style} {onclick}   ></div>
    }
}

fn get_facelet_transform(facelet_position: FaceletPosition) -> Transform {
    let hp = (facelet_position.get_horizontal_position() as usize) as f32;
    let x: f32 = hp * (FACELETSIZE + FACELETSPACING);

    let vp = (facelet_position.get_vertical_position() as usize) as f32;
    let y: f32 = vp * (FACELETSIZE + FACELETSPACING);

    Transform::Translate { x, y }
}
