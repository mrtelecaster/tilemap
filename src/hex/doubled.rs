//! Doubled hex coordinates. Method for making pseudo-rectangular maps that's a bit more
//! mathematically elegant than the [Offset coordinate system](crate::hex::offset)

use std::ops::Add;

use crate::traits::TileCoords;



/// A coordinate pair for an offset coordinate hex map
#[derive(PartialEq)]
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

impl<T> TileCoords for DoubledCoords<T> where T: Add<Output=T> + Copy + From<isize> + PartialEq {
    fn adjacent_coords(&self) -> Vec<Self> where Self: Sized {
        vec![
			self + DoubledCoords::new((-1).into(), (-1).into()),
			self + DoubledCoords::new(1.into(), (-1).into()),
			self + DoubledCoords::new(2.into(), 0.into()),
			self + DoubledCoords::new(1.into(), 1.into()),
			self + DoubledCoords::new((-1).into(), 1.into()),
			self + DoubledCoords::new((-2).into(), 0.into()),
		]
    }
}

impl<T> Add for DoubledCoords<T> where T: Add<Output=T> + Copy {

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
	}
}
