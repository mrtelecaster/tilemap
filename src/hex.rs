//! Pre-made types for hexagonal coordinate systems
//! 
//! Made referencing the fantastic [*Hexagonal Grids* article](https://www.redblobgames.com/grids/hexagons)
//! at [Red Blob Games](https://www.redblobgames.com/)

use std::ops::{Add, Sub};
use num::{NumCast, Integer, Signed};
use crate::traits::*;


/// Axial hexagon coordinate system
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AxialCoords<T> where T: Integer {
	pub q: T,
	pub r: T,
}

impl<T> AxialCoords<T> where T: Integer {
	pub fn new(q: T, r: T) -> Self {
		Self{ q, r }
	}
}

impl<T> TileCoords for AxialCoords<T> where T: Copy + Integer + NumCast {

    fn adjacent(&self) -> Vec<Self> where Self: Sized {
		let one = NumCast::from(1).unwrap();
        vec![
			AxialCoords::new(self.q, self.r - one),
			AxialCoords::new(self.q + one, self.r - one),
			AxialCoords::new(self.q + one, self.r),
			AxialCoords::new(self.q, self.r + one),
			AxialCoords::new(self.q - one, self.r + one),
			AxialCoords::new(self.q - one, self.r),
		]
    }

    fn tile_distance<D>(&self, other: &Self) -> D where D: Copy + Integer + Signed + NumCast {

		let delta = self - other;
		let q: D = NumCast::from(delta.q).unwrap();
		let r = NumCast::from(delta.r).unwrap();
		let two = NumCast::from(2).unwrap();
		(q.abs() + (q + r).abs() + r.abs()) / two
    }
}

impl<T> Add for AxialCoords<T>
where T: Integer {

    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self{
			q: self.q + rhs.q,
			r: self.r + rhs.r,
		}
    }
}

impl<T> Sub for AxialCoords<T>
where T: Integer {

    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self{
			q: self.q - rhs.q,
			r: self.r - rhs.r,
		}
    }
}

impl<'a, T> Sub<&'a AxialCoords<T>> for &'a AxialCoords<T>
where T: Copy + Integer {

	type Output = AxialCoords<T>;

	fn sub(self, rhs: Self) -> Self::Output {
		AxialCoords{
			q: self.q - rhs.q,
			r: self.r - rhs.r,
		}
	}
}



#[cfg(test)]
mod tests {
	use super::*;
	mod axial_coords {
		use super::*;
		mod traits {
			use super::*;
			mod tile_coords {

				use super::*;

				#[test]
				fn adjacent() {
					let coord = AxialCoords::new(0, 0);
					let adjacent_coords = coord.adjacent();
					assert_eq!(adjacent_coords.len(), 6);
					assert!(adjacent_coords.contains(&AxialCoords::new(0, -1)));
					assert!(adjacent_coords.contains(&AxialCoords::new(1, -1)));
					assert!(adjacent_coords.contains(&AxialCoords::new(1, 0)));
					assert!(adjacent_coords.contains(&AxialCoords::new(0, 1)));
					assert!(adjacent_coords.contains(&AxialCoords::new(-1, 1)));
					assert!(adjacent_coords.contains(&AxialCoords::new(-1, 0)));
					

					let coord = AxialCoords::new(-1, 2);
					let adjacent_coords = coord.adjacent();
					assert_eq!(adjacent_coords.len(), 6);
					assert!(adjacent_coords.contains(&AxialCoords::new(-1, 1)));
					assert!(adjacent_coords.contains(&AxialCoords::new(0, 1)));
					assert!(adjacent_coords.contains(&AxialCoords::new(0, 2)));
					assert!(adjacent_coords.contains(&AxialCoords::new(-1, 3)));
					assert!(adjacent_coords.contains(&AxialCoords::new(-2, 3)));
					assert!(adjacent_coords.contains(&AxialCoords::new(-2, 2)));
				}

				#[test]
				fn distance() {
					let center = AxialCoords::new(0, 0);

					assert_eq!(0, AxialCoords::new(0, 0).tile_distance(&center));

					assert_eq!(1, AxialCoords::new(1, 0).tile_distance(&center));
					assert_eq!(1, AxialCoords::new(1, -1).tile_distance(&center));
					assert_eq!(1, AxialCoords::new(0, -1).tile_distance(&center));
					assert_eq!(1, AxialCoords::new(-1, 0).tile_distance(&center));
					assert_eq!(1, AxialCoords::new(-1, 1).tile_distance(&center));
					assert_eq!(1, AxialCoords::new(0, 1).tile_distance(&center));

					assert_eq!(2, AxialCoords::new(0, -2).tile_distance(&center));
					assert_eq!(2, AxialCoords::new(1, -2).tile_distance(&center));
					assert_eq!(2, AxialCoords::new(2, -2).tile_distance(&center));
					assert_eq!(2, AxialCoords::new(2, -1).tile_distance(&center));
					assert_eq!(2, AxialCoords::new(2, 0).tile_distance(&center));
				}
			}
		}
	}
}