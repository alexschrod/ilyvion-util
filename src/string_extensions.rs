//! Various [`String`] and [`str`] extensions

use std::borrow::Cow;

/// The trait responsible for adding methods to [`str`].
pub trait StrExtensions {
    /// Works like [`str::to_ascii_lowercase`], but returns a [`Cow::Borrowed`] when the original
    /// already is ASCII and lowercase.
    fn to_ascii_lowercase_cow(&self) -> Cow<'_, str>;

    /// Returns whether or not this [`str`] is already ASCII lowercase.
    fn is_ascii_lowercase(&self) -> bool;
}

impl StrExtensions for str {
    fn to_ascii_lowercase_cow(&self) -> Cow<'_, str> {
        if self.is_ascii_lowercase() {
            Cow::Borrowed(self)
        } else {
            Cow::Owned(self.to_ascii_lowercase())
        }
    }

    fn is_ascii_lowercase(&self) -> bool {
        let bytes = self.as_bytes();
        bytes.is_ascii()
            && bytes
                .iter()
                .all(|b| !b.is_ascii_alphabetic() || b.is_ascii_lowercase())
    }
}
