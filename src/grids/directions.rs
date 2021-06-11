use super::ICoord;

use enum_map::Enum;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Four-way directions.
///
/// These start at North and increment counter-clockwise,
/// so you can convert them to integers with `as` and use them
/// in rotational calculations if you need.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Enum)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Direction4 {
    North,
    East,
    South,
    West,
}

impl Direction4 {
    /// All the directions in order.
    /// This is used internally for rotations and flips.
    /// I made it public just in case it's helpful for you the programmer.
    pub const DIRECTIONS: [Direction4; 4] = [
        Direction4::North,
        Direction4::East,
        Direction4::South,
        Direction4::West,
    ];

    /// Rotate this by the given amount.
    ///
    /// ```
    /// # use cogs_gamedev::grids::{Direction4, Rotation};
    /// use Direction4::*;
    /// use Rotation::*;
    ///
    /// assert_eq!(North.rotate(Clockwise), East);
    /// assert_eq!(North.rotate(CounterClockwise), West);
    /// ```
    pub fn rotate(self, rot: Rotation) -> Self {
        self.rotate_by(rot.steps_clockwise())
    }

    /// Get this direction, rotated by this many steps clockwise.
    /// Negative numbers go counter-clockwise.
    ///
    /// ```
    /// # use cogs_gamedev::grids::Direction4;
    /// use Direction4::*;
    /// assert_eq!(North.rotate_by(1), East);
    /// assert_eq!(North.rotate_by(2), South);
    /// assert_eq!(North.rotate_by(-1), West);
    /// assert_eq!(North.rotate_by(5).rotate_by(-11), South);
    /// ```
    pub fn rotate_by(self, steps_clockwise: isize) -> Self {
        let idx = self as isize;
        let new_idx =
            ((idx + steps_clockwise).rem_euclid(Self::DIRECTIONS.len() as isize)) as usize;
        Self::DIRECTIONS[new_idx]
    }

    /// Flip this direction.
    ///
    /// ```
    /// # use cogs_gamedev::grids::Direction4;
    /// use Direction4::*;
    /// assert_eq!(North.flip(), South);
    /// assert_eq!(West.flip(), East);
    /// assert_eq!(East.flip().flip(), East);
    /// ```
    pub fn flip(self) -> Self {
        self.rotate_by(2)
    }

    /// Get this direction in radians.
    ///
    /// This uses trigonometric + graphical standard, where:
    /// - 0 radians is to the right
    /// - Positive radians increment *clockwise*. NOTE: this is opposite from normal trig,
    /// but makes sense in computer graphics where +Y is downwards.
    ///
    /// If you need it in degrees just call `.to_degrees` on the result.
    ///
    /// ```
    /// # use cogs_gamedev::grids::Direction4;
    /// use Direction4::*;
    /// use std::f32::consts::TAU;
    ///
    /// let north_radians = North.radians();
    /// assert!((north_radians - (TAU / 4.0 * 3.0)).abs() < 1e-10);
    ///
    /// let west_radians = West.radians();
    /// assert!((west_radians - (TAU / 4.0 * 2.0)).abs() < 1e-10);
    ///
    /// ```
    pub fn radians(self) -> f32 {
        ((self as i8) - 1).rem_euclid(4) as f32 * std::f32::consts::TAU / 4.0
    }

    /// Get the deltas a step in this direction would result in, as a ICoord.
    ///
    /// ```
    /// # use cogs_gamedev::grids::Direction4;
    /// # use cogs_gamedev::grids::ICoord;
    /// use Direction4::*;
    ///
    /// assert_eq!(North.deltas(), ICoord {x: 0, y: -1});
    /// assert_eq!(West.deltas(), ICoord {x: -1, y: 0});
    /// ```
    pub fn deltas(self) -> ICoord {
        let (x, y) = match self {
            Direction4::North => (0, -1),
            Direction4::East => (1, 0),
            Direction4::South => (0, 1),
            Direction4::West => (-1, 0),
        };
        ICoord { x, y }
    }

    /// See if this direction points horizontally (ie, is `East` or `West`).
    ///
    /// ```
    /// # use cogs_gamedev::grids::Direction4;
    /// use Direction4::*;
    ///
    /// assert!(East.is_horizontal());
    /// assert!(!South.is_horizontal());
    /// ```
    pub fn is_horizontal(self) -> bool {
        matches!(self, Direction4::East | Direction4::West)
    }

    /// See if this direction points vertically (ie, is `North` or `South`).
    ///
    /// ```
    /// # use cogs_gamedev::grids::Direction4;
    /// use Direction4::*;
    ///
    /// assert!(North.is_vertical());
    /// assert!(!West.is_vertical());
    /// ```
    pub fn is_vertical(self) -> bool {
        matches!(self, Direction4::North | Direction4::South)
    }
}

/// Eight-way directions.
///
/// These start at North and increment counter-clockwise,
/// so you can convert them to integers with `as` and use them
/// in rotational calculations if you need.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Enum)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Direction8 {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction8 {
    /// All the directions in order.
    /// This is used internally for rotations and flips.
    /// I made it public just in case it's helpful for you the programmer.
    pub const DIRECTIONS: [Direction8; 8] = [
        Direction8::North,
        Direction8::NorthEast,
        Direction8::East,
        Direction8::SouthEast,
        Direction8::South,
        Direction8::SouthWest,
        Direction8::West,
        Direction8::NorthWest,
    ];

    /// Rotate this by the given amount.
    ///
    /// ```
    /// # use cogs_gamedev::grids::{Direction8, Rotation};
    /// use Direction8::*;
    /// use Rotation::*;
    ///
    /// assert_eq!(NorthEast.rotate(Clockwise), East);
    /// assert_eq!(South.rotate(CounterClockwise), SouthEast);
    /// ```
    pub fn rotate(self, rot: Rotation) -> Self {
        self.rotate_by(rot.steps_clockwise())
    }

    /// Get this direction, rotated by this many steps clockwise.
    /// Negative numbers go counter-clockwise.
    ///
    /// ```
    /// # use cogs_gamedev::grids::Direction8;
    /// use Direction8::*;
    /// let north = North;
    /// assert_eq!(north.rotate_by(1), NorthEast);
    /// assert_eq!(north.rotate_by(2), East);
    /// assert_eq!(north.rotate_by(-1), NorthWest);
    /// assert_eq!(north.rotate_by(4), South);
    /// assert_eq!(north.rotate_by(5).rotate_by(-11), East);
    /// ```
    pub fn rotate_by(self, steps_clockwise: isize) -> Self {
        let idx = self as isize;
        let new_idx =
            ((idx + steps_clockwise).rem_euclid(Self::DIRECTIONS.len() as isize)) as usize;
        Self::DIRECTIONS[new_idx]
    }

    /// Flip this direction.
    ///
    /// ```
    /// # use cogs_gamedev::grids::Direction8;
    /// use Direction8::*;
    /// assert_eq!(North.flip(), South);
    /// assert_eq!(West.flip(), East);
    /// assert_eq!(SouthWest.flip(), NorthEast);
    /// assert_eq!(East.flip().flip(), East);
    /// ```
    pub fn flip(self) -> Self {
        self.rotate_by(4)
    }

    /// Get this direction in radians.
    ///
    /// This uses trigonometric + graphical standard, where:
    /// - 0 radians is to the right
    /// - Positive radians increment *clockwise*. NOTE: this is opposite from normal trig,
    /// but makes sense in computer graphics where +Y is downwards.
    ///
    /// If you need it in degrees just call `.to_degrees` on the result.
    ///
    /// ```
    /// # use cogs_gamedev::grids::Direction8;
    /// use Direction8::*;
    /// use std::f32::consts::TAU;
    ///
    /// let north_radians = North.radians();
    /// assert!((north_radians - (TAU / 8.0 * 6.0)).abs() < 1e-10);
    ///
    /// let west_radians = West.radians();
    /// assert!((west_radians - (TAU / 8.0 * 4.0)).abs() < 1e-10);
    ///
    /// let southeast_radians = SouthEast.radians();
    /// assert!((southeast_radians - (TAU / 8.0)).abs() < 1e-10);
    ///
    /// ```
    pub fn radians(self) -> f32 {
        ((self as i8) - 2).rem_euclid(8) as f32 * std::f32::consts::TAU / 8.0
    }

    /// Get the deltas a step in this direction would result in,
    /// as an ICoord.
    ///
    /// ```
    /// # use cogs_gamedev::grids::Direction8;
    /// # use cogs_gamedev::grids::ICoord;
    /// use Direction8::*;
    ///
    /// assert_eq!(East.deltas(), ICoord {x: 1, y: 0});
    /// assert_eq!(NorthWest.deltas(), ICoord {x: -1, y: -1});
    /// ```
    pub fn deltas(self) -> ICoord {
        let (x, y) = match self {
            Direction8::North => (0, -1),
            Direction8::NorthEast => (1, -1),
            Direction8::East => (1, 0),
            Direction8::SouthEast => (1, 1),
            Direction8::South => (0, 1),
            Direction8::SouthWest => (-1, 1),
            Direction8::West => (-1, 0),
            Direction8::NorthWest => (-1, -1),
        };
        ICoord { x, y }
    }
}

/// 2-way rotations: clockwise or counterclockwise.
/// These don't indicate any specific angle by themselves, only in relation to something.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Enum)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Rotation {
    Clockwise,
    CounterClockwise,
}

impl Rotation {
    /// Get the number of steps clockwise this does.
    /// - `Clockwise` is 1
    /// - `CounterClockwise` is -1
    pub fn steps_clockwise(&self) -> isize {
        match self {
            Rotation::Clockwise => 1,
            Rotation::CounterClockwise => -1,
        }
    }
}
