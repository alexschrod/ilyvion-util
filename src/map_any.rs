//! Adds a `map()` method to any type, allowing for inline conversion from one
//! type to another.

/// Adds a `map()` method to any type, allowing for inline conversion from one
/// type to another.
pub trait MapAny: Sized {
    /// Takes a closure and calls it with `Self`, then returns whatever the
    /// closure returned.
    fn map<T, F: Fn(Self) -> T>(self, map_fn: F) -> T;
}

impl<T> MapAny for T {
    fn map<U, F: Fn(Self) -> U>(self, map_fn: F) -> U {
        map_fn(self)
    }
}
