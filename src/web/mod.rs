mod app;
mod cubie_cube;
mod facelet;
mod facelet_cube;

pub mod prelude {

    pub use crate::web::app::*;
    pub use crate::web::cubie_cube::*;
    pub use crate::web::facelet::*;
    pub use crate::web::facelet_cube::*;
}
