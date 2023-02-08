//! Pre-made types for hexagonal coordinate systems
//! 
//! Made referencing the fantastic [*Hexagonal Grids* article](https://www.redblobgames.com/grids/hexagons)
//! at [Red Blob Games](https://www.redblobgames.com/)

pub mod axial; pub use axial::AxialCoords;
pub mod cube; pub use cube::CubeCoords;
pub mod offset; pub use offset::OffsetCoords;
pub mod util;
