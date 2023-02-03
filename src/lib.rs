//! # Tilemap
//!
//! Generic tile maps library for use developing games. This is meant for more of a board game map
//! where tiles represent spaces with different attributes that game pieces move through. It's
//! intended primarily for strategy games but can be adapted to other needs.
//! 
//! I am developing this for [a Bevy project] of mine, but now that this has been split out into its
//! own library I would like to keep it as engine-agnostic as possible. However, the way it's
//! structured will still probably favor Bevy development (for example, the default coordinate
//! directions), so keep that in mind if using this for another engine.

pub mod traits;
pub mod hex;
pub mod square;

use std::{
	collections::HashMap,
	hash::Hash,
};



pub struct TileMap<C, T> where C: traits::TileCoords {
	map: HashMap<C, T>,
}

impl<C, T> TileMap<C, T> where C: traits::TileCoords + Eq + Hash {

	pub fn contains_tile_at(&self, coords: &C) -> bool {
		self.map.contains_key(coords)
	}

	pub fn get(&self, coords: &C) -> Option<&T> {
		self.map.get(coords)
	}

	pub fn get_adjacent(&self, coords: &C) -> Vec<T> where T: Copy {
		let mut list = Vec::new();
		for adjacent_coords in coords.adjacent() {
			if let Some(tile) = self.get(&adjacent_coords) {
				list.push(*tile)
			}
		}
		list
	}

	pub fn insert(&mut self, coords: C, tile: T) {
		self.map.insert(coords, tile);
	}
}

impl<C, T> Default for TileMap<C, T> where C: traits::TileCoords {
	fn default() -> Self {
		Self { map: HashMap::default() }
	}
}



#[cfg(test)]
mod tests {

	use super::*;

	mod tilemap {

		use super::*;

		mod methods {

			use super::*;
			use crate::hex::AxialCoords;

			#[test]
			fn get_adjacent() {
				let mut map: TileMap<AxialCoords<i32>, &str> = TileMap::default();
				map.insert(AxialCoords::new(0, 0), "center");
				map.insert(AxialCoords::new(1, -1), "adjacent");
				map.insert(AxialCoords::new(2, -1), "not adjacent");
				let adjacent_tiles = map.get_adjacent(&AxialCoords::new(0, 0));
				assert!(adjacent_tiles.contains(&"adjacent"));
				assert!(!adjacent_tiles.contains(&"center"));
				assert!(!adjacent_tiles.contains(&"not_adjacent"));
			}

			#[test]
			fn insert() {
				let mut map: TileMap<AxialCoords<i32>, &str> = TileMap::default();
				assert_eq!(None, map.get(&AxialCoords::new(0, 0)));
				map.insert(AxialCoords::new(0, 0), "foo");
				assert_eq!(Some(&"foo"), map.get(&AxialCoords::new(0, 0)));
			}
		}
	}
}
