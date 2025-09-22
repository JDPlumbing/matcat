//! # matcat
//!
//! A compact material catalog system.
//!
//! Materials are identified by a 5-byte code (`MatCatId`) and deterministically
//! expanded into full property sets (`MatProps`) using procedural generation.
//!
//! Public API is kept flat: just `use matcat::...`.

pub mod materials;
pub mod category_ranges;

pub use materials::{
    MatCatId,
    MatProps,
    props_for,
    find_closest_material,
};

pub use category_ranges::*;
