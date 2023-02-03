//! Cube coordinates. Has simpler math than axial coords, but takes up more space.

use std::{fmt::Debug, hash::Hash, ops::{Add, Neg, Sub}};
use crate::{traits::TileCoords, hex::axial::AxialCoords};



// CUBE COORDINATE STRUCT ----------------------------------------------------------------------- //

/// Cube coordinate set
#[derive(Debug, PartialEq)]
pub struct CubeCoords<T> {
	pub q: T,
	pub r: T,
	pub s: T,
}

impl<T> CubeCoords<T> {

	/// Initialize a new cube coordinate set with the given coordinates
	pub fn new(q: T, r: T, s: T) -> Self {
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
	pub fn splat(val: T) -> Self where T: Copy {
		Self{ q: val, r: val, s: val }
	}
}

impl<T> TileCoords for CubeCoords<T> where T: Add<Output=T> + Copy + Debug + Eq + From<isize> + Hash {

    fn adjacent_coords(&self) -> Vec<Self> where Self: Sized {
        vec![
			self + CubeCoords::new(1.into(), (-1).into(), 0.into()),
			self + CubeCoords::new(1.into(), 0.into(), (-1).into()),
			self + CubeCoords::new(0.into(), 1.into(), (-1).into()),
			self + CubeCoords::new((-1).into(), 1.into(), 0.into()),
			self + CubeCoords::new((-1).into(), 0.into(), 1.into()),
			self + CubeCoords::new(0.into(), (-1).into(), 1.into()),
		]
    }
}

impl<T> Add for CubeCoords<T> where T: Add<Output=T> {

    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self{
			q: self.q + rhs.q,
			r: self.r + rhs.r,
			s: self.s + rhs.s,
		}
    }
}

impl<T> Add<&CubeCoords<T>> for CubeCoords<T> where T: Add<Output=T> + Copy {

    type Output = Self;

    fn add(self, rhs: &CubeCoords<T>) -> Self::Output {
        Self{
			q: self.q + rhs.q,
			r: self.r + rhs.r,
			s: self.s + rhs.s,
		}
    }
}

impl<T> Add<CubeCoords<T>> for &CubeCoords<T> where T: Add<Output=T> + Copy {

    type Output = CubeCoords<T>;

    fn add(self, rhs: CubeCoords<T>) -> Self::Output {
        CubeCoords{
			q: self.q + rhs.q,
			r: self.r + rhs.r,
			s: self.s + rhs.s,
		}
    }
}

impl<T> Add<&CubeCoords<T>> for &CubeCoords<T> where T: Add<Output=T> + Copy {

    type Output = CubeCoords<T>;

    fn add(self, rhs: &CubeCoords<T>) -> Self::Output {
        CubeCoords{
			q: self.q + rhs.q,
			r: self.r + rhs.r,
			s: self.s + rhs.s,
		}
    }
}

impl<T> From<AxialCoords<T>> for CubeCoords<T> where T: Copy + Neg<Output=T> + Sub<Output=T> {

	/// Creates a new cube coordinate from the given axial coordinate [as described here]
	/// (https://www.redblobgames.com/grids/hexagons/#conversions-axial)
    fn from(c: AxialCoords<T>) -> Self {
        Self{
			q: c.q,
			r: c.r,
			s: -c.q - c.r,
		}
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
	}
}
