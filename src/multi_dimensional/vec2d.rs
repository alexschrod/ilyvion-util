use std::ops::{Index, IndexMut};

/// This struct represents a two-dimensional window into a one-dimensional `Vec`. This is
/// accomplished through taking either a `columns` parameter, and dividing the size of the `Vec`
/// evenly into `rows` based on its length, or by taking `rows` and `columns` directly, trusting
/// that the caller provided correct values. The latter option provides a zero-cost abstraction.
///
/// # Example
/// ```
/// # use ilyvion_util::multi_dimensional::Vec2D;
/// let mut window = Vec2D::from(vec![0u32; 8], 2);
/// window[0][1] = 1;
/// window[1][1] = 2;
/// window[2][1] = 3;
/// window[3][1] = 4;
///
/// let values = window.into_inner();
///
/// assert_eq!(values, [0, 1, 0, 2, 0, 3, 0, 4]);
/// ```
#[derive(Debug)]
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
    ///
    /// # Examples
    /// ```
    /// # use ilyvion_util::multi_dimensional::Vec2D;
    /// let v = Vec2D::new_with(2, 2, |y, x| 100 * y + 10 * x);
    /// let values = v.into_inner();
    ///
    /// assert_eq!(values, [0, 10, 100, 110]);
    /// ```
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
    /// due to indexing outside the range of the [`Vec`].
    ///
    /// Using this constructor gives you an essentially zero-cost abstraction.
    pub fn from_unchecked(raw: Vec<T>, rows: usize, columns: usize) -> Self {
        Self { raw, rows, columns }
    }

    /// Unwraps this `Vec2D<T>`, returning the underlying [`Vec`].
    pub fn into_inner(self) -> Vec<T> {
        self.raw
    }
}

impl<T: Default> Vec2D<T> {
    /// Creates a new `Vec2D` with rows divided into `columns` length (i.e., `w[rows][columns]`)
    /// with `T::default()` in every entry.
    ///
    /// # Panics
    ///
    /// If `rows * columns > usize::MAX`.
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
