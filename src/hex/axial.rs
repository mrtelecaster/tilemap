//! Axial hex coordinates. More space efficient than cube but math is a bit of a pain.

use std::{fmt::Debug, ops::{Add, Sub, BitAnd, Div, Neg}};
use num::{Integer, NumCast, Signed};
use crate::{traits::TileCoords, hex::{CubeCoords, OffsetCoords}};



#[derive(Debug, PartialEq)]
pub struct AxialCoords<T> {
	pub q: T,
	pub r: T,
}

impl<T> AxialCoords<T> {

	/// Create a new axial coordinate pair with the given Q and R coordinates
	pub fn new(q: T, r: T) -> Self {
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
	pub fn splat(val: T) -> Self where T: Copy {
		Self::new(val, val)
	}
}


// TILE COORDS TRAIT IMPLEMENTATION ------------------------------------------------------------- //

impl<T> TileCoords<T> for AxialCoords<T> where T: Copy + Debug + NumCast + Signed {
    fn adjacent_coords(&self) -> Vec<Self> where Self: Sized {
		let one: T = NumCast::from(1).unwrap();
		let zero: T = NumCast::from(0).unwrap();
		let neg_one: T = NumCast::from(-1).unwrap();
        vec![
			self + AxialCoords::new(one, zero),
			self + AxialCoords::new(zero, one),
			self + AxialCoords::new(neg_one, one),
			self + AxialCoords::new(neg_one, zero),
			self + AxialCoords::new(zero, neg_one),
			self + AxialCoords::new(one, neg_one),
		]
    }

    fn distance<D>(&self, other: &Self) -> D where D: Integer + From<T> {
        CubeCoords::from(self).distance(&CubeCoords::from(other))
    }
}


// STD OPS IMPLEMENTATIONS ---------------------------------------------------------------------- //

impl<T> Add for AxialCoords<T> where T: Add<Output=T> {

    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output{
			q: self.q + rhs.q,
			r: self.r + rhs.r,
		}
    }
}

impl<T> Add<AxialCoords<T>> for &AxialCoords<T> where T: Add<Output=T> + Copy {

	type Output = AxialCoords<T>;

	fn add(self, rhs: AxialCoords<T>) -> Self::Output {
		Self::Output{
			q: self.q + rhs.q,
			r: self.r + rhs.r,
		}
	}
}

impl<T> Sub for AxialCoords<T> where T: Copy + Neg<Output=T> + Sub<Output=T> {

	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		Self::from(CubeCoords::from(self) - CubeCoords::from(rhs))
	}
}



// `FROM` TRAIT --------------------------------------------------------------------------------- //

impl<T> From<CubeCoords<T>> for AxialCoords<T>
{
	/// Creates a new axial coordinate from the given cube coordinate [as described here]
	/// (https://www.redblobgames.com/grids/hexagons/#conversions-axial)
    fn from(c: CubeCoords<T>) -> Self {
		Self { q: c.q, r: c.r }
    }
}

impl<T> From<OffsetCoords<T>> for AxialCoords<T>
where T: BitAnd<Output=T> + Copy + Div<Output=T> + NumCast + Sub<Output=T>
{
	/// Creates a new axial coordinate pair from the given set of offset coordinates [as described
	/// in the article](https://www.redblobgames.com/grids/hexagons/#conversions-offset)
    fn from(c: OffsetCoords<T>) -> Self {
		let one = NumCast::from(1).unwrap();
		let two = NumCast::from(2).unwrap();
        let q = c.q - (c.r - (c.r & one)) / two;
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
