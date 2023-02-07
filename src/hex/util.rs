use num::{traits::real::Real, Integer, NumCast, Signed};


/// Rounds continuous fractional cube coordinates to discrete integer coordinates. Garunteed to
/// always return a valid set of coordinates.
pub fn cube_round<T, U>(q: T, r: T, s: T) -> (U, U, U)
where T: Real + NumCast, U: Copy + Integer + NumCast + Signed
{
	let mut int_q: U = U::from(q.round()).unwrap();
	let mut int_r: U = U::from(r.round()).unwrap();
	let mut int_s: U = U::from(s.round()).unwrap();

	let q_diff: T = (T::from(int_q).unwrap() - q).abs();
	let r_diff: T = (T::from(int_r).unwrap() - r).abs();
	let s_diff: T = (T::from(int_s).unwrap() - s).abs();

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