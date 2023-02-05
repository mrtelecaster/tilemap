//! Pre-made types for square grid coordinate systems

use std::{fmt::Debug, ops::Add};

use crate::traits::TileCoords;



/// Basic square coordinates. Each tile has equal width and height, is uniformly spaced, and has 4
/// side neighbors and 8 corner neighbors (including the side neighbors)
#[derive(Debug, PartialEq)]
pub struct SquareCoords<T> {
	pub x: T,
	pub y: T,
}

impl<T> SquareCoords<T> {

	pub fn new(x: T, y: T) -> Self {
		Self{ x, y }
	}
}

impl<T> TileCoords for SquareCoords<T> where T: Add<Output=T> + Copy + Debug + From<isize> + PartialEq {

    fn adjacent_coords(&self) -> Vec<Self> where Self: Sized {
        vec![
			self + SquareCoords::new((-1).into(), (-1).into()),
			self + SquareCoords::new((-1).into(), (0).into()),
			self + SquareCoords::new((-1).into(), (1).into()),
			self + SquareCoords::new((0).into(), (-1).into()),
			self + SquareCoords::new((0).into(), (1).into()),
			self + SquareCoords::new((1).into(), (-1).into()),
			self + SquareCoords::new((1).into(), (0).into()),
			self + SquareCoords::new((1).into(), (1).into()),
		]
    }
}

impl<T> Add for SquareCoords<T> where T: Add<Output=T> {

    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self{
			x: self.x + rhs.x,
			y: self.y + rhs.y,
		}
    }
}

impl<T> Add<SquareCoords<T>> for &SquareCoords<T> where T: Add<Output=T> + Copy {

    type Output = SquareCoords<T>;

    fn add(self, rhs: SquareCoords<T>) -> Self::Output {
        Self::Output{
			x: self.x + rhs.x,
			y: self.y + rhs.y,
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
			fn adjacent_coords() {
				
				let coord = SquareCoords::new(0, 0);
				let adjacent_coords = coord.adjacent_coords();
				assert!(adjacent_coords.contains(&SquareCoords::new(-1, -1)));
				assert!(adjacent_coords.contains(&SquareCoords::new(-1, 0)));
				assert!(adjacent_coords.contains(&SquareCoords::new(-1, 1)));
				assert!(adjacent_coords.contains(&SquareCoords::new(0, -1)));
				assert!(adjacent_coords.contains(&SquareCoords::new(0, 1)));
				assert!(adjacent_coords.contains(&SquareCoords::new(1, -1)));
				assert!(adjacent_coords.contains(&SquareCoords::new(1, 0)));
				assert!(adjacent_coords.contains(&SquareCoords::new(1, 1)));
				assert_eq!(8, adjacent_coords.len());

				let coord = SquareCoords::new(3, -1);
				let adjacent_coords = coord.adjacent_coords();
				assert!(adjacent_coords.contains(&SquareCoords::new(2, -2)));
				assert!(adjacent_coords.contains(&SquareCoords::new(2, -1)));
				assert!(adjacent_coords.contains(&SquareCoords::new(2, 0)));
				assert!(adjacent_coords.contains(&SquareCoords::new(3, -2)));
				assert!(adjacent_coords.contains(&SquareCoords::new(3, 0)));
				assert!(adjacent_coords.contains(&SquareCoords::new(4, -2)));
				assert!(adjacent_coords.contains(&SquareCoords::new(4, -1)));
				assert!(adjacent_coords.contains(&SquareCoords::new(4, 0)));
				assert_eq!(8, adjacent_coords.len());
			}
		}
	}
}
