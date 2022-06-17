use crate::core::prelude::*;
use crate::state::prelude::*;

use yew::prelude::*;
use yewdux::prelude::*;

pub fn face(color: Option<FaceColor>, facelet_position: FaceletPosition, view: ViewType) -> Html {
    let facelet_translate = get_facelet_transform(facelet_position);
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

    let all_transforms = initial_transform.into_iter()
    .chain(face_transforms.into_iter())
    .chain(std::iter:: once(TransformComponent::Translate(facelet_translate)));

    let combined_transforms = TransformComponent::combine_transforms(all_transforms);
    let transform_string = TransformComponent::get_transform_string(&combined_transforms);

    let style = format!(
        "{transform_string} width: {size}vw; height: {size}vw; transform-origin: {origin}vw {origin}vw;",
        transform_string = transform_string,
        size = FACELETSIZE.to_string(),
        origin = (FACELETSIZE * 1.5).to_string(),
    );

    html! {
        <div {class} {style} {onclick}   ></div>
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
