//! A trait and a type useful for dealing with types with a `NaN` value.
//! This is `f32` and `f64`.

use shrinkwraprs::Shrinkwrap;
use std::cmp::Ordering;
use std::fmt::Debug;

/// Trait that lets you generalize over types that have a NaN.
pub trait NanType: Copy + Clone + Default + Debug + PartialOrd + PartialEq {
    /// Returns `true` if this value is `NaN`.
    fn is_nan(self) -> bool;
}
impl NanType for f32 {
    fn is_nan(self) -> bool {
        self.is_nan()
    }
}
impl NanType for f64 {
    fn is_nan(self) -> bool {
        self.is_nan()
    }
}

/// A type that wraps a `NanType` with the guarantee that its contained value is not
/// `NaN`.
#[derive(PartialEq, PartialOrd, Shrinkwrap, Copy, Clone, Default, Debug)]
pub struct NonNan<T: NanType>(T);

impl<T: NanType> NonNan<T> {
    /// Creates a new `NonNan<T>`.
    ///
    /// # Panics
    ///
    /// If `val.is_nan()` is `true`.
    pub fn new(val: T) -> Self {
        assert!(!val.is_nan(), "NaN values are not allowed");
        Self(val)
    }
}

impl<T: NanType> Eq for NonNan<T> {}

impl<T: NanType> Ord for NonNan<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<T: NanType> From<T> for NonNan<T> {
    fn from(t: T) -> Self {
        Self::new(t)
    }
}
