//! Traits used to make the tile systems generic. Implement these traits to create your own custom
//! tile types.

use num::{Integer, NumCast, Signed};



/// Trait for creating different types of tile coordinate systems.
/// 
/// Planned/in development systems are square, hexagonal, and triangular
pub trait TileCoords {

	/// Returns a [`Vec`] of coordinates that are adjacent to this set of coordinates
	fn adjacent(&self) -> Vec<Self> where Self: Sized;

	/// Returns the straight line tile distance in tiles between this and another tile
	fn tile_distance<D>(&self, other: &Self) -> D where D: Copy + Integer + Signed + NumCast;
}
