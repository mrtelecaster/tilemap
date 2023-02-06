//! Doubled hex coordinates. Method for making pseudo-rectangular maps that's a bit more
//! mathematically elegant than the [Offset coordinate system](crate::hex::offset)

use std::{fmt::Debug, ops::{Add, BitAnd, Div, Mul, Neg, Sub}};

use num::NumCast;

use crate::{traits::TileCoords, hex::{AxialCoords, CubeCoords, OffsetCoords}};



/// A coordinate pair for an offset coordinate hex map
#[derive(Debug, PartialEq)]
pub struct DoubledCoords<T> {
	/// Column
	pub q: T,
	/// Row
	pub r: T,
}

impl<T> DoubledCoords<T> {

	/// Create a new offset coordinate pair with the given Q and R coordinates
	pub fn new(q: T, r: T) -> Self {
		Self{ q, r }
	}

	/// Creates a new coordinate pair where both values are the given input value.
	/// 
	/// ```
	/// # use tilemap::hex::doubled::DoubledCoords;
	/// let coord = DoubledCoords::splat(3);
	/// assert_eq!(3, coord.q);
	/// assert_eq!(3, coord.r);
	/// ```
	pub fn splat(val: T) -> Self where T: Copy {
		Self::new(val, val)
	}
}

impl<T> TileCoords<T> for DoubledCoords<T> where T: Add<Output=T> + Copy + Debug + NumCast + PartialEq {
    fn adjacent_coords(&self) -> Vec<Self> where Self: Sized {
		let neg_two: T = NumCast::from(-2).unwrap();
		let neg_one: T = NumCast::from(-1).unwrap();
		let zero: T = NumCast::from(0).unwrap();
		let one: T = NumCast::from(1).unwrap();
		let two: T = NumCast::from(2).unwrap();
        vec![
			self + DoubledCoords::new(neg_one, neg_one),
			self + DoubledCoords::new(one, neg_one),
			self + DoubledCoords::new(two, zero),
			self + DoubledCoords::new(one, one),
			self + DoubledCoords::new(neg_one, one),
			self + DoubledCoords::new(neg_two, zero),
		]
    }
}


// STD OPS IMPLEMENTATIONS ---------------------------------------------------------------------- //

impl<T> Add for DoubledCoords<T> where T: Add<Output=T> {

    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output{
			q: self.q + rhs.q,
			r: self.r + rhs.r,
		}
    }
}

impl<T> Add<DoubledCoords<T>> for &DoubledCoords<T> where T: Add<Output=T> + Copy {

	type Output = DoubledCoords<T>;

	fn add(self, rhs: DoubledCoords<T>) -> Self::Output {
		Self::Output{
			q: self.q + rhs.q,
			r: self.r + rhs.r,
		}
	}
}

impl<T> Sub for DoubledCoords<T>
where T: Add<Output=T> + BitAnd<Output=T> + Copy + Div<Output=T> + Mul<Output=T> + Neg<Output=T> + NumCast + Sub<Output=T>
{
	type Output = DoubledCoords<T>;

	fn sub(self, rhs: DoubledCoords<T>) -> Self::Output {
		let lhs_cube = CubeCoords::from(self);
		let rhs_cube = CubeCoords::from(rhs);
		let result_cube = lhs_cube - rhs_cube;
		DoubledCoords::from(result_cube)
	}
}


// `FROM` IMPLEMENTATIONS ----------------------------------------------------------------------- //

impl<T> From<AxialCoords<T>> for DoubledCoords<T> where T: Add<Output=T> + Copy + Mul<Output=T> + NumCast {
	/// Creates a new doubled coordinate pair from the given axial coordinates, [as described in
	/// the article](https://www.redblobgames.com/grids/hexagons/#conversions-doubled)
    fn from(c: AxialCoords<T>) -> Self {
		let two: T = NumCast::from(2).unwrap();
        Self{ q: c.q * two + c.r, r: c.r }
    }
}

impl<T> From<CubeCoords<T>> for DoubledCoords<T> where T: Add<Output=T> + Copy + Mul<Output=T> + NumCast {
    fn from(c: CubeCoords<T>) -> Self {
        Self::from(AxialCoords::from(c))
    }
}

impl<T> From<OffsetCoords<T>> for DoubledCoords<T> where T: Add<Output=T> + BitAnd<Output=T> + Copy + Div<Output=T> + Mul<Output=T> + NumCast + Sub<Output=T> {
    fn from(c: OffsetCoords<T>) -> Self {
        Self::from(AxialCoords::from(c))
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
				let coord = DoubledCoords::splat(0);
				let adjacent_coords = coord.adjacent_coords();
				assert_eq!(6, adjacent_coords.len());
				assert!(adjacent_coords.contains(&DoubledCoords::new(2, 0)));
				assert!(adjacent_coords.contains(&DoubledCoords::new(1, 1)));
				assert!(adjacent_coords.contains(&DoubledCoords::new(-1, 1)));
				assert!(adjacent_coords.contains(&DoubledCoords::new(-2, 0)));
				assert!(adjacent_coords.contains(&DoubledCoords::new(-1, -1)));
				assert!(adjacent_coords.contains(&DoubledCoords::new(1, -1)));

				// test an off center coordinate
				let coord = DoubledCoords::new(3, 1);
				let adjacent_coords = coord.adjacent_coords();
				assert_eq!(6, adjacent_coords.len());
				assert!(adjacent_coords.contains(&DoubledCoords::new(4, 0)));
				assert!(adjacent_coords.contains(&DoubledCoords::new(5, 1)));
				assert!(adjacent_coords.contains(&DoubledCoords::new(4, 2)));
				assert!(adjacent_coords.contains(&DoubledCoords::new(4, 2)));
				assert!(adjacent_coords.contains(&DoubledCoords::new(2, 2)));
				assert!(adjacent_coords.contains(&DoubledCoords::new(1, 1)));
			}
		}

		#[test]
		fn from_axial_coords() {
			assert_eq!(DoubledCoords::new(0, 0), AxialCoords::new(0, 0).into());
			assert_eq!(DoubledCoords::new(2, 0), AxialCoords::new(1, 0).into());
			assert_eq!(DoubledCoords::new(4, 0), AxialCoords::new(2, 0).into());
			assert_eq!(DoubledCoords::new(1, 1), AxialCoords::new(0, 1).into());
			assert_eq!(DoubledCoords::new(3, 1), AxialCoords::new(1, 1).into());
			assert_eq!(DoubledCoords::new(0, 2), AxialCoords::new(-1, 2).into());
			assert_eq!(DoubledCoords::new(2, 2), AxialCoords::new(0, 2).into());
		}

		#[test]
		fn from_cube_coords() {
			assert_eq!(DoubledCoords::new(0, 0), CubeCoords::new(0, 0, 0).into());
			assert_eq!(DoubledCoords::new(2, 0), CubeCoords::new(1, 0, -1).into());
			assert_eq!(DoubledCoords::new(4, 0), CubeCoords::new(2, 0, -2).into());
			assert_eq!(DoubledCoords::new(1, 1), CubeCoords::new(0, 1, -1).into());
			assert_eq!(DoubledCoords::new(3, 1), CubeCoords::new(1, 1, -2).into());
			assert_eq!(DoubledCoords::new(0, 2), CubeCoords::new(-1, 2, -1).into());
			assert_eq!(DoubledCoords::new(2, 2), CubeCoords::new(0, 2, -2).into());
		}

		#[test]
		#[ignore]
		fn from_offset_coords() {
			assert_eq!(DoubledCoords::new(0, 0), OffsetCoords::new(0, 0).into());
			assert_eq!(DoubledCoords::new(1, 0), OffsetCoords::new(1, 0).into());
			assert_eq!(DoubledCoords::new(0, 1), OffsetCoords::new(0, 0).into());
		}
	}
}
