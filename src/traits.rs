//! Traits used to make the tile systems generic. Implement these traits to create your own custom
//! tile types.

use std::fmt::Debug;
use num::Integer;



/// Trait for creating different types of tile coordinate systems. Implement this for a struct to
/// use that struct as tile map coordinates.
pub trait TileCoords<T>: Debug + Sized + PartialEq {

	/// Returns a [`Vec`] of coordinates that are adjacent to this set of coordinates
	fn adjacent_coords(&self) -> Vec<Self>;

	fn distance<D>(&self, other: &Self) -> D where D: Integer + From<T>;
}
