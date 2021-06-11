//! Integer-based coordinates.

use super::{Direction4, Direction8};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use std::{
    convert::TryFrom,
    convert::TryInto,
    num::TryFromIntError,
    ops::{Add, AddAssign, Mul, MulAssign},
};

/// Unsigned-int coordinates
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Coord {
    /// Make a new coord.
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    /// Get this as an index into an array representing a 2d array.
    ///
    /// (AKA, `y * width + x`.)
    pub fn to_2d_idx(self, width: usize) -> usize {
        self.y * width + self.x
    }

    /// Convert this into an ICoord.
    pub fn to_icoord(self) -> ICoord {
        self.into()
    }
}

impl Add for Coord {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Coord {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Mul<usize> for Coord {
    type Output = Self;
    fn mul(self, rhs: usize) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl MulAssign<usize> for Coord {
    fn mul_assign(&mut self, rhs: usize) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

/// Try to convert an ICoord to a Coord.
/// Will return Error if the ICoord has any negatives in it.
impl TryFrom<ICoord> for Coord {
    type Error = TryFromIntError;
    fn try_from(value: ICoord) -> Result<Self, Self::Error> {
        Ok(Self {
            x: value.x.try_into()?,
            y: value.y.try_into()?,
        })
    }
}

/// Signed-int coordinates
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ICoord {
    pub x: isize,
    pub y: isize,
}

impl ICoord {
    /// Create a new ICoord
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    /// Return the quadrant this coordinate is in.
    ///
    /// - 1: +X, +Y
    /// - 2: -X, +Y
    /// - 3: -X, -Y
    /// - 4: +X, -Y
    ///
    /// Zeroes are treated as positive.
    ///
    /// ```
    /// # use cogs_gamedev::grids::ICoord;
    /// assert_eq!(ICoord::new(4, 5).quadrant(), 1);
    /// assert_eq!(ICoord::new(-3, -2).quadrant(), 3);
    /// // Zero is treated as positive
    /// assert_eq!(ICoord::new(0, -8).quadrant(), 4);
    /// assert_eq!(ICoord::new(0, 0).quadrant(), 1);
    /// ```
    pub fn quadrant(self) -> usize {
        match (self.x >= 0, self.y >= 0) {
            (true, true) => 1,
            (false, true) => 2,
            (false, false) => 3,
            (true, false) => 4,
        }
    }

    /// Try to convert this to a Coord.
    /// Returns `Err(())` in case any part is negative.
    #[deprecated(note = "use `.try_into()")]
    #[allow(clippy::result_unit_err)]
    pub fn to_coord(self) -> Result<Coord, ()> {
        self.try_into().map_err(|_e| ())
    }
}

impl Add for ICoord {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for ICoord {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Add<Direction4> for ICoord {
    type Output = Self;
    fn add(self, rhs: Direction4) -> Self::Output {
        self + rhs.deltas()
    }
}

impl AddAssign<Direction4> for ICoord {
    fn add_assign(&mut self, rhs: Direction4) {
        *self += rhs.deltas();
    }
}

impl Add<Direction8> for ICoord {
    type Output = Self;
    fn add(self, rhs: Direction8) -> Self::Output {
        self + rhs.deltas()
    }
}

impl AddAssign<Direction8> for ICoord {
    fn add_assign(&mut self, rhs: Direction8) {
        *self += rhs.deltas();
    }
}

impl Mul<isize> for ICoord {
    type Output = Self;
    fn mul(self, rhs: isize) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl MulAssign<isize> for ICoord {
    fn mul_assign(&mut self, rhs: isize) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl From<Coord> for ICoord {
    fn from(value: Coord) -> Self {
        Self {
            x: value.x as isize,
            y: value.y as isize,
        }
    }
}
