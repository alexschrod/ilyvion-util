use core::fmt;
use std::ops::{Index, IndexMut};

/// This struct represents a two-dimensional window into a one-dimensional `Vec`. This is
/// accomplished through taking either a `columns` parameter, and dividing the size of the `Vec`
/// evenly into `rows` based on its length.
pub struct Vec2D<T> {
    raw: Vec<T>,
    rows: usize,
    columns: usize,
}

impl<T> Vec2D<T> {
    /// Creates a new `Vec2D` with rows divided into `columns` length (i.e., `w[rows][columns]`)
    /// placing the result of `func` in each respective entry.
    ///
    /// # Panics
    ///
    /// If `rows * columns > usize::MAX`.
    pub fn new_with<F>(rows: usize, columns: usize, mut func: F) -> Self
    where
        F: FnMut(usize, usize) -> T,
    {
        let raw: Vec<_> = (0..rows
            .checked_mul(columns)
            .expect("rows * columns > usize::MAX"))
            .map(|count| func(count / columns, count % columns))
            .collect();
        Self { raw, rows, columns }
    }

    /// Creates a new `Vec2D` with rows divided into `columns` length. (I.e., `w[rows][columns]`)
    ///
    /// # Panics
    ///
    /// If the length of `raw` cannot be divided evenly into `column`s
    #[allow(clippy::must_use_candidate)]
    pub fn from(raw: Vec<T>, columns: usize) -> Self {
        let rows = raw.len() / columns;
        if raw.len() % columns != 0 {
            panic!("The length of raw must divide evenly into columns.");
        }

        Self { raw, rows, columns }
    }

    /// Creates a new `Vec2D` divided into `rows` number of slices with `columns` entries each.
    ///
    /// Providing incorrect values for `rows` and `columns` will most likely lead to run-time panics
    /// due to indexing outside the range of the `Vec`.
    ///
    /// Using this constructor gives you an essentially zero-cost abstraction.
    #[allow(clippy::must_use_candidate)]
    pub fn from_unchecked(raw: Vec<T>, rows: usize, columns: usize) -> Self {
        Self { raw, rows, columns }
    }
}

impl<T: Default> Vec2D<T> {
    /// Creates a new `Vec2D` with rows divided into `columns` length (i.e., `w[rows][columns]`)
    /// with `T::default()` in every entry.
    ///
    /// # Panics
    ///
    /// If `rows * columns > usize::MAX`.
    #[allow(clippy::must_use_candidate)]
    pub fn new(rows: usize, columns: usize) -> Self {
        Self::new_with(rows, columns, |_, _| T::default())
    }
}

impl<T> Index<usize> for Vec2D<T> {
    type Output = [T];

    fn index(&self, row: usize) -> &Self::Output {
        assert!(row < self.rows);
        &self.raw.as_slice()[row * self.columns..][..self.columns]
    }
}

impl<T> IndexMut<usize> for Vec2D<T> {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        assert!(row < self.rows);
        &mut self.raw.as_mut_slice()[row * self.columns..][..self.columns]
    }
}

impl<T> Index<(usize, usize)> for Vec2D<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self[index.0][index.1]
    }
}

impl<T> IndexMut<(usize, usize)> for Vec2D<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self[index.0][index.1]
    }
}

impl<T: fmt::Debug> fmt::Debug for Vec2D<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Window2D")
            .field("rows", &self.rows)
            .field("columns", &self.columns)
            .field("raw", &self.raw)
            .finish()
    }
}
