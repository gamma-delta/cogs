use super::ICoord;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A rectangle with integer values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IRect {
    pub left: isize,
    pub top: isize,
    pub width: usize,
    pub height: usize,
}

impl IRect {
    pub fn new(left: isize, top: isize, width: usize, height: usize) -> Self {
        Self {
            left,
            top,
            width,
            height,
        }
    }

    /// Return a new rectangle centered at the given position with the given w/h
    pub fn centered(center: ICoord, width: usize, height: usize) -> Self {
        let top = center.y - height as isize / 2;
        let left = center.x - width as isize / 2;
        Self::new(left, top, width, height)
    }

    /// Does this rect contain the pos?
    ///
    /// Points on the boundary count.
    pub fn contains(&self, pos: ICoord) -> bool {
        self.top <= pos.y && self.bottom() >= pos.y && self.left <= pos.x && self.right() >= pos.x
    }

    pub fn area(&self) -> usize {
        self.width * self.height
    }

    /// Iterator through all the positions in the rect.
    /// It goes in reading order: left-to-right, top-to-bottom.
    ///
    /// (Sorry hebrew speakers)
    pub fn contained_coords(&self) -> RectIter {
        RectIter::new(*self)
    }

    pub fn right(&self) -> isize {
        self.left + self.width as isize - 1
    }

    pub fn bottom(&self) -> isize {
        self.top + self.height as isize - 1
    }

    pub fn shifted(self, by: ICoord) -> IRect {
        IRect {
            top: self.top + by.y,
            left: self.left + by.x,
            ..self
        }
    }
}

impl std::ops::Add<ICoord> for IRect {
    type Output = IRect;
    fn add(self, rhs: ICoord) -> Self::Output {
        self.shifted(rhs)
    }
}

pub struct RectIter {
    rect: IRect,
    cursor: ICoord,
    exhausted: bool,
}

impl RectIter {
    pub fn new(rect: IRect) -> Self {
        Self {
            rect,
            cursor: ICoord::new(rect.left, rect.top),
            // if the rect is just a line, don't output anything.
            exhausted: rect.width == 0 || rect.height == 0,
        }
    }
}

impl Iterator for RectIter {
    type Item = ICoord;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.exhausted {
            let out = self.cursor;

            self.cursor.x += 1;
            if !self.rect.contains(self.cursor) {
                self.cursor.x = self.rect.left;
                self.cursor.y += 1;
                if !self.rect.contains(self.cursor) {
                    self.exhausted = true;
                    // And return this one final point
                    // next go-around we return None
                }
            }

            Some(out)
        } else {
            None
        }
    }
}
