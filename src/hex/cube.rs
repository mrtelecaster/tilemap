//! Cube coordinates. Has simpler math than axial coords, but takes up more space.

use std::{fmt::Debug, ops::{Add, Sub}};
use crate::{
	traits::TileCoords,
	hex::{AxialCoords, OffsetCoords},
};

use super::util::cube_round;



// CUBE COORDINATE STRUCT ----------------------------------------------------------------------- //

/// Cube coordinate set
#[derive(Debug, PartialEq)]
pub struct CubeCoords {
	pub q: isize,
	pub r: isize,
	pub s: isize,
}

impl CubeCoords {

	/// Initialize a new cube coordinate set with the given coordinates
	pub fn new(q: isize, r: isize, s: isize) -> Self {
		Self{ q, r, s }
	}

	/// Initializes a new `CubeCoords` instance with all coordinates set to the given value
	/// 
	/// ```
	/// # use tilemap::hex::cube::CubeCoords;
	/// let c = CubeCoords::splat(3);
	/// assert_eq!(3, c.q);
	/// assert_eq!(3, c.r);
	/// assert_eq!(3, c.s);
	/// ```
	pub fn splat(val: isize) -> Self {
		Self{ q: val, r: val, s: val }
	}

	pub fn from_round(q: f32, r: f32, s: f32) -> Self {
		let (int_q, int_r, int_s) = cube_round(q, r, s);
		Self::new(int_q, int_r, int_s)
	}

	pub fn is_valid(&self) -> bool {
		self.s == -self.q - self.r
	}
}


// TILE COORDS TRAIT IMPLEMENTATION ------------------------------------------------------------- //

impl TileCoords for CubeCoords {

    fn adjacent_coords(&self) -> Vec<Self> where Self: Sized {
		let one = 1;
		let zero = 0;
		let neg_one = -1;
        vec![
			self + CubeCoords::new(one, neg_one, zero),
			self + CubeCoords::new(one, zero, neg_one),
			self + CubeCoords::new(zero, one, neg_one),
			self + CubeCoords::new(neg_one, one, zero),
			self + CubeCoords::new(neg_one, zero, one),
			self + CubeCoords::new(zero, neg_one, one),
		]
    }

    fn distance(&self, other: &Self) -> isize {
        let vec = self - other;
		let q = vec.q.abs();
		let r = vec.r.abs();
		let s = vec.s.abs();
		let two = 2;
		((q + r + s) / two).into()
    }
}


// `std::ops` IMPLEMENTATIONS ------------------------------------------------------------------- //

impl Add for CubeCoords {

    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self{
			q: self.q + rhs.q,
			r: self.r + rhs.r,
			s: self.s + rhs.s,
		}
    }
}

impl Add<&CubeCoords> for CubeCoords {

    type Output = Self;

    fn add(self, rhs: &CubeCoords) -> Self::Output {
        Self{
			q: self.q + rhs.q,
			r: self.r + rhs.r,
			s: self.s + rhs.s,
		}
    }
}

impl Add<CubeCoords> for &CubeCoords {

    type Output = CubeCoords;

    fn add(self, rhs: CubeCoords) -> Self::Output {
        CubeCoords{
			q: self.q + rhs.q,
			r: self.r + rhs.r,
			s: self.s + rhs.s,
		}
    }
}

impl Add<&CubeCoords> for &CubeCoords {

    type Output = CubeCoords;

    fn add(self, rhs: &CubeCoords) -> Self::Output {
        CubeCoords{
			q: self.q + rhs.q,
			r: self.r + rhs.r,
			s: self.s + rhs.s,
		}
    }
}

impl Sub for CubeCoords {

    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self{ q: self.q - rhs.q, r: self.r - rhs.r, s: self.s - rhs.s }
    }
}

impl Sub<&CubeCoords> for &CubeCoords {
	type Output = CubeCoords;

	fn sub(self, rhs: &CubeCoords) -> Self::Output {
		CubeCoords::new(self.q - rhs.q, self.r - rhs.r, self.s - rhs.s)
	}
}


// `FROM` IMPLEMENTATIONS ----------------------------------------------------------------------- //

impl From<AxialCoords> for CubeCoords
{
	/// Creates a new cube coordinate from the given axial coordinate [as described here]
	/// (https://www.redblobgames.com/grids/hexagons/#conversions-axial)
    fn from(c: AxialCoords) -> Self {
        Self{
			q: c.q,
			r: c.r,
			s: -c.q - c.r,
		}
    }
}

impl From<&AxialCoords> for CubeCoords
{
	fn from(c: &AxialCoords) -> Self {
		Self::new(c.q, c.r, -c.q - c.r)
	}
}

impl From<OffsetCoords> for CubeCoords
{
	/// Creates a new cube coordinate set from the given offset coordinates, [as described in the article](https://www.redblobgames.com/grids/hexagons/#conversions-offset)
    fn from(c: OffsetCoords) -> Self {
        Self::from(AxialCoords::from(c))
    }
}

impl From<&OffsetCoords> for CubeCoords
{
	fn from(c: &OffsetCoords) -> Self {
		Self::from(OffsetCoords::new(c.q, c.r))
	}
}


// UNIT TESTS ----------------------------------------------------------------------------------- //

#[cfg(test)]
mod tests {

	use super::*;

	mod methods {

		use super::*;

		#[test]
		fn is_valid() {
			assert!(CubeCoords::new(-2, 3, -1).is_valid());
			assert!(!CubeCoords::new(-2, 3, 0).is_valid());
			assert!(!CubeCoords::new(-2, 3, -2).is_valid());
		}
	}

	mod traits {

	use super::*;

		mod tile_coords {

			use super::*;

			#[test]
			fn adjacent() {
				// test simplest case relative to center
				let coord = CubeCoords::splat(0);
				let adjacent_coords = coord.adjacent_coords();
				assert_eq!(6, adjacent_coords.len());
				assert!(adjacent_coords.contains(&CubeCoords::new(1, -1, 0)));
				assert!(adjacent_coords.contains(&CubeCoords::new(1, 0, -1)));
				assert!(adjacent_coords.contains(&CubeCoords::new(0, 1, -1)));
				assert!(adjacent_coords.contains(&CubeCoords::new(-1, 1, 0)));
				assert!(adjacent_coords.contains(&CubeCoords::new(-1, 0, 1)));
				assert!(adjacent_coords.contains(&CubeCoords::new(0, -1, 1)));

				// test an off center coordinate
				let coord = CubeCoords::new(2, -3, 1);
				let adjacent_coords = coord.adjacent_coords();
				assert_eq!(6, adjacent_coords.len());
				assert!(adjacent_coords.contains(&CubeCoords::new(3, -3, 0)));
				assert!(adjacent_coords.contains(&CubeCoords::new(2, -2, 0)));
				assert!(adjacent_coords.contains(&CubeCoords::new(1, -2, 1)));
				assert!(adjacent_coords.contains(&CubeCoords::new(1, -3, 2)));
				assert!(adjacent_coords.contains(&CubeCoords::new(2, -4, 2)));
				assert!(adjacent_coords.contains(&CubeCoords::new(3, -4, 1)));
			}

			#[test]
			fn distance() {
				assert_eq!(0, CubeCoords::splat(0).distance(&CubeCoords::splat(0)));
				assert_eq!(1, CubeCoords::new(1, -1, 0).distance(&CubeCoords::splat(0)));
				assert_eq!(2, CubeCoords::new(1, -1, 0).distance(&CubeCoords::new(-1, 0, 1)));
				assert_eq!(3, CubeCoords::new(2, -1, -1).distance(&CubeCoords::new(-1, 0, 1)));
			}
		}

		#[test]
		fn from_axial_coords() {
			assert_eq!(CubeCoords::new(0, 0, 0), AxialCoords::new(0, 0).into());

			assert_eq!(CubeCoords::new(1, -1, 0), AxialCoords::new(1, -1).into());
			assert_eq!(CubeCoords::new(1, 0, -1), AxialCoords::new(1, 0).into());
			assert_eq!(CubeCoords::new(0, 1, -1), AxialCoords::new(0, 1).into());
			assert_eq!(CubeCoords::new(-1, 1, 0), AxialCoords::new(-1, 1).into());
			assert_eq!(CubeCoords::new(-1, 0, 1), AxialCoords::new(-1, 0).into());
			assert_eq!(CubeCoords::new(0, -1, 1), AxialCoords::new(0, -1).into());

			assert_eq!(CubeCoords::new(2, -2, 0), AxialCoords::new(2, -2).into());
			assert_eq!(CubeCoords::new(2, -1, -1), AxialCoords::new(2, -1).into());
			assert_eq!(CubeCoords::new(2, 0, -2), AxialCoords::new(2, 0).into());
			assert_eq!(CubeCoords::new(1, 1, -2), AxialCoords::new(1, 1).into());
			assert_eq!(CubeCoords::new(0, 2, -2), AxialCoords::new(0, 2).into());
			assert_eq!(CubeCoords::new(-1, 2, -1), AxialCoords::new(-1, 2).into());
			assert_eq!(CubeCoords::new(-2, 2, 0), AxialCoords::new(-2, 2).into());
			assert_eq!(CubeCoords::new(-2, 1, 1), AxialCoords::new(-2, 1).into());
			assert_eq!(CubeCoords::new(-2, 0, 2), AxialCoords::new(-2, 0).into());
			assert_eq!(CubeCoords::new(-1, -1, 2), AxialCoords::new(-1, -1).into());
			assert_eq!(CubeCoords::new(0, -2, 2), AxialCoords::new(0, -2).into());
			assert_eq!(CubeCoords::new(1, -2, 1), AxialCoords::new(1, -2).into());
		}

		#[test]
		fn from_offset_coords() {
			assert_eq!(CubeCoords::new(-1, -2, 3), OffsetCoords::new(-2, -2).into());
			assert_eq!(CubeCoords::new(0, -2, 2), OffsetCoords::new(-1, -2).into());
			assert_eq!(CubeCoords::new(1, -2, 1), OffsetCoords::new(0, -2).into());
			assert_eq!(CubeCoords::new(2, -2, 0), OffsetCoords::new(1, -2).into());
			assert_eq!(CubeCoords::new(3, -2, -1), OffsetCoords::new(2, -2).into());

			assert_eq!(CubeCoords::new(-1, -1, 2), OffsetCoords::new(-2, -1).into());
			assert_eq!(CubeCoords::new(0, -1, 1), OffsetCoords::new(-1, -1).into());
			assert_eq!(CubeCoords::new(1, -1, 0), OffsetCoords::new(0, -1).into());
			assert_eq!(CubeCoords::new(2, -1, -1), OffsetCoords::new(1, -1).into());
			assert_eq!(CubeCoords::new(3, -1, -2), OffsetCoords::new(2, -1).into());

			assert_eq!(CubeCoords::new(-2, 0, 2), OffsetCoords::new(-2, 0).into());
			assert_eq!(CubeCoords::new(-1, 0, 1), OffsetCoords::new(-1, 0).into());
			assert_eq!(CubeCoords::new(0, 0, 0), OffsetCoords::new(0, 0).into());
			assert_eq!(CubeCoords::new(1, 0, -1), OffsetCoords::new(1, 0).into());
			assert_eq!(CubeCoords::new(2, 0, -2), OffsetCoords::new(2, 0).into());

			assert_eq!(CubeCoords::new(-2, 1, 1), OffsetCoords::new(-2, 1).into());
			assert_eq!(CubeCoords::new(-1, 1, 0), OffsetCoords::new(-1, 1).into());
			assert_eq!(CubeCoords::new(0, 1, -1), OffsetCoords::new(0, 1).into());
			assert_eq!(CubeCoords::new(1, 1, -2), OffsetCoords::new(1, 1).into());
			assert_eq!(CubeCoords::new(2, 1, -3), OffsetCoords::new(2, 1).into());

			assert_eq!(CubeCoords::new(-3, 2, 1), OffsetCoords::new(-2, 2).into());
			assert_eq!(CubeCoords::new(-2, 2, 0), OffsetCoords::new(-1, 2).into());
			assert_eq!(CubeCoords::new(-1, 2, -1), OffsetCoords::new(0, 2).into());
			assert_eq!(CubeCoords::new(0, 2, -2), OffsetCoords::new(1, 2).into());
			assert_eq!(CubeCoords::new(1, 2, -3), OffsetCoords::new(2, 2).into());
		}
	
		#[test]
		fn sub() {
			assert_eq!(CubeCoords::new(0, -1, 1), CubeCoords::new(0, -3, 3) - CubeCoords::new(0, -2, 2));
			assert_eq!(CubeCoords::new(0, 1, -1), CubeCoords::new(0, -2, 2) - CubeCoords::new(0, -3, 3));
			assert_eq!(CubeCoords::new(3, -2, -1), CubeCoords::new(1, 1, -2) - CubeCoords::new(-2, 3, -1));
		}
	}
}
