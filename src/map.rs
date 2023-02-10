//! Tilemap related things

use std::{collections::HashMap, hash::Hash};
use crate::hex::AxialCoords;


pub struct TileMap<C, T>
{
	map: HashMap<C, T>,
}

impl<C, T> TileMap<C, T>
{
	pub fn new() -> Self {
		Self{ map: HashMap::new() }
	}

	pub fn get_tile(&self, coord: &C) -> Option<&T> where C: Eq + Hash {
		self.map.get(coord)
	}

	pub fn set_tile(&mut self, coord: C, tile: T) where C: Eq + Hash {
		self.map.insert(coord, tile);
	}
}



/// Tile map using hexagonal coordinates
pub type HexMap<T> = TileMap<AxialCoords, T>;