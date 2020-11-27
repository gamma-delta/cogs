//! Contains 4-way and 8-way directions.
//!
//! `Direction4` has north, east, south, and west. (Remember, Never Eat Soggy Waffles.)
//!
//! `Direction8` has north, northeast, east, southeast, south, southwest, west, and northwest.

mod dir4;
pub use dir4::Direction4;

mod dir8;
pub use dir8::Direction8;
