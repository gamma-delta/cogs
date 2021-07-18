//! A collection of easing functions.

use num_traits::{Float, FloatConst};

/// Trait for things that can interpolate values.
///
/// See [this handy cheatsheet](https://easings.net).
pub trait Interpolator<Value>: Float + FloatConst {
    /// Get a value self% between start and end.
    /// `self` being 0 should mean all the way at `start`; `self` being 1 means all the way at `end`.
    /// The implementation must accept values outside of 0 and 1, however,
    /// for easing functions like `elastic_in`.
    ///
    /// All the easing functions are defined in terms of this function.
    /// An equation is run on `self` to get a modified value, and the modified
    /// value has `lerp` called on it.
    fn lerp(self, start: Value, end: Value) -> Value;

    fn sine_in(self, start: Value, end: Value) -> Value {
        let it = Self::one() - ((self * Self::PI()) / Self::from(2).unwrap()).cos();
        it.lerp(start, end)
    }
    fn sine_out(self, start: Value, end: Value) -> Value {
        let it = ((self * Self::PI()) / Self::from(2).unwrap()).sin();
        it.lerp(start, end)
    }
    fn sine_in_out(self, start: Value, end: Value) -> Value {
        let it = -((self * Self::PI()).cos() - Self::one()) / Self::from(2).unwrap();
        it.lerp(start, end)
    }

    fn quad_in(self, start: Value, end: Value) -> Value {
        let it = self * self;
        it.lerp(start, end)
    }
    fn quad_out(self, start: Value, end: Value) -> Value {
        let it = Self::one() - (Self::one() - self) * (Self::one() - self);
        it.lerp(start, end)
    }
    fn quad_in_out(self, start: Value, end: Value) -> Value {
        let two = Self::from(2).unwrap();
        let it = if self < Self::from(0.5).unwrap() {
            two * self * self
        } else {
            Self::one() - (-two * self + two).powi(2) / two
        };
        it.lerp(start, end)
    }

    // impl the rest later
}

impl<F> Interpolator<F> for F
where
    F: Float + FloatConst,
{
    fn lerp(self, start: F, end: F) -> F {
        start * (Self::one() - self) + end * self
    }
}

impl<F, const N: usize> Interpolator<[F; N]> for F
where
    F: Float + FloatConst + Copy,
{
    fn lerp(self, start: [F; N], end: [F; N]) -> [F; N] {
        let mut out = [Self::zero(); N];
        for i in 0..N {
            out[i] = self.lerp(start[i], end[i]);
        }
        out
    }
}
