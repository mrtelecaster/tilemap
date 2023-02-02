//! Traits used to make the tile systems generic. Implement these traits to create your own custom
//! tile types.



/// Trait for creating different types of tile coordinate systems.
/// 
/// Planned/in development systems are square, hexagonal, and triangular
pub trait TileCoords {

	/// Returns a [`Vec`] of coordinates that are adjacent to this set of coordinates
	fn adjacent(&self) -> Vec<Self> where Self: Sized;

	/// Returns the continuous, straight line distance between this tile and the other 
	fn distance<D>(&self, other: &Self) -> D;
}
