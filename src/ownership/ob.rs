use std::borrow::{Borrow, BorrowMut};
use std::ops::{Deref, DerefMut};

/// A smart pointer that either owns or mutably borrows.
pub enum Ob<'b, T> {
    /// Contains the owned value
    Owned(T),
    /// Contains the borrowed value
    Borrowed(&'b mut T),
}

impl<'b, T> Ob<'b, T> {
    /// Extracts the owned data.
    ///
    /// Returns `self` in `Err` if it's not owned.
    pub fn into_owned(self) -> Result<T, Self> {
        match self {
            Ob::Owned(owned) => Ok(owned),
            _ => Err(self),
        }
    }

    /// Extracts the borrowed data.
    ///
    /// Returns `self` in `Err` if it's not borrowed.
    pub fn into_borrowed(self) -> Result<&'b mut T, Self> {
        match self {
            Ob::Borrowed(borrowed) => Ok(borrowed),
            _ => Err(self),
        }
    }

    fn inner_ref(&self) -> &T {
        match self {
            Ob::Owned(owned) => owned,
            Ob::Borrowed(borrowed) => *borrowed,
        }
    }

    fn inner_mut(&mut self) -> &mut T {
        match self {
            Ob::Owned(owned) => owned,
            Ob::Borrowed(borrowed) => *borrowed,
        }
    }
}

impl<'b, T> Deref for Ob<'b, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner_ref()
    }
}

impl<'b, T> DerefMut for Ob<'b, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner_mut()
    }
}

impl<'b, T> Borrow<T> for Ob<'b, T> {
    fn borrow(&self) -> &T {
        self.inner_ref()
    }
}

impl<'b, T> BorrowMut<T> for Ob<'b, T> {
    fn borrow_mut(&mut self) -> &mut T {
        self.inner_mut()
    }
}

impl<'b, T> AsRef<T> for Ob<'b, T> {
    fn as_ref(&self) -> &T {
        self.inner_ref()
    }
}

impl<'b, T> AsMut<T> for Ob<'b, T> {
    fn as_mut(&mut self) -> &mut T {
        self.inner_mut()
    }
}
