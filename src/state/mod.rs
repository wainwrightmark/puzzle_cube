mod cube_state;
mod view_state;
mod data_state;


pub mod prelude {

    pub use crate::state::view_state::*;
    pub use crate::state::cube_state::*;
    pub use crate::state::data_state::*;

    pub const FACELETSIZE: f64 = 10.0;
    pub const FACELETSPACING: f64 = 1.0;
    pub const FACESPACING: f64 = 1.0;
    pub const SVG_WIDTH: f64 = (FACELETSIZE + FACESPACING) * 12.0 + (FACESPACING * 3.0);
    pub const SVG_HEIGHT: f64 = (FACELETSIZE + FACESPACING) * 9.0 + (FACESPACING * 2.0);
}
