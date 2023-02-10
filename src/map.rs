//! Tilemap related things

use std::{collections::HashMap, hash::Hash};
use crate::hex::AxialCoords;


// TILEMAP STRUCT ------------------------------------------------------------------------------- //

/// A structure that can hold a map of tiles at arbitrary coordinates
pub struct TileMap<C, T>
{
	map: HashMap<C, T>,
}

impl<C, T> TileMap<C, T>
{
	/// Creates a new `TileMap` with no tiles
	pub fn new() -> Self
	{
		Self{ map: HashMap::new() }
	}

	/// Gets the tile that's at the given coordinates. If there is no tile at those coordinates,
	/// `None` is returned.
	pub fn get_tile(&self, coord: &C) -> Option<&T> where C: Eq + Hash
	{
		self.map.get(coord)
	}

	/// Insert a new tile into the map at the given coordinates.
	/// 
	/// If there is already a tile there, it will be replaced by the new tile, with the old tile
	/// data returned by the function.
	pub fn insert_tile(&mut self, coord: C, tile: T) -> Option<T> where C: Eq + Hash
	{
		self.map.insert(coord, tile)
	}
}


// MAP ALIASES ---------------------------------------------------------------------------------- //

/// Tile map using hexagonal coordinates
pub type HexMap<T> = TileMap<AxialCoords, T>;
