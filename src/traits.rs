//! Traits used to make the tile systems generic. Implement these traits to create your own custom
//! tile types.

use num::{Integer, NumCast, Signed, traits::real::Real};



/// Trait for creating different types of tile coordinate systems.
/// 
/// Planned/in development systems are square, hexagonal, and triangular
pub trait TileCoords {

	/// Creates a new instance of `Self` with coordinates corresponding to the given X and Y world positions
	fn from_position<P>(x: P, y: P) -> Self where P: NumCast + Real;

	/// Returns a [`Vec`] of coordinates that are adjacent to this set of coordinates
	fn adjacent(&self) -> Vec<Self> where Self: Sized;

	/// Returns the straight line tile distance in tiles between this and another tile
	fn tile_distance<D>(&self, other: &Self) -> D where D: Copy + Integer + Signed + NumCast;

	/// Returns the world position of the center of a tile at these coordinates
	fn center_position<P>(&self) -> (P, P) where P: NumCast + Real;

	fn width<F>() -> F where F: NumCast + Real;

	fn height<F>() -> F where F: NumCast + Real;
}
