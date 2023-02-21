//! Module containing pathfinding logic

use std::{collections::HashSet, fmt::Debug, hash::Hash};
use crate::{map::TileMap, traits::{Tile, TileCoords}};


pub struct PathfindNode<C> {
	pub total_cost: isize,
	pub from_coords: Option<C>
}


/// Used to find paths through tilemaps
pub struct Pathfinder<C> where C: Eq + Hash {
	coords_to_search: HashSet<C>,
	searched_coords: HashSet<C>,
	path_map: TileMap<C, PathfindNode<C>>,
}

impl<C> Pathfinder<C> where C: Eq + Hash {

	/// Creates a new pathfinder instance
	pub fn new() -> Self {
		Self{ coords_to_search: HashSet::new(), searched_coords: HashSet::new(), path_map: TileMap::new() }
	}

	/// Finds a path from `start` to `end` coordinates
	pub fn find_path<T>(&mut self, map: &TileMap<C, T>, start: C, end: C) -> Option<Vec<C>> where C: Copy + TileCoords {
		self.coords_to_search.clear();
		self.searched_coords.clear();
		self.path_map = TileMap::new();

		self.coords_to_search.insert(start);
		self.path_map.insert_tile(start, PathfindNode{ total_cost: 0, from_coords: None });

		while let Some(test_coords) = self.get_next_coords()
		{
			let cost_from_test_coords = self.path_map.get_tile(&test_coords).unwrap().total_cost;

			if test_coords == end
			{
				return Some(self.current_path_from(&end));
			}
			let adjacent_coords = test_coords.adjacent_coords();

			for adjacent_coord in adjacent_coords
			{
				// if coords correspond to a tile, test it, otherwise skip it
				if let Some(world_tile) = map.get_tile(&adjacent_coord)
				{
					self.test_coords(&adjacent_coord, &test_coords, cost_from_test_coords + 1);
				}
			}
		}

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

	/// Returns the path from `coords` to the starting point
	/// 
	/// Follows nodes' `from_coords` variable as a linked list. The returned vector is in order from
	/// destination to source, so that popping from the vector gets coordinates in the intended
	/// order, from source to destination. So the starting point of the path is in the last index,
	/// and the destination coords of the path are index 0
	fn current_path_from(&self, coords: &C) -> Vec<C> where C: Copy
	{
		let mut node = self.get_node(coords).unwrap();
		let mut path = vec![*coords];

		while let Some(new_coords) = node.from_coords
		{
			path.push(new_coords);
			node = self.get_node(&new_coords).unwrap();
		}

		path
	}

	/// Tests a pathfinding node at `dest` for its current cost, and update its source node to `src`
	/// if the total cost to go from that node to `dest` is lower than `dest`'s currnt cost.
	/// 
	/// `cost_from_src` is the cost of moving to `dest` from `src` and must be calculated by the caller of this function.
	/// 
	/// If no node exists at `coords`, one will be created with its source automatically set to `src`
	fn test_coords(&mut self, dest: &C, src: &C, cost_from_src: isize) where C: Copy + Debug {
		let src_node = self.get_node(src).unwrap_or_else(|| {
			panic!("Found `None` looking for source node at {:?}", src);
		});
		let total_cost_from_src = src_node.total_cost + cost_from_src;
		if let Some(node) = self.get_node_mut(dest) {
			if total_cost_from_src < node.total_cost {
				node.total_cost = total_cost_from_src;
				node.from_coords = Some(*src);
			}
		} else {
			let new_node = PathfindNode{ total_cost: total_cost_from_src , from_coords: Some(*src) };
			self.insert_node(dest, new_node);
		}
	}

	fn get_node(&self, coords: &C) -> Option<&PathfindNode<C>> {
		self.path_map.get_tile(coords)
	}

	fn get_node_mut(&mut self, coords: &C) -> Option<&mut PathfindNode<C>> {
		self.path_map.get_tile_mut(coords)
	}

	/// Inserts a new pathfinding node at the given coordinates. Also adds the coordinates to `coords_to_search`
	fn insert_node(&mut self, coords: &C, node: PathfindNode<C>) where C: Copy{
		self.path_map.insert_tile(*coords, node);
		self.coords_to_search.insert(*coords);
	}
}


#[cfg(test)]
mod tests
{
	use std::path::Path;

use super::*;
	use crate::hex::AxialCoords;

	#[test]
	fn path_from_coords()
	{
		let mut pathfinder = Pathfinder::new();
		let origin_coords = AxialCoords::new(0, 0);
		let destination_coords = AxialCoords::new(1, 1);
		let low_cost_coords = AxialCoords::new(1, 0);
		let high_cost_coords = AxialCoords::new(0, 1);

		pathfinder.insert_node(
			&origin_coords,
			PathfindNode{ total_cost: 0, from_coords: None },
		);
		pathfinder.insert_node(
			&low_cost_coords,
			PathfindNode{ total_cost: 6, from_coords: Some(origin_coords) },
		);
		pathfinder.insert_node(
			&high_cost_coords,
			PathfindNode{ total_cost: 9, from_coords: Some(origin_coords) },
		);
		pathfinder.insert_node(
			&destination_coords,
			PathfindNode{ total_cost: 8, from_coords: Some(low_cost_coords) },
		);

		let path = pathfinder.current_path_from(&low_cost_coords);
		assert_eq!(vec![low_cost_coords, origin_coords], path);

		let path = pathfinder.current_path_from(&destination_coords);
		assert_eq!(vec![destination_coords, low_cost_coords, origin_coords], path);

		let path = pathfinder.current_path_from(&high_cost_coords);
		assert_eq!(vec![high_cost_coords, origin_coords], path);

		let path = pathfinder.current_path_from(&origin_coords);
		assert_eq!(vec![origin_coords], path);
	}

	#[test]
	fn next_coords_to_search() {
		// initialize pathfinder
		let mut pathfinder = Pathfinder::<AxialCoords>::new();

		// no next node when pathfinder is empty
		assert_eq!(None, pathfinder.get_next_coords());

		let origin = AxialCoords::new(0, 0);
		let near_coords = AxialCoords::new(1, 2);
		let far_coords = AxialCoords::new(3, 4);
		pathfinder.insert_node(&far_coords, PathfindNode{ total_cost: 3, from_coords: Some(origin) });
		pathfinder.insert_node(&near_coords, PathfindNode{ total_cost: 1, from_coords: Some(origin) });

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
	fn test_coords() {
		let mut pathfinder = Pathfinder::new();
		let origin_coords = AxialCoords::new(0, 0);
		let destination_coords = AxialCoords::new(1, 1);
		let low_cost_coords = AxialCoords::new(1, 0);
		let high_cost_coords = AxialCoords::new(0, 1);

		pathfinder.insert_node(
			&origin_coords,
			PathfindNode{ total_cost: 0, from_coords: None },
		);
		pathfinder.insert_node(
			&low_cost_coords,
			PathfindNode{ total_cost: 6, from_coords: Some(origin_coords) },
		);
		pathfinder.insert_node(
			&high_cost_coords,
			PathfindNode{ total_cost: 9, from_coords: Some(origin_coords) },
		);

		// test destination coords with no node - check that node is created
		pathfinder.test_coords(&destination_coords, &high_cost_coords, 1);
		let node = pathfinder.get_node(&destination_coords).unwrap();
		assert_eq!(10, node.total_cost);
		assert_eq!(Some(high_cost_coords), node.from_coords);

		// test that when checked from a better node, the existing node is updated
		pathfinder.test_coords(&destination_coords, &low_cost_coords, 2);
		let node = pathfinder.get_node(&destination_coords).unwrap();
		assert_eq!(8, node.total_cost);
		assert_eq!(Some(low_cost_coords), node.from_coords);
	}
}
