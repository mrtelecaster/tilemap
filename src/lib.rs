//! # Tilemap
//!
//! Generic tile maps library for use developing games. This is meant for more of a board game map
//! where tiles represent spaces with different attributes that game pieces move through. It's
//! intended primarily for strategy games but can be adapted to other needs.
//! 
//! I am developing this for [a Bevy project] of mine, but now that this has been split out into its
//! own library I would like to keep it as engine-agnostic as possible. However, the way it's
//! structured will still probably favor Bevy development (for example, the default coordinate
//! directions), so keep that in mind if using this for another engine.

pub mod traits;
pub mod map;
pub mod hex;
