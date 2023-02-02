//! Pre-made types for square grid coordinate systems



/// Basic square coordinates. Each tile has equal width and height, is uniformly spaced, and has 4
/// side neighbors and 8 corner neighbors (including the side neighbors)
pub struct SquareCoords<T> {
	pub x: T,
	pub y: T,
}