use std::borrow::{Borrow, BorrowMut};
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};

/// A smart pointer that either owns or mutably borrows a value.
///
/// # Example
///
/// ```
/// use borrowned::Borrowned;
///
/// fn print_text(text: &Borrowned<'_, String>) {
///     println!("{}", text);
/// }
///
/// let owned = Borrowned::Owned("hello".to_string());
/// let mut owned2 = "world".to_string();
/// let borrowed = Borrowned::Borrowed(&mut owned2);
///
/// print_text(&owned);
/// print_text(&borrowed);
/// print_text(&owned2.into());
///
/// ```
#[derive(Debug)]
pub enum Borrowned<'b, T> {
    /// Contains the owned value
    Owned(T),
    /// Contains the borrowed value
    Borrowed(&'b mut T),
}

impl<'b, T> Borrowned<'b, T> {
    /// Extracts the owned data.
    ///
    /// Returns `self` in `Err` if it's not owned.
    pub fn try_into_owned(self) -> Result<T, Self> {
        match self {
            Borrowned::Owned(owned) => Ok(owned),
            _ => Err(self),
        }
    }

    /// Extracts the borrowed data.
    ///
    /// Returns `self` in `Err` if it's not borrowed.
    pub fn try_into_borrowed(self) -> Result<&'b mut T, Self> {
        match self {
            Borrowned::Borrowed(borrowed) => Ok(borrowed),
            _ => Err(self),
        }
    }

    fn inner_ref(&self) -> &T {
        match self {
            Borrowned::Owned(owned) => owned,
            Borrowned::Borrowed(borrowed) => borrowed,
        }
    }

    fn inner_mut(&mut self) -> &mut T {
        match self {
            Borrowned::Owned(owned) => owned,
            Borrowned::Borrowed(borrowed) => borrowed,
        }
    }
}

impl<'b, T> Deref for Borrowned<'b, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner_ref()
    }
}

impl<'b, T> DerefMut for Borrowned<'b, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner_mut()
    }
}

impl<'b, T> Borrow<T> for Borrowned<'b, T> {
    fn borrow(&self) -> &T {
        self.inner_ref()
    }
}

impl<'b, T> BorrowMut<T> for Borrowned<'b, T> {
    fn borrow_mut(&mut self) -> &mut T {
        self.inner_mut()
    }
}

impl<'b, T> AsRef<T> for Borrowned<'b, T> {
    fn as_ref(&self) -> &T {
        self.inner_ref()
    }
}

impl<'b, T> AsMut<T> for Borrowned<'b, T> {
    fn as_mut(&mut self) -> &mut T {
        self.inner_mut()
    }
}

impl<'b, T: Clone> Clone for Borrowned<'b, T> {
    fn clone(&self) -> Self {
        match self {
            Borrowned::Owned(owned) => Borrowned::Owned(owned.clone()),
            Borrowned::Borrowed(borrowed) => Borrowned::Owned((*borrowed).clone()),
        }
    }
}

impl<'b, T: PartialEq> PartialEq for Borrowned<'b, T> {
    fn eq(&self, other: &Self) -> bool {
        let b_self = self.inner_ref();
        let b_other = other.inner_ref();

        b_self.eq(b_other)
    }
}

impl<'b, T: Eq> Eq for Borrowned<'b, T> {}

impl<'b, T: PartialOrd> PartialOrd for Borrowned<'b, T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let b_self = self.inner_ref();
        let b_other = other.inner_ref();

        b_self.partial_cmp(b_other)
    }
}

impl<'b, T: Ord> Ord for Borrowned<'b, T> {
    fn cmp(&self, other: &Self) -> Ordering {
        let b_self = self.inner_ref();
        let b_other = other.inner_ref();

        b_self.cmp(b_other)
    }
}

impl<'b, T: Hash> Hash for Borrowned<'b, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let b_self = self.inner_ref();
        b_self.hash(state);
    }
}

impl<'b, T: Default> Default for Borrowned<'b, T> {
    fn default() -> Self {
        Self::Owned(T::default())
    }
}

impl<'b, T: fmt::Display> fmt::Display for Borrowned<'b, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.inner_ref(), f)
    }
}

impl<'b, T> From<T> for Borrowned<'b, T> {
    fn from(owned: T) -> Self {
        Self::Owned(owned)
    }
}

impl<'b, T> From<&'b mut T> for Borrowned<'b, T> {
    fn from(borrowed: &'b mut T) -> Self {
        Self::Borrowed(borrowed)
    }
}

#[cfg(test)]
mod tests {
    use crate::Borrowned;

    #[test]
    fn into_owned_gives_owned_when_owned() {
        let hw = "Hello World".to_string();
        let ob = Borrowned::Owned(hw.clone());
        let hw2 = ob.try_into_owned();

        assert_eq!(hw2, Ok(hw));
    }

    #[test]
    fn into_owned_gives_self_when_not_owned() {
        let mut hw = "Hello World".to_string();
        let ob = Borrowned::Borrowed(&mut hw);
        let hw2 = ob.try_into_owned();

        assert!(hw2.is_err());
    }

    #[test]
    fn into_borrowed_gives_borrowed_when_borrowed() {
        let mut hw = "Hello World".to_string();
        let ob = Borrowned::Borrowed(&mut hw);
        let hw2 = ob.try_into_borrowed();

        assert!(hw2.is_ok());
    }

    #[test]
    fn into_borrowed_gives_self_when_not_borrowed() {
        let hw = "Hello World".to_string();
        let ob = Borrowned::Owned(hw);
        let hw2 = ob.try_into_borrowed();

        assert!(hw2.is_err());
    }
}
