use std::borrow::{Borrow, BorrowMut};
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};

/// A smart pointer that either owns or mutably borrows.
#[derive(Debug)]
pub enum BorrownedBox<'b, T: ?Sized> {
    /// Contains the owned value
    Owned(Box<T>),
    /// Contains the borrowed value
    Borrowed(&'b mut T),
}

impl<'b, T: ?Sized> BorrownedBox<'b, T> {
    /// Extracts the owned data.
    ///
    /// Returns `self` in `Err` if it's not owned.
    pub fn try_into_box(self) -> Result<Box<T>, Self> {
        match self {
            BorrownedBox::Owned(owned) => Ok(owned),
            _ => Err(self),
        }
    }

    /// Extracts the borrowed data.
    ///
    /// Returns `self` in `Err` if it's not borrowed.
    pub fn try_into_borrowed(self) -> Result<&'b mut T, Self> {
        match self {
            BorrownedBox::Borrowed(borrowed) => Ok(borrowed),
            _ => Err(self),
        }
    }

    fn inner_ref(&self) -> &T {
        match self {
            BorrownedBox::Owned(owned) => owned,
            BorrownedBox::Borrowed(borrowed) => *borrowed,
        }
    }

    fn inner_mut(&mut self) -> &mut T {
        match self {
            BorrownedBox::Owned(owned) => owned,
            BorrownedBox::Borrowed(borrowed) => *borrowed,
        }
    }
}

impl<'b, T: ?Sized> Deref for BorrownedBox<'b, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner_ref()
    }
}

impl<'b, T: ?Sized> DerefMut for BorrownedBox<'b, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner_mut()
    }
}

impl<'b, T: ?Sized> Borrow<T> for BorrownedBox<'b, T> {
    fn borrow(&self) -> &T {
        self.inner_ref()
    }
}

impl<'b, T: ?Sized> BorrowMut<T> for BorrownedBox<'b, T> {
    fn borrow_mut(&mut self) -> &mut T {
        self.inner_mut()
    }
}

impl<'b, T: ?Sized> AsRef<T> for BorrownedBox<'b, T> {
    fn as_ref(&self) -> &T {
        self.inner_ref()
    }
}

impl<'b, T: ?Sized> AsMut<T> for BorrownedBox<'b, T> {
    fn as_mut(&mut self) -> &mut T {
        self.inner_mut()
    }
}

impl<'b, T: Clone + ?Sized> Clone for BorrownedBox<'b, T> {
    fn clone(&self) -> Self {
        match self {
            BorrownedBox::Owned(owned) => BorrownedBox::Owned(owned.clone()),
            BorrownedBox::Borrowed(borrowed) => BorrownedBox::Owned(Box::new((*borrowed).clone())),
        }
    }
}

impl<'b, T: PartialEq + ?Sized> PartialEq for BorrownedBox<'b, T> {
    fn eq(&self, other: &Self) -> bool {
        let b_self = self.inner_ref();
        let b_other = other.inner_ref();

        b_self.eq(b_other)
    }
}

impl<'b, T: Eq + ?Sized> Eq for BorrownedBox<'b, T> {}

impl<'b, T: PartialOrd + ?Sized> PartialOrd for BorrownedBox<'b, T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let b_self = self.inner_ref();
        let b_other = other.inner_ref();

        b_self.partial_cmp(b_other)
    }
}

impl<'b, T: Ord> Ord for BorrownedBox<'b, T> {
    fn cmp(&self, other: &Self) -> Ordering {
        let b_self = self.inner_ref();
        let b_other = other.inner_ref();

        b_self.cmp(&b_other)
    }
}

impl<'b, T: Hash + ?Sized> Hash for BorrownedBox<'b, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let b_self = self.inner_ref();
        b_self.hash(state);
    }
}

impl<'b, T: Default + ?Sized> Default for BorrownedBox<'b, T> {
    fn default() -> Self {
        Self::Owned(Box::default())
    }
}

impl<'b, T: fmt::Display + ?Sized> fmt::Display for BorrownedBox<'b, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.inner_ref(), f)
    }
}

#[cfg(test)]
mod tests {
    use crate::ownership::BorrownedBox;

    #[test]
    fn into_owned_gives_owned_when_owned() {
        let hw = "Hello World".to_string();
        let ob = BorrownedBox::Owned(Box::new(hw.clone()));
        let hw2 = ob.try_into_box();

        assert_eq!(hw2, Ok(Box::new(hw)));
    }

    #[test]
    fn into_owned_gives_self_when_not_owned() {
        let mut hw = "Hello World".to_string();
        let ob = BorrownedBox::Borrowed(&mut hw);
        let hw2 = ob.try_into_box();

        assert!(hw2.is_err());
    }

    #[test]
    fn into_borrowed_gives_borrowed_when_borrowed() {
        let mut hw = "Hello World".to_string();
        let ob = BorrownedBox::Borrowed(&mut hw);
        let hw2 = ob.try_into_borrowed();

        assert!(hw2.is_ok());
    }

    #[test]
    fn into_borrowed_gives_self_when_not_borrowed() {
        let hw = "Hello World".to_string();
        let ob = BorrownedBox::Owned(Box::new(hw));
        let hw2 = ob.try_into_borrowed();

        assert!(hw2.is_err());
    }
}
