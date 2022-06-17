mod cube_state;
mod view_state;
mod data_state;


pub mod prelude {

    pub use crate::state::view_state::*;
    pub use crate::state::cube_state::*;
    pub use crate::state::data_state::*;

    pub const FACELETSIZE: f64 = 6.0;
    pub const FACELETSPACING: f64 = 0.1;
    pub const FACESPACING: f64 = 0.5;
 
}
