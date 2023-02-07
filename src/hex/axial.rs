//! Axial hex coordinates. More space efficient than cube but math is a bit of a pain.

use std::{fmt::Debug, ops::{Add, Sub}};
use crate::{traits::TileCoords, hex::{CubeCoords, OffsetCoords}};



#[derive(Debug, PartialEq)]
pub struct AxialCoords {
	pub q: isize,
	pub r: isize,
}

impl AxialCoords {

	/// Create a new axial coordinate pair with the given Q and R coordinates
	pub fn new(q: isize, r: isize) -> Self {
		Self{ q, r }
	}

	/// Creates a new coordinate pair where both values are the given input value.
	/// 
	/// ```
	/// # use tilemap::hex::axial::AxialCoords;
	/// let coord = AxialCoords::splat(3);
	/// assert_eq!(3, coord.q);
	/// assert_eq!(3, coord.r);
	/// ```
	pub fn splat(val: isize) -> Self {
		Self::new(val, val)
	}
}


// TILE COORDS TRAIT IMPLEMENTATION ------------------------------------------------------------- //

impl TileCoords for AxialCoords {
    fn adjacent_coords(&self) -> Vec<Self> where Self: Sized {
        vec![
			self + AxialCoords::new(1, 0),
			self + AxialCoords::new(0, 1),
			self + AxialCoords::new(-1, 1),
			self + AxialCoords::new(-1, 0),
			self + AxialCoords::new(0, -1),
			self + AxialCoords::new(1, -1),
		]
    }

    fn distance(&self, other: &Self) -> isize {
        CubeCoords::from(self).distance(&CubeCoords::from(other))
    }
}


// STD OPS IMPLEMENTATIONS ---------------------------------------------------------------------- //

impl Add for AxialCoords {

    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output{
			q: self.q + rhs.q,
			r: self.r + rhs.r,
		}
    }
}

impl Add<AxialCoords> for &AxialCoords {

	type Output = AxialCoords;

	fn add(self, rhs: AxialCoords) -> Self::Output {
		Self::Output{
			q: self.q + rhs.q,
			r: self.r + rhs.r,
		}
	}
}

impl Sub for AxialCoords {

	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		Self::from(CubeCoords::from(self) - CubeCoords::from(rhs))
	}
}



// `FROM` TRAIT --------------------------------------------------------------------------------- //

impl From<CubeCoords> for AxialCoords
{
	/// Creates a new axial coordinate from the given cube coordinate
	/// [as described here](https://www.redblobgames.com/grids/hexagons/#conversions-axial)
    fn from(c: CubeCoords) -> Self {
		Self { q: c.q, r: c.r }
    }
}

impl From<OffsetCoords> for AxialCoords
{
	/// Creates a new axial coordinate pair from the given set of offset coordinates
	/// [as described in the article](https://www.redblobgames.com/grids/hexagons/#conversions-offset)
    fn from(c: OffsetCoords) -> Self {
        let q = c.q - (c.r - (c.r & 1)) / 2;
		let r = c.r;
		Self{ q, r }
    }
}

#[cfg(test)]
mod tests {

	use super::*;

	mod traits {

		use super::*;

		mod tile_coords {

			use super::*;

			#[test]
			fn adjacent() {
				// test simplest case relative to center
				let coord = AxialCoords::splat(0);
				let adjacent_coords = coord.adjacent_coords();
				assert_eq!(6, adjacent_coords.len());
				assert!(adjacent_coords.contains(&AxialCoords::new(1, -1)));
				assert!(adjacent_coords.contains(&AxialCoords::new(1, 0)));
				assert!(adjacent_coords.contains(&AxialCoords::new(0, 1)));
				assert!(adjacent_coords.contains(&AxialCoords::new(-1, 1)));
				assert!(adjacent_coords.contains(&AxialCoords::new(-1, 0)));
				assert!(adjacent_coords.contains(&AxialCoords::new(0, -1)));

				// test an off center coordinate
				let coord = AxialCoords::new(2, -3);
				let adjacent_coords = coord.adjacent_coords();
				assert_eq!(6, adjacent_coords.len());
				assert!(adjacent_coords.contains(&AxialCoords::new(3, -3)));
				assert!(adjacent_coords.contains(&AxialCoords::new(2, -2)));
				assert!(adjacent_coords.contains(&AxialCoords::new(1, -2)));
				assert!(adjacent_coords.contains(&AxialCoords::new(1, -3)));
				assert!(adjacent_coords.contains(&AxialCoords::new(2, -4)));
				assert!(adjacent_coords.contains(&AxialCoords::new(3, -4)));
			}

			#[test]
			fn distance() {
				assert_eq!(0, AxialCoords::splat(0).distance(&AxialCoords::splat(0)));
				assert_eq!(1, AxialCoords::new(1, -1).distance(&AxialCoords::splat(0)));
				assert_eq!(2, AxialCoords::new(1, -1).distance(&AxialCoords::new(-1, 0)));
				assert_eq!(3, AxialCoords::new(2, -1).distance(&AxialCoords::new(-1, 0)));
			}
		}

		#[test]
		fn from_cube_coords() {
			assert_eq!(AxialCoords::new(0, 0), CubeCoords::new(0, 0, 0).into());

			assert_eq!(AxialCoords::new(1, -1), CubeCoords::new(1, -1, 0).into());
			assert_eq!(AxialCoords::new(1, 0), CubeCoords::new(1, 0, -1).into());
			assert_eq!(AxialCoords::new(0, 1), CubeCoords::new(0, 1, -1).into());
			assert_eq!(AxialCoords::new(-1, 1), CubeCoords::new(-1, 1, 0).into());
			assert_eq!(AxialCoords::new(-1, 0), CubeCoords::new(-1, 0, 1).into());
			assert_eq!(AxialCoords::new(0, -1), CubeCoords::new(0, -1, 1).into());

			assert_eq!(AxialCoords::new(2, -2), CubeCoords::new(2, -2, 0).into());
			assert_eq!(AxialCoords::new(2, -1), CubeCoords::new(2, -1, -1).into());
			assert_eq!(AxialCoords::new(2, 0), CubeCoords::new(2, 0, -2).into());
			assert_eq!(AxialCoords::new(1, 1), CubeCoords::new(1, 1, -2).into());
			assert_eq!(AxialCoords::new(0, 2), CubeCoords::new(0, 2, -2).into());
			assert_eq!(AxialCoords::new(-1, 2), CubeCoords::new(-1, 2, -1).into());
			assert_eq!(AxialCoords::new(-2, 2), CubeCoords::new(-2, 2, 0).into());
			assert_eq!(AxialCoords::new(-2, 1), CubeCoords::new(-2, 1, 1).into());
			assert_eq!(AxialCoords::new(-2, 0), CubeCoords::new(-2, 0, 2).into());
			assert_eq!(AxialCoords::new(-1, -1), CubeCoords::new(-1, -1, 2).into());
			assert_eq!(AxialCoords::new(0, -2), CubeCoords::new(0, -2, 2).into());
			assert_eq!(AxialCoords::new(1, -2), CubeCoords::new(1, -2, 1).into());
		}

		#[test]
		fn from_offset_coords() {
			assert_eq!(AxialCoords::new(-1, -2), OffsetCoords::new(-2, -2).into());
			assert_eq!(AxialCoords::new(0, -2), OffsetCoords::new(-1, -2).into());
			assert_eq!(AxialCoords::new(1, -2), OffsetCoords::new(0, -2).into());
			assert_eq!(AxialCoords::new(2, -2), OffsetCoords::new(1, -2).into());
			assert_eq!(AxialCoords::new(3, -2), OffsetCoords::new(2, -2).into());

			assert_eq!(AxialCoords::new(-1, -1), OffsetCoords::new(-2, -1).into());
			assert_eq!(AxialCoords::new(0, -1), OffsetCoords::new(-1, -1).into());
			assert_eq!(AxialCoords::new(1, -1), OffsetCoords::new(0, -1).into());
			assert_eq!(AxialCoords::new(2, -1), OffsetCoords::new(1, -1).into());
			assert_eq!(AxialCoords::new(3, -1), OffsetCoords::new(2, -1).into());

			assert_eq!(AxialCoords::new(-2, 0), OffsetCoords::new(-2, 0).into());
			assert_eq!(AxialCoords::new(-1, 0), OffsetCoords::new(-1, 0).into());
			assert_eq!(AxialCoords::new(0, 0), OffsetCoords::new(0, 0).into());
			assert_eq!(AxialCoords::new(1, 0), OffsetCoords::new(1, 0).into());
			assert_eq!(AxialCoords::new(2, 0), OffsetCoords::new(2, 0).into());

			assert_eq!(AxialCoords::new(-2, 1), OffsetCoords::new(-2, 1).into());
			assert_eq!(AxialCoords::new(-1, 1), OffsetCoords::new(-1, 1).into());
			assert_eq!(AxialCoords::new(0, 1), OffsetCoords::new(0, 1).into());
			assert_eq!(AxialCoords::new(1, 1), OffsetCoords::new(1, 1).into());
			assert_eq!(AxialCoords::new(2, 1), OffsetCoords::new(2, 1).into());

			assert_eq!(AxialCoords::new(-3, 2), OffsetCoords::new(-2, 2).into());
			assert_eq!(AxialCoords::new(-2, 2), OffsetCoords::new(-1, 2).into());
			assert_eq!(AxialCoords::new(-1, 2), OffsetCoords::new(0, 2).into());
			assert_eq!(AxialCoords::new(0, 2), OffsetCoords::new(1, 2).into());
			assert_eq!(AxialCoords::new(1, 2), OffsetCoords::new(2, 2).into());
		}
	}
}
