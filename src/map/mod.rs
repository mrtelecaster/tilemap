//! Tilemap related things

use std::{collections::HashMap, hash::Hash};
use crate::{hex::AxialCoords, traits::{TileCoords, Tile}};

mod path;


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

	pub fn find_path(&self, start: C, end: C) -> Vec<C> where C: Clone + TileCoords, T: Tile {
		/*
		let mut searched_nodes: TileMap<C, PathfindingTile<C>> = TileMap::new();
		let mut nodes_to_search: TileMap<C, PathfindingTile<C>> = TileMap::new();
		let adjacent_coords = start.adjacent_coords();
		for adj in adjacent_coords {
			if let Some(tile) = self.get_tile(&adj) {
				nodes_to_search.insert_tile(adj, PathfindingTile::new(tile.pathfind_cost(), Some(start.clone())));
			}
		}
		searched_nodes.insert_tile(start, PathfindingTile::new(0, None));
		while nodes_to_search.len() > 0 {
			let mut best_node_cost = None
			for unsearched_coords in nodes_to_search {
				let unsearched_node = self.map[unsearched_coords];
				if  best_node_cost.is_none()
			}
		}
		*/
		vec![]
	}

	pub fn len(&self) -> usize {
		self.map.len()
	}
}

// MAP ALIASES ---------------------------------------------------------------------------------- //

/// Tile map using hexagonal coordinates
pub type HexMap<T> = TileMap<AxialCoords, T>;


// UNIT TESTS ----------------------------------------------------------------------------------- //

mod tests
{
	use super::*;

	mod pathfinding
	{
		use crate::traits::TileCoords;
		use super::*;

		struct EmptyTile;

		impl Tile for EmptyTile {}

		#[test]
		#[ignore]
		fn equal_cost() {
			let mut map: HexMap<EmptyTile> = HexMap::new();
			let center = AxialCoords::splat(0);
			let initial_coords = center.area_tiles(2);
			for coord in initial_coords.iter() {
				map.insert_tile(*coord, EmptyTile);
			}
			let path = map.find_path(AxialCoords::new(-2, 1), AxialCoords::new(1, -1));
			assert_eq!(4, path.len());
			assert!(path.contains(&AxialCoords::new(-2, 1)));
			assert!(path.contains(&AxialCoords::new(1, -1)));
		}
	}
}
