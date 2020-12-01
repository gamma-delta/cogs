use crate::int_coords::ICoord;

/// Four-way directions.
///
/// These start at North and increment counter-clockwise,
/// so you can convert them to integers with `as` and use them
/// in rotational calculations if you need.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
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

    /// Get this direction, rotated by this many steps clockwise.
    /// Negative numbers go counter-clockwise.
    ///
    /// ```
    /// # use cogs_gamedev::directions::Direction4;
    /// use Direction4::*;
    /// let north = North;
    /// assert_eq!(north.rotate(1), East);
    /// assert_eq!(north.rotate(2), South);
    /// assert_eq!(north.rotate(-1), West);
    /// assert_eq!(north.rotate(5).rotate(-11), South);
    /// ```
    pub fn rotate(self, steps_clockwise: isize) -> Self {
        let idx = self as isize;
        let new_idx =
            ((idx + steps_clockwise).rem_euclid(Self::DIRECTIONS.len() as isize)) as usize;
        Self::DIRECTIONS[new_idx]
    }

    /// Flip this direction.
    ///
    /// ```
    /// # use cogs_gamedev::directions::Direction4;
    /// use Direction4::*;
    /// assert_eq!(North.flip(), South);
    /// assert_eq!(West.flip(), East);
    /// assert_eq!(East.flip().flip(), East);
    /// ```
    pub fn flip(self) -> Self {
        self.rotate(2)
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
    /// # use cogs_gamedev::directions::Direction4;
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
    /// # use cogs_gamedev::directions::Direction4;
    /// # use cogs_gamedev::int_coords::ICoord;
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
}
