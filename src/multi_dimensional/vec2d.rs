use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct Vec2D<T> {
    raw: Vec<T>,
    rows: usize,
    columns: usize,
}

impl<T: Default + Clone> Vec2D<T> {
    pub fn new(rows: usize, columns: usize) -> Self {
        Self {
            raw: vec![T::default(); rows * columns],
            rows,
            columns,
        }
    }
}

impl<T: Clone> Vec2D<T> {
    pub fn new_with<F>(rows: usize, columns: usize, mut func: F) -> Self
    where
        F: FnMut(usize, usize) -> T,
    {
        let raw: Vec<_> = (0..rows.checked_mul(columns).expect("rows * colums > usize"))
            .map(|count| func(count / columns, count % columns))
            .collect();
        Self { raw, rows, columns }
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
