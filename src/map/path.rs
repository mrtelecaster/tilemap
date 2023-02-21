//! Module containing pathfinding logic

use std::{collections::HashSet, hash::Hash};
use crate::{map::TileMap, traits::{Tile, TileCoords}};


pub struct PathfindNode<C> {
	pub total_cost: isize,
	pub prev_coords: C
}


/// Used to find paths through tilemaps
pub struct Pathfinder<C> {
	coords_to_search: HashSet<C>,
	searched_coords: HashSet<C>,
	path_map: TileMap<C, PathfindNode<C>>,
}

impl<C> Pathfinder<C> {

	/// Creates a new pathfinder instance
	pub fn new() -> Self {
		Self{ coords_to_search: HashSet::new(), searched_coords: HashSet::new(), path_map: TileMap::new() }
	}

	/// Finds a path from `start` to `end` coordinates
	pub fn find_path<T>(&mut self, map: &TileMap<C, T>, start: C, end: C) -> Option<Vec<C>> where C: Copy + TileCoords {
		None
	}

	/// Returns the coords in `coords_to_search` with the lowest total cost. If there are no nodes
	/// left to search, returns `None`.
	/// 
	/// This function on finding a new set of coordinates will also move it from the
	/// `coords_to_search` table and into the `searched_coords` table when pathfinding is choosing a
	/// new set of coordinates to search after searching a previous one.
	fn get_next_coords(&mut self) -> Option<C> where C: Copy + TileCoords {
		let mut best_coords = None;
		for test_coords in self.coords_to_search.iter() {
			if best_coords.is_none() {
				best_coords = Some(*test_coords);
			} else {
				let best_node = self.path_map.get_tile(&best_coords.unwrap()).unwrap();
				let test_node = self.path_map.get_tile(&test_coords).unwrap();
				if test_node.total_cost < best_node.total_cost {
					best_coords = Some(*test_coords)
				}
			}
		}
		if let Some(c) = best_coords {
			self.coords_to_search.remove(&c);
			self.searched_coords.insert(c);
		}
		best_coords
	}

	fn get_node(&self, coords: &C) -> Option<&PathfindNode<C>> where C: Eq + Hash {
		self.path_map.get_tile(coords)
	}

	/// Inserts a new pathfinding node at the given coordinates. Also adds the coordinates to `coords_to_search`
	fn insert_node(&mut self, coords: &C, node: PathfindNode<C>) where C: Copy + Eq + Hash {
		self.path_map.insert_tile(*coords, node);
		self.coords_to_search.insert(*coords);
	}
}


#[cfg(test)]
mod tests
{
	use super::*;
	use crate::hex::AxialCoords;

	#[test]
	fn next_coords_to_search() {
		// initialize pathfinder
		let mut pathfinder = Pathfinder::<AxialCoords>::new();

		// no next node when pathfinder is empty
		assert_eq!(None, pathfinder.get_next_coords());

		let origin = AxialCoords::new(0, 0);
		let near_coords = AxialCoords::new(1, 2);
		let far_coords = AxialCoords::new(3, 4);
		pathfinder.insert_node(&far_coords, PathfindNode{ total_cost: 3, prev_coords: origin });
		pathfinder.insert_node(&near_coords, PathfindNode{ total_cost: 1, prev_coords: origin });

		let next_coords = pathfinder.get_next_coords();
		assert_eq!(Some(near_coords), next_coords);
		assert!(pathfinder.coords_to_search.contains(&far_coords));
		assert!(!pathfinder.coords_to_search.contains(&near_coords));
		assert!(pathfinder.searched_coords.contains(&near_coords));
		assert!(!pathfinder.searched_coords.contains(&far_coords));

		let next_coords = pathfinder.get_next_coords();
		assert_eq!(Some(far_coords), next_coords);
		assert!(!pathfinder.coords_to_search.contains(&far_coords));
		assert!(!pathfinder.coords_to_search.contains(&near_coords));
		assert!(pathfinder.searched_coords.contains(&near_coords));
		assert!(pathfinder.searched_coords.contains(&far_coords));
	}

	#[test]
	#[ignore]
	fn find_path()
	{
		let path_start_coords = AxialCoords::new(-2, 1);
		let path_end_coords = AxialCoords::new(1, -1);
		let origin_coords = AxialCoords::splat(0); // set center of map at 0, 0
		let initial_tile_coords = origin_coords.area_tiles(2); // get an area of tiles to initialize the map
		let mut map = TileMap::new();
		for initial_coord in initial_tile_coords {
			map.insert_tile(initial_coord, ());
		}
		let mut pathfinder = Pathfinder::new();
		let path = pathfinder.find_path(&map, path_start_coords, path_end_coords).unwrap();
		// check path has expected length
		assert_eq!(4, path.len());
		// check path contains start and end coords
		assert!(path.contains(&path_start_coords));
		assert!(path.contains(&path_end_coords));
		// check the middle section of the path contains expected tiles
		assert!(path.contains(&AxialCoords::new(-1, 0)) || path.contains(&AxialCoords::new(-1, 1)));
		assert!(path.contains(&AxialCoords::new(0, 0)) || path.contains(&AxialCoords::new(0, 1)));
	}
}
