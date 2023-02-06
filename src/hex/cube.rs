//! Cube coordinates. Has simpler math than axial coords, but takes up more space.

use std::{fmt::Debug, ops::{Add, Sub, Neg, BitAnd, Div}};
use num::{NumCast, Integer};
use crate::{
	traits::TileCoords,
	hex::{AxialCoords, DoubledCoords, OffsetCoords},
};



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

	pub fn is_valid(&self) -> bool where T: Copy + Neg<Output=T> + PartialEq + Sub<Output=T> {
		self.s == -self.q - self.r
	}
}


// TILE COORDS TRAIT IMPLEMENTATION ------------------------------------------------------------- //

impl<T> TileCoords<T> for CubeCoords<T> where T: Add<Output=T> + Copy + Debug + NumCast + PartialEq {

    fn adjacent_coords(&self) -> Vec<Self> where Self: Sized {
		let one: T = NumCast::from(1).unwrap();
		let zero: T = NumCast::from(0).unwrap();
		let neg_one: T = NumCast::from(-1).unwrap();
        vec![
			self + CubeCoords::new(one, neg_one, zero),
			self + CubeCoords::new(one, zero, neg_one),
			self + CubeCoords::new(zero, one, neg_one),
			self + CubeCoords::new(neg_one, one, zero),
			self + CubeCoords::new(neg_one, zero, one),
			self + CubeCoords::new(zero, neg_one, one),
		]
    }

    fn distance<D>(&self, other: &Self) -> D where D: Integer {
        todo!()
    }
}


// `std::ops` IMPLEMENTATIONS ------------------------------------------------------------------- //

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

impl<T> Sub for CubeCoords<T> where T: Sub<Output=T> {

    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self{ q: self.q - rhs.q, r: self.r - rhs.r, s: self.s - rhs.s }
    }
}


// `FROM` IMPLEMENTATIONS ----------------------------------------------------------------------- //

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

impl<T> From<DoubledCoords<T>> for CubeCoords<T>
where T: Add<Output=T> + BitAnd<Output=T> + Copy + Div<Output=T> + Neg<Output=T> + NumCast + Sub<Output=T>
{
    fn from(c: DoubledCoords<T>) -> Self {
        Self::from(OffsetCoords::from(c))
    }
}

impl<T> From<OffsetCoords<T>> for CubeCoords<T>
where T: BitAnd<Output=T> + Copy + Div<Output=T> + Neg<Output=T> + NumCast + Sub<Output=T>
{
	/// Creates a new cube coordinate set from the given offset coordinates, [as described in the article](https://www.redblobgames.com/grids/hexagons/#conversions-offset)
    fn from(c: OffsetCoords<T>) -> Self {
        Self::from(AxialCoords::from(c))
    }
}



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
		#[ignore]
		fn from_doubled_coords() {
			assert_eq!(CubeCoords::new(0, 0, 0), DoubledCoords::new(0, 0).into());
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
