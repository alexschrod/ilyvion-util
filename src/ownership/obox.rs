use std::borrow::{Borrow, BorrowMut};
use std::ops::{Deref, DerefMut};

/// A smart pointer that either owns or mutably borrows.
pub enum OBox<'b, T: ?Sized> {
    /// Contains the owned value
    Owned(Box<T>),
    /// Contains the borrowed value
    Borrowed(&'b mut T),
}

impl<'b, T: ?Sized> OBox<'b, T> {
    /// Extracts the owned data.
    ///
    /// Returns `self` in `Err` if it's not owned.
    pub fn into_box(self) -> Result<Box<T>, Self> {
        match self {
            OBox::Owned(owned) => Ok(owned),
            _ => Err(self),
        }
    }

    /// Extracts the borrowed data.
    ///
    /// Returns `self` in `Err` if it's not borrowed.
    pub fn into_borrowed(self) -> Result<&'b mut T, Self> {
        match self {
            OBox::Borrowed(borrowed) => Ok(borrowed),
            _ => Err(self),
        }
    }

    fn inner_ref(&self) -> &T {
        match self {
            OBox::Owned(owned) => owned,
            OBox::Borrowed(borrowed) => *borrowed,
        }
    }

    fn inner_mut(&mut self) -> &mut T {
        match self {
            OBox::Owned(owned) => owned,
            OBox::Borrowed(borrowed) => *borrowed,
        }
    }
}

impl<'b, T: ?Sized> Deref for OBox<'b, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner_ref()
    }
}

impl<'b, T: ?Sized> DerefMut for OBox<'b, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner_mut()
    }
}

impl<'b, T: ?Sized> Borrow<T> for OBox<'b, T> {
    fn borrow(&self) -> &T {
        self.inner_ref()
    }
}

impl<'b, T: ?Sized> BorrowMut<T> for OBox<'b, T> {
    fn borrow_mut(&mut self) -> &mut T {
        self.inner_mut()
    }
}

impl<'b, T: ?Sized> AsRef<T> for OBox<'b, T> {
    fn as_ref(&self) -> &T {
        self.inner_ref()
    }
}

impl<'b, T: ?Sized> AsMut<T> for OBox<'b, T> {
    fn as_mut(&mut self) -> &mut T {
        self.inner_mut()
    }
}
