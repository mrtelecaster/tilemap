//! Axial hex coordinates. More space efficient than cube but math is a bit of a pain.

use std::ops::Add;

use crate::traits::TileCoords;



#[derive(PartialEq)]
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

impl<T> TileCoords for AxialCoords<T> where T: Add<Output=T> + Copy + From<isize> + PartialEq {
    fn adjacent_coords(&self) -> Vec<Self> where Self: Sized {
        vec![
			self + AxialCoords::new(1.into(), 0.into()),
			self + AxialCoords::new(0.into(), 1.into()),
			self + AxialCoords::new((-1).into(), 1.into()),
			self + AxialCoords::new((-1).into(), 0.into()),
			self + AxialCoords::new(0.into(), (-1).into()),
			self + AxialCoords::new(1.into(), (-1).into()),
		]
    }
}

impl<T> Add for AxialCoords<T> where T: Add<Output=T> + Copy {

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
		}
	}
}
