mod cube_state;
mod data_state;
mod transform;
mod view_state;

pub mod prelude {

    pub use crate::state::cube_state::*;
    pub use crate::state::data_state::*;
    pub use crate::state::transform::*;
    pub use crate::state::view_state::*;

    pub const FACELETSIZE: f32 = 100.0;
    pub const FACELETSPACING: f32 = 0.0;
    pub const FACESPACING: f32 = 5.0;
}
