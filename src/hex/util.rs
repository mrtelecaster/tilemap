//! Helper functions that either don't belong with a particular module, or are easier to read and
//! test as a standalone function


/// Rounds continuous fractional cube coordinates to discrete integer coordinates. Garunteed to
/// always return a valid set of coordinates.
pub fn cube_round(q: f32, r: f32, s: f32) -> (isize, isize, isize)
{
	let mut int_q = q.round() as isize;
	let mut int_r = r.round() as isize;
	let mut int_s = s.round() as isize;

	let q_diff = (int_q as f32 - q).abs();
	let r_diff = (int_r as f32 - r).abs();
	let s_diff = (int_s as f32 - s).abs();

	if q_diff > r_diff && q_diff > s_diff {
		int_q = -int_r - int_s;
	} else if r_diff > s_diff {
		int_r = -int_q - int_s;
	} else {
		int_s = -int_q - int_r;
	}

	(int_q, int_r, int_s)
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn round() {
		assert_eq!((0, 0, 0), cube_round(0.0, 0.0, 0.0));
		assert_eq!((0, 0, 0), cube_round(0.4, -0.4, 0.0));
		assert_eq!((1, -1, 0), cube_round(0.6, -0.4, 0.0));
		assert_eq!((1, -1, 0), cube_round(0.6, -0.6, 0.0));
		assert_eq!((1, -1, 0), cube_round(1.4, -1.4, 0.0));
		assert_eq!((2, -1, -1), cube_round(2.0, -1.0, 0.0));
		assert_eq!((3, -2, -1), cube_round(3.0, -2.0, 0.0));
		assert_eq!((-1, 4, -3), cube_round(-1.0, 4.0, 0.0));
	}
}