//! Offset hex coordinates. Simple method for making pseudo-rectangular maps

use std::{fmt::Debug, ops::{Add, Sub}};
use crate::{traits::TileCoords, hex::{AxialCoords, CubeCoords}};



/// A coordinate pair for an offset coordinate hex map
#[derive(Debug, PartialEq)]
pub struct OffsetCoords {
	/// Column
	pub q: isize,
	/// Row
	pub r: isize,
}

impl OffsetCoords {

	/// Create a new offset coordinate pair with the given Q and R coordinates
	pub fn new(q: isize, r: isize) -> Self {
		Self{ q, r }
	}

	/// Creates a new coordinate pair where both values are the given input value.
	/// 
	/// ```
	/// # use tilemap::hex::offset::OffsetCoords;
	/// let coord = OffsetCoords::splat(3);
	/// assert_eq!(3, coord.q);
	/// assert_eq!(3, coord.r);
	/// ```
	pub fn splat(val: isize) -> Self where isize: Copy {
		Self::new(val, val)
	}
}


/// TILE COORDS TRAIT IMPLEMENTATION ------------------------------------------------------------ //

impl TileCoords for OffsetCoords
{
    fn adjacent_coords(&self) -> Vec<Self> {
		let neg_one = -1;
		let zero = 0;
		let one = 1;
        vec![
			self + OffsetCoords::new(neg_one, neg_one),
			self + OffsetCoords::new(zero, neg_one),
			self + OffsetCoords::new(one, zero),
			self + OffsetCoords::new(zero, one),
			self + OffsetCoords::new(neg_one, one),
			self + OffsetCoords::new(neg_one, zero),
		]
    }

    fn distance(&self, other: &Self) -> isize {
        CubeCoords::from(self).distance(&CubeCoords::from(other))
    }
}


// STD OPS IMPLEMENTATIONS ---------------------------------------------------------------------- //

impl Add for OffsetCoords {

    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output{
			q: self.q + rhs.q,
			r: self.r + rhs.r,
		}
    }
}

impl Add<OffsetCoords> for &OffsetCoords {

	type Output = OffsetCoords;

	fn add(self, rhs: OffsetCoords) -> Self::Output {
		Self::Output{
			q: self.q + rhs.q,
			r: self.r + rhs.r,
		}
	}
}

impl Sub for OffsetCoords
{
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		let cube: CubeCoords = CubeCoords::from(self) - CubeCoords::from(rhs);
		OffsetCoords::from(cube)
	}
}


// `FROM` IMPLEMENTATIONS ----------------------------------------------------------------------- //

impl From<AxialCoords> for OffsetCoords
{
	/// Creates a new offset coordinate pair from the given axial coordinates, [as described in the
	/// article](https://www.redblobgames.com/grids/hexagons/#conversions-offset)
    fn from(c: AxialCoords) -> Self {
		let one = 1;
		let two = 2;
        let q = c.q + (c.r - (c.r & one)) / two;
		let r = c.r;
		Self{ q, r }
    }
}

impl From<CubeCoords> for OffsetCoords
{
	/// Creates a new offset coordinate pair from the given cube coordinate set, [as described in
	/// the article](https://www.redblobgames.com/grids/hexagons/#conversions-offset)
    fn from(c: CubeCoords) -> Self {
        Self::from(AxialCoords::from(c))
    }
}


// UNIT TESTS ----------------------------------------------------------------------------------- //

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
				let coord = OffsetCoords::splat(0);
				let adjacent_coords = coord.adjacent_coords();
				assert_eq!(6, adjacent_coords.len());
				assert!(adjacent_coords.contains(&OffsetCoords::new(1, 0)));
				assert!(adjacent_coords.contains(&OffsetCoords::new(0, 1)));
				assert!(adjacent_coords.contains(&OffsetCoords::new(-1, 1)));
				assert!(adjacent_coords.contains(&OffsetCoords::new(-1, 0)));
				assert!(adjacent_coords.contains(&OffsetCoords::new(-1, -1)));
				assert!(adjacent_coords.contains(&OffsetCoords::new(0, -1)));

				// test an off center coordinate
				let coord = OffsetCoords::new(3, 2);
				let adjacent_coords = coord.adjacent_coords();
				assert_eq!(6, adjacent_coords.len());
				assert!(adjacent_coords.contains(&OffsetCoords::new(2, 1)));
				assert!(adjacent_coords.contains(&OffsetCoords::new(3, 1)));
				assert!(adjacent_coords.contains(&OffsetCoords::new(4, 2)));
				assert!(adjacent_coords.contains(&OffsetCoords::new(3, 3)));
				assert!(adjacent_coords.contains(&OffsetCoords::new(2, 3)));
				assert!(adjacent_coords.contains(&OffsetCoords::new(2, 2)));
			}

			#[test]
			fn distance() {
				assert_eq!(0, OffsetCoords::splat(0).distance(&OffsetCoords::splat(0)));
				assert_eq!(1, OffsetCoords::new(1, 0).distance(&OffsetCoords::splat(0)));
				assert_eq!(2, OffsetCoords::new(1, 0).distance(&OffsetCoords::new(-1, -1)));
				assert_eq!(3, OffsetCoords::new(1, 1).distance(&OffsetCoords::new(-1, -1)));
			}
		}

		#[test]
		fn from_axial_coords() {
			assert_eq!(OffsetCoords::new(-2, -2), AxialCoords::new(-1, -2).into());
			assert_eq!(OffsetCoords::new(-1, -2), AxialCoords::new(0, -2).into());
			assert_eq!(OffsetCoords::new(0, -2), AxialCoords::new(1, -2).into());
			assert_eq!(OffsetCoords::new(1, -2), AxialCoords::new(2, -2).into());
			assert_eq!(OffsetCoords::new(2, -2), AxialCoords::new(3, -2).into());

			assert_eq!(OffsetCoords::new(-2, -1), AxialCoords::new(-1, -1).into());
			assert_eq!(OffsetCoords::new(-1, -1), AxialCoords::new(0, -1).into());
			assert_eq!(OffsetCoords::new(0, -1), AxialCoords::new(1, -1).into());
			assert_eq!(OffsetCoords::new(1, -1), AxialCoords::new(2, -1).into());
			assert_eq!(OffsetCoords::new(2, -1), AxialCoords::new(3, -1).into());

			assert_eq!(OffsetCoords::new(-2, 0), AxialCoords::new(-2, 0).into());
			assert_eq!(OffsetCoords::new(-1, 0), AxialCoords::new(-1, 0).into());
			assert_eq!(OffsetCoords::new(0, 0), AxialCoords::new(0, 0).into());
			assert_eq!(OffsetCoords::new(1, 0), AxialCoords::new(1, 0).into());
			assert_eq!(OffsetCoords::new(2, 0), AxialCoords::new(2, 0).into());

			assert_eq!(OffsetCoords::new(-2, 1), AxialCoords::new(-2, 1).into());
			assert_eq!(OffsetCoords::new(-1, 1), AxialCoords::new(-1, 1).into());
			assert_eq!(OffsetCoords::new(0, 1), AxialCoords::new(0, 1).into());
			assert_eq!(OffsetCoords::new(1, 1), AxialCoords::new(1, 1).into());
			assert_eq!(OffsetCoords::new(2, 1), AxialCoords::new(2, 1).into());

			assert_eq!(OffsetCoords::new(-2, 2), AxialCoords::new(-3, 2).into());
			assert_eq!(OffsetCoords::new(-1, 2), AxialCoords::new(-2, 2).into());
			assert_eq!(OffsetCoords::new(0, 2), AxialCoords::new(-1, 2).into());
			assert_eq!(OffsetCoords::new(1, 2), AxialCoords::new(0, 2).into());
			assert_eq!(OffsetCoords::new(2, 2), AxialCoords::new(1, 2).into());
		}

		#[test]
		fn from_cube_coords() {
			assert_eq!(OffsetCoords::new(-2, -2), CubeCoords::new(-1, -2, 3).into());
			assert_eq!(OffsetCoords::new(-1, -2), CubeCoords::new(0, -2, 2).into());
			assert_eq!(OffsetCoords::new(0, -2), CubeCoords::new(1, -2, 1).into());
			assert_eq!(OffsetCoords::new(1, -2), CubeCoords::new(2, -2, 0).into());
			assert_eq!(OffsetCoords::new(2, -2), CubeCoords::new(3, -2, -1).into());

			assert_eq!(OffsetCoords::new(-2, -1), CubeCoords::new(-1, -1, 2).into());
			assert_eq!(OffsetCoords::new(-1, -1), CubeCoords::new(0, -1, 1).into());
			assert_eq!(OffsetCoords::new(0, -1), CubeCoords::new(1, -1, 0).into());
			assert_eq!(OffsetCoords::new(1, -1), CubeCoords::new(2, -1, -1).into());
			assert_eq!(OffsetCoords::new(2, -1), CubeCoords::new(3, -1, -2).into());

			assert_eq!(OffsetCoords::new(-2, 0), CubeCoords::new(-2, 0, 2).into());
			assert_eq!(OffsetCoords::new(-1, 0), CubeCoords::new(-1, 0, 1).into());
			assert_eq!(OffsetCoords::new(0, 0), CubeCoords::new(0, 0, 0).into());
			assert_eq!(OffsetCoords::new(1, 0), CubeCoords::new(1, 0, -1).into());
			assert_eq!(OffsetCoords::new(2, 0), CubeCoords::new(2, 0, -2).into());

			assert_eq!(OffsetCoords::new(-2, 1), CubeCoords::new(-2, 1, 1).into());
			assert_eq!(OffsetCoords::new(-1, 1), CubeCoords::new(-1, 1, 0).into());
			assert_eq!(OffsetCoords::new(0, 1), CubeCoords::new(0, 1, -1).into());
			assert_eq!(OffsetCoords::new(1, 1), CubeCoords::new(1, 1, -2).into());
			assert_eq!(OffsetCoords::new(2, 1), CubeCoords::new(2, 1, -3).into());

			assert_eq!(OffsetCoords::new(-2, 2), CubeCoords::new(-3, 2, 1).into());
			assert_eq!(OffsetCoords::new(-1, 2), CubeCoords::new(-2, 2, 0).into());
			assert_eq!(OffsetCoords::new(0, 2), CubeCoords::new(-1, 2, -1).into());
			assert_eq!(OffsetCoords::new(1, 2), CubeCoords::new(0, 2, -2).into());
			assert_eq!(OffsetCoords::new(2, 2), CubeCoords::new(1, 2, -3).into());
		}
	}
}
