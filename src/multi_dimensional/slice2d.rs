use std::ops::{Index, IndexMut};

/// This struct represents a two-dimensional window into a one-dimensional slice. This is
/// accomplished through taking either a `columns` parameter, and dividing the size of the slice
/// evenly into `rows` based on its length, or by taking `rows` and `columns` directly, trusting
/// that the caller provided correct values. The latter option provides a zero-cost abstraction.
///
/// # Examples
/// ```
/// # use ilyvion_util::multi_dimensional::Window2D;
/// let mut values = [0u32; 8];
/// let mut window = Window2D::new_mut(&mut values, 2);
/// window[0][1] = 1;
/// window[1][1] = 2;
/// window[2][1] = 3;
/// window[3][1] = 4;
///
/// assert_eq!(values, [0, 1, 0, 2, 0, 3, 0, 4]);
/// ```
#[derive(Debug)]
pub struct Window2D<T> {
    rows: usize,
    columns: usize,
    raw: T,
}

impl<'b, T> Window2D<&'b mut [T]> {
    /// Creates a new `Window2D` with rows divided into `columns` length. (I.e., `w[rows][columns]`)
    ///
    /// # Panics
    ///
    /// If the length of `raw` cannot be divided evenly into `column`s
    pub fn new_mut(raw: &'b mut [T], columns: usize) -> Self {
        let rows = raw.len() / columns;
        if raw.len() % columns != 0 {
            panic!("The length of raw must divide evenly into columns.");
        }

        Self { raw, rows, columns }
    }

    /// Creates a new `Window2D` divided into `rows` number of slices with `columns` entries each.
    ///
    /// Providing incorrect values for `rows` and `columns` will most likely lead to run-time panics
    /// due to indexing outside the range of the slice.
    ///
    /// Using this constructor gives you an essentially zero-cost abstraction.  
    pub fn new_mut_unchecked(raw: &'b mut [T], rows: usize, columns: usize) -> Self {
        Self { raw, rows, columns }
    }
}

impl<'b, T> Window2D<&'b [T]> {
    /// Creates a new `Window2D` with rows divided into `columns` length. (I.e., `w[rows][columns]`)
    ///
    /// # Panics
    ///
    /// If the length of `raw` cannot be divided evenly into `column`s
    pub fn new_ref(raw: &'b [T], columns: usize) -> Self {
        let rows = raw.len() / columns;
        if raw.len() % columns != 0 {
            panic!("The length of raw must divide evenly into columns.");
        }

        Self { raw, rows, columns }
    }

    /// Creates a new `Window2D` divided into `rows` number of slices with `columns` entries each.
    ///
    /// Providing incorrect values for `rows` and `columns` will most likely lead to run-time panics
    /// due to indexing outside the range of the slice.
    ///
    /// Using this constructor gives you an essentially zero-cost abstraction.
    pub fn new_ref_unchecked(raw: &'b [T], rows: usize, columns: usize) -> Self {
        Self { raw, rows, columns }
    }
}

impl<T> Index<usize> for Window2D<&'_ [T]> {
    type Output = [T];

    fn index(&self, row: usize) -> &Self::Output {
        assert!(row < self.rows);
        &self.raw[row * self.columns..][..self.columns]
    }
}

impl<T> Index<usize> for Window2D<&'_ mut [T]> {
    type Output = [T];

    fn index(&self, row: usize) -> &Self::Output {
        assert!(row < self.rows);
        &self.raw[row * self.columns..][..self.columns]
    }
}

impl<T> IndexMut<usize> for Window2D<&'_ mut [T]> {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        assert!(row < self.rows);
        &mut self.raw[row * self.columns..][..self.columns]
    }
}
