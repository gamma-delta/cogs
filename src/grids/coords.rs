//! Integer-based coordinates.

use super::{Direction4, Direction8};

use itertools::Itertools;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use std::{
    convert::TryFrom,
    convert::TryInto,
    fmt::Display,
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
        // what did you think i was kidding or something
        self.y * width + self.x
    }

    /// Convert this into an ICoord.
    pub fn to_icoord(self) -> ICoord {
        self.into()
    }

    /// Get a list of this coordinate's orthagonal neighbors.
    /// They are given in clockwise order starting with the neighbor to the north,
    /// as if each of [`Direction4::DIRECTIONS`] had been added to them.
    ///
    /// If a neighbor is out of bounds, it is skipped in the output.
    ///
    /// There may be 2, 3, or 4 neighbors:
    /// - 2 if this is at `(0, 0)`
    /// - 3 if this is on an edge (`x` or `y` are 0)
    /// - 4 otherwise.
    ///
    /// ```
    /// # use cogs_gamedev::grids::{Coord, Direction4};
    ///
    /// assert_eq!(
    ///     Coord::new(5, 7).neighbors4(),
    ///     &[
    ///         Coord::new(5, 6),
    ///         Coord::new(6, 7),
    ///         Coord::new(5, 8),
    ///         Coord::new(4, 7),
    ///     ]
    /// );
    ///
    /// // May return fewer than 4 neighbors
    /// assert_eq!(
    ///     Coord::new(0, 5).neighbors4(),
    ///     &[
    ///         Coord::new(0, 4),
    ///         Coord::new(1, 5),
    ///         Coord::new(0, 6),
    ///         // Skip (-1, 5) for being out of bounds
    ///     ]
    /// );
    /// ```
    ///
    /// [`Direction4::DIRECTIONS`]: super::Direction4::DIRECTIONS
    pub fn neighbors4(self) -> Vec<Coord> {
        Direction4::DIRECTIONS
            .iter()
            .filter_map(|dir| {
                let iself = self.to_icoord();
                let ineighbor = iself + *dir;
                ineighbor.to_coord() // conveniently already returns an option.
            })
            .collect_vec()
    }

    /// Get a list of this coordinate's orthagonal and diagonal neighbors.
    /// They are given in clockwise order starting with the neighbor to the north,
    /// as if each of [`Direction8::DIRECTIONS`] had been added to them.
    ///
    /// If a neighbor is out of bounds, it is skipped in the output.
    ///
    /// There may be 3, 5, or 8 neighbors:
    /// - 3 if this is at `(0, 0)`
    /// - 5 if this is on an edge (`x` or `y` are 0)
    /// - 8 otherwise.
    ///
    /// ```
    /// # use cogs_gamedev::grids::Coord;
    /// # use cogs_gamedev::grids::Direction8;
    ///
    /// assert_eq!(
    ///     Coord::new(5, 7).neighbors8(),
    ///     [
    ///         Coord::new(5, 6),
    ///         Coord::new(6, 6),
    ///         Coord::new(6, 7),
    ///         Coord::new(6, 8),
    ///         Coord::new(5, 8),
    ///         Coord::new(4, 8),
    ///         Coord::new(4, 7),
    ///         Coord::new(4, 6),
    ///     ]
    /// );
    ///
    /// // May return fewer than 8 neighbors
    /// assert_eq!(
    ///     Coord::new(0, 5).neighbors8(),
    ///     &[
    ///         Coord::new(0, 4),
    ///         Coord::new(1, 4),
    ///         Coord::new(1, 5),
    ///         Coord::new(1, 6),
    ///         Coord::new(0, 6),
    ///         // Skip (-1, 6) for being out of bounds
    ///         // Skip (-1, 5)
    ///         // Skip (-1, 4)
    ///     ]
    /// );
    /// ```
    ///
    /// [`Direction8::DIRECTIONS`]: super::Direction8::DIRECTIONS
    pub fn neighbors8(self) -> Vec<Coord> {
        Direction8::DIRECTIONS
            .iter()
            .filter_map(|dir| {
                let iself = self.to_icoord();
                let ineighbor = iself + *dir;
                ineighbor.to_coord() // conveniently already returns an option.
            })
            .collect_vec()
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

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
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
    /// Returns `None` in case any part is negative.
    pub fn to_coord(self) -> Option<Coord> {
        self.try_into().ok()
    }

    /// Get a list of this coordinate's orthagonal neighbors.
    /// They are given in clockwise order starting with the neighbor to the north,
    /// as if each of [`Direction4::DIRECTIONS`] had been added to them.
    ///
    /// ```
    /// # use cogs_gamedev::grids::ICoord;
    /// # use cogs_gamedev::grids::Direction4;
    ///
    /// assert_eq!(
    ///     ICoord::new(5, 7).neighbors4(),
    ///     [
    ///         ICoord::new(5, 6),
    ///         ICoord::new(6, 7),
    ///         ICoord::new(5, 8),
    ///         ICoord::new(4, 7),
    ///     ]
    /// );
    ///
    /// let origin = ICoord::new(-7, -12);
    /// assert_eq!(
    ///     origin.neighbors4()[..],
    ///     Direction4::DIRECTIONS.iter().map(|dir| origin + *dir).collect::<Vec<_>>()[..],
    /// );
    /// ```
    ///
    /// [`Direction4::DIRECTIONS`]: super::Direction4::DIRECTIONS
    pub fn neighbors4(self) -> [ICoord; 4] {
        [
            self + Direction4::North,
            self + Direction4::East,
            self + Direction4::South,
            self + Direction4::West,
        ]
    }

    /// Get a list of this coordinate's orthagonal and diagonal neighbors.
    /// They are given in clockwise order starting with the neighbor to the north,
    /// as if each of [`Direction8::DIRECTIONS`] had been added to them.
    ///
    /// ```
    /// # use cogs_gamedev::grids::ICoord;
    /// # use cogs_gamedev::grids::Direction8;
    ///
    /// assert_eq!(
    ///     ICoord::new(5, 7).neighbors8(),
    ///     [
    ///         ICoord::new(5, 6),
    ///         ICoord::new(6, 6),
    ///         ICoord::new(6, 7),
    ///         ICoord::new(6, 8),
    ///         ICoord::new(5, 8),
    ///         ICoord::new(4, 8),
    ///         ICoord::new(4, 7),
    ///         ICoord::new(4, 6),
    ///     ]
    /// );
    ///
    /// let origin = ICoord::new(-7, -12);
    /// assert_eq!(
    ///     origin.neighbors8()[..],
    ///     Direction8::DIRECTIONS.iter().map(|dir| origin + *dir).collect::<Vec<_>>()[..],
    /// );
    /// ```
    ///
    /// [`Direction8::DIRECTIONS`]: super::Direction8::DIRECTIONS
    pub fn neighbors8(self) -> [ICoord; 8] {
        [
            self + Direction8::North,
            self + Direction8::NorthEast,
            self + Direction8::East,
            self + Direction8::SouthEast,
            self + Direction8::South,
            self + Direction8::SouthWest,
            self + Direction8::West,
            self + Direction8::NorthWest,
        ]
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

impl Display for ICoord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
