#![feature(const_option)]
#![feature(const_for)]
#![feature(map_try_insert)]

#![feature(generic_const_exprs)]
#![feature(once_cell)]

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(incomplete_features)]

use crate::web::prelude::*;
pub mod core;
pub mod state;
pub mod web;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
