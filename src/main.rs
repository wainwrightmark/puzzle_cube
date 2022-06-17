#![feature(const_option)]
#![feature(map_try_insert)]





use crate::web::prelude::*;
pub mod core;
pub mod state;
pub mod web;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
