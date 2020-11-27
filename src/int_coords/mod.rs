//! Integer-based coordinates.

use std::{
    convert::TryFrom,
    num::TryFromIntError,
    ops::{Add, AddAssign},
};

/// Unsigned-int coordinates
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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

/// Try to convert an ICoord to a Coord.
/// Will return Error if the ICoord has any negatives in it.
impl TryFrom<ICoord> for Coord {
    type Error = TryFromIntError;
    fn try_from(value: ICoord) -> Result<Self, Self::Error> {
        use std::convert::TryInto;
        Ok(Self {
            x: value.x.try_into()?,
            y: value.y.try_into()?,
        })
    }
}

/// Signed-int coordinates
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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
    /// # use cogs_gamedev::int_coords::ICoord;
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

impl From<Coord> for ICoord {
    fn from(value: Coord) -> Self {
        Self {
            x: value.x as isize,
            y: value.y as isize,
        }
    }
}