//! Offset hex coordinates. Simple method for making pseudo-rectangular maps

use std::{fmt::Debug, ops::{Add, BitAnd, Div, Neg, Sub}};

use crate::{traits::TileCoords, hex::{AxialCoords, CubeCoords, DoubledCoords}};



/// A coordinate pair for an offset coordinate hex map
#[derive(Debug, PartialEq)]
pub struct OffsetCoords<T> {
	/// Column
	pub q: T,
	/// Row
	pub r: T,
}

impl<T> OffsetCoords<T> {

	/// Create a new offset coordinate pair with the given Q and R coordinates
	pub fn new(q: T, r: T) -> Self {
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
	pub fn splat(val: T) -> Self where T: Copy {
		Self::new(val, val)
	}
}

impl<T> TileCoords for OffsetCoords<T> where T: Add<Output=T> + Copy + Debug + From<isize> + PartialEq {
    fn adjacent_coords(&self) -> Vec<Self> where Self: Sized {
        vec![
			self + OffsetCoords::new((-1).into(), (-1).into()),
			self + OffsetCoords::new(0.into(), (-1).into()),
			self + OffsetCoords::new(1.into(), 0.into()),
			self + OffsetCoords::new(0.into(), 1.into()),
			self + OffsetCoords::new((-1).into(), 1.into()),
			self + OffsetCoords::new((-1).into(), 0.into()),
		]
    }
}

impl<T> Add for OffsetCoords<T> where T: Add<Output=T> + Copy {

    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output{
			q: self.q + rhs.q,
			r: self.r + rhs.r,
		}
    }
}

impl<T> Add<OffsetCoords<T>> for &OffsetCoords<T> where T: Add<Output=T> + Copy {

	type Output = OffsetCoords<T>;

	fn add(self, rhs: OffsetCoords<T>) -> Self::Output {
		Self::Output{
			q: self.q + rhs.q,
			r: self.r + rhs.r,
		}
	}
}

impl<T> From<AxialCoords<T>> for OffsetCoords<T>
where T: Add<Output=T> + BitAnd<Output=T> + Copy + Div<Output=T> + From<isize> + Neg<Output=T> + Sub<Output=T> {
	/// Creates a new offset coordinate pair from the given axial coordinates, [as described in the
	/// article](https://www.redblobgames.com/grids/hexagons/#conversions-offset)
    fn from(c: AxialCoords<T>) -> Self {
        let q = c.q + (c.r - (c.r & 1.into())) / 2.into();
		let r = c.r;
		Self{ q, r }
    }
}

impl<T> From<CubeCoords<T>> for OffsetCoords<T>
where T: Add<Output=T> + BitAnd<Output=T> + Copy + Div<Output=T> + From<isize> + Neg<Output=T> + Sub<Output=T> {
	/// Creates a new offset coordinate pair from the given cube coordinate set, [as described in
	/// the article](https://www.redblobgames.com/grids/hexagons/#conversions-offset)
    fn from(c: CubeCoords<T>) -> Self {
        Self::from(AxialCoords::from(c))
    }
}

impl<T> From<DoubledCoords<T>> for OffsetCoords<T> {
    fn from(_: DoubledCoords<T>) -> Self {
        todo!()
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

		#[test]
		#[ignore]
		fn from_doubled_coords() {
			assert_eq!(OffsetCoords::new(0, 0), DoubledCoords::new(0, 0).into());
		}
	}
}