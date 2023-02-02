//! Pre-made types for hexagonal coordinate systems
//! 
//! Made referencing the fantastic [*Hexagonal Grids* article](https://www.redblobgames.com/grids/hexagons)
//! at [Red Blob Games](https://www.redblobgames.com/)



/// Axial hexagon coordinate system
pub struct AxialCoords<T> {
	pub q: T,
	pub r: T,
}