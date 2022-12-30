//! Various [`Iterator`] extensions

/// An [`Iterator`] extension trait that provides extra iterator methods.
pub trait IteratorExtensions: Iterator {
    /// Consumes an iterator, creating two collections from it.
    ///
    /// The predicate passed to `partition_map()` can return `true`, or `false`.
    /// For all the elements for which it returned `true`, it calls `left_map` and for all of the
    /// elements for which it returned `false`, it calls `right_map`. The results of these mappings
    /// are then returned as a pair.
    ///
    /// This method was based on/inspired by [`Iterator::partition()`].
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use std::collections::HashMap;
    /// use ilyvion_util::iterator_extensions::IteratorExtensions;
    ///
    /// let a = [1, 2, 3, 4, 5, 6];
    ///
    /// let (even, odd_map): (Vec<i32>, HashMap<i32, f32>) =
    ///     a.iter()
    ///         .partition_map(|&n| n % 2 == 0, |n| n, |&n| (n, (n * 10) as f32));
    ///
    /// assert_eq!(even, vec![2, 4, 6]);
    /// assert_eq!(odd_map.len(), 3);
    /// assert_eq!(odd_map[&1], 10.0);
    /// assert_eq!(odd_map[&3], 30.0);
    /// assert_eq!(odd_map[&5], 50.0);
    /// ```
    fn partition_map<P, L, LT, R, RT, A, B>(
        self,
        mut predicate: P,
        mut left_map: L,
        mut right_map: R,
    ) -> (A, B)
    where
        Self: Sized,
        P: FnMut(&Self::Item) -> bool,
        L: FnMut(Self::Item) -> LT,
        R: FnMut(Self::Item) -> RT,
        A: Default + Extend<LT>,
        B: Default + Extend<RT>,
    {
        let mut left: A = Default::default();
        let mut right: B = Default::default();

        let left_b = &mut left;
        let right_b = &mut right;

        self.fold((), move |(), x| {
            if predicate(&x) {
                left_b.extend(Some(left_map(x)));
            } else {
                right_b.extend(Some(right_map(x)));
            }
        });

        (left, right)
    }
}

impl<T: ?Sized + Iterator> IteratorExtensions for T {}
