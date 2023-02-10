//! Traits used to make the tile systems generic. Implement these traits to create your own custom
//! tile types.

use std::fmt::Debug;

use num::NumCast;



/// Trait for creating different types of tile coordinate systems. Implement this for a struct to
/// use that struct as tile map coordinates.
pub trait TileCoords: Debug + Sized + PartialEq {

	/// Returns a [`Vec`] of coordinates that are adjacent to this set of coordinates
	fn adjacent_coords(&self) -> Vec<Self>;

	fn distance(&self, other: &Self) -> isize;

	fn line_to(&self, other: &Self) -> Vec<Self>;

	fn from_world(x: f32, y: f32) -> Self;

	/// Converts this tile coordinate into cartesian world coordinates, representing the center of
	/// the tile.
	fn to_world(&self) -> (f32, f32);
}


/// Trait used to denote tile data that can be stored in a tilemap and used for pathfinding
pub trait Tile {

	/// Returns the cost of traversing this tile. Used for pathfinding.
	/// 
	/// Default implementation returns `1`, so if your game does not need to have different movement
	/// costs for different types of tiles, then you don't need to implement this function.
	fn pathfind_cost<T>(&self) -> T where T: NumCast {
		NumCast::from(1).unwrap()
	}
}
