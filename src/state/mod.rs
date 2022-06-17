mod cube_state;
mod data_state;
mod view_state;
mod transform;

pub mod prelude {

    pub use crate::state::cube_state::*;
    pub use crate::state::data_state::*;
    pub use crate::state::view_state::*;
    pub use crate::state::transform::*;

    pub const FACELETSIZE: f64 = 6.0;
    pub const FACELETSPACING: f64 = 0.1;
    pub const FACESPACING: f64 = 0.5;
}
