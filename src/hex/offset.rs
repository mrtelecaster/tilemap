//! Offset hex coordinates. Simple method for making pseudo-rectangular maps

use std::ops::Add;

use crate::traits::TileCoords;



/// A coordinate pair for an offset coordinate hex map
#[derive(PartialEq)]
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

impl<T> TileCoords<T> for OffsetCoords<T> where T: Add<Output=T> + Copy + From<isize> {
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
	}
}
