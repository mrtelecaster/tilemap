//! Module containing pathfinding logic

use std::{collections::HashSet, fmt::Debug, hash::Hash};
use crate::{map::TileMap, traits::{Tile, TileCoords}};


#[derive(Clone)]
pub struct PathfindNode<C> {
	pub total_cost: isize,
	pub from_coords: Option<C>
}


/// Used to find paths through tilemaps
#[derive(Clone)]
pub struct Pathfinder<C> where C: Clone + Eq + Hash {
	coords_to_search: HashSet<C>,
	searched_coords: HashSet<C>,
	path_map: TileMap<C, PathfindNode<C>>,
}

impl<C> Pathfinder<C> where C: Clone + Eq + Hash {

	/// Finds a path from `start` to `end` coordinates
	pub fn find_path<T>(map: &TileMap<C, T>, start: C, end: C) -> Option<Vec<C>> where C: Clone + Copy + TileCoords, T: Tile {

		let mut pathmap = TileMap::<C, PathfindNode<C>>::new();
		pathmap.insert_tile(start, PathfindNode{ total_cost: 0, from_coords: None });
		let mut coords_to_search = HashSet::<C>::new();
		let mut searched_coords = HashSet::<C>::new();
		let mut test_coords_opt = Some(start);

		// loop while next coordinate to search is not none
		while test_coords_opt.is_some() {

			let test_coords = test_coords_opt.unwrap();

			if test_coords == end {
				let mut path_coords = test_coords;
				let mut path = vec![path_coords];
				let mut path_node = pathmap.get_tile(&path_coords).unwrap();
				while path_node.from_coords.is_some() {
					path_coords = path_node.from_coords.unwrap();
					path.push(path_coords);
					path_node = pathmap.get_tile(&path_coords).unwrap();
				}
				return Some(path);
			}

			let adjacent_coords = test_coords.adjacent_coords();
			
			for adjacent_coord in adjacent_coords.iter() {

				let adjacent_tile = match map.get_tile(adjacent_coord) {
					Some(tile) => tile,
					None => { continue; }
				};
				
				let cost_from_test_coords = {
					let test_node = pathmap.get_tile(&test_coords).unwrap();
					test_node.total_cost + adjacent_tile.pathfind_cost::<T>()
				};
				if pathmap.contains_coords(adjacent_coord) {
					let mut adjacent_node = pathmap.get_tile_mut(adjacent_coord).unwrap();
					if cost_from_test_coords < adjacent_node.total_cost {
						adjacent_node.total_cost = cost_from_test_coords;
						adjacent_node.from_coords = Some(test_coords);
					}
				}
				else {
					let new_node = PathfindNode{ total_cost: cost_from_test_coords, from_coords: Some(test_coords) };
					pathmap.insert_tile(*adjacent_coord, new_node);
					coords_to_search.insert(*adjacent_coord);
				}
			}

			searched_coords.insert(test_coords);
			coords_to_search.remove(&test_coords);

			// get next coordinate to search. If `None`, loop exits and no path is returned. This should remove the chosen new test coords from the `coords_to_search` list
			test_coords_opt = {
				let mut best_coords: Option<C> = None;
				for coord in coords_to_search.iter() {
					if let Some(best_coord) = best_coords {
						let best_node = pathmap.get_tile(&best_coord).unwrap();
						let test_node = pathmap.get_tile(&coord).unwrap();
						if test_node.total_cost < best_node.total_cost {
							best_coords = Some(*coord);
						}
					} else {
						best_coords = Some(*coord);
					}
				}
				best_coords
			};
		}

		None
	}
}
