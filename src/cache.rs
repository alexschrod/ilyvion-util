//! Cache implementations based on the Rust book exercises.

use std::borrow::Borrow;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use std::hash::Hash;

/// Caches the result of an (presumably) expensive operation
/// such that accessing the result multiple times doesn't result in
/// running the expensive operation multiple times.
#[derive(Debug)]
pub struct Cache<F, V>
where
    F: FnOnce() -> V,
{
    calculation_fn: Option<F>,
    value: Option<V>,
}

impl<F, V> Cache<F, V>
where
    F: FnOnce() -> V,
{
    /// Creates a new `Cache<F, V>` initialized with the given `calculation_fn` function.
    /// The function  will not be called until the result of the calculation is needed.
    pub fn new(calculation_fn: F) -> Self {
        Self {
            calculation_fn: Some(calculation_fn),
            value: None,
        }
    }

    /// Gets a mutable reference to the contained calculated value.
    /// Runs the calculation function if this method call is the first time the value is accessed.
    pub fn value_mut(&mut self) -> &mut V {
        let calculation_fn = &mut self.calculation_fn;
        self.value
            .get_or_insert_with(|| (calculation_fn.take().unwrap())())
    }

    /// Gets a shared reference to the contained calculated value if it has
    /// already been calculated.
    pub fn value(&self) -> Option<&V> {
        self.value.as_ref()
    }
}

/// Caches the result of an (presumably) expensive operation
/// such that accessing the result multiple times doesn't result in
/// running the expensive operation multiple times.
#[derive(Debug)]
pub struct KeyedCache<F, K, V>
where
    F: FnMut(&K) -> V,
    K: Hash + Eq,
{
    calculation_fn: F,
    values: HashMap<K, V>,
}

impl<F, K, V> KeyedCache<F, K, V>
where
    F: FnMut(&K) -> V,
    K: Hash + Eq,
{
    /// Creates a new `KeyedCache<F, V>` initialized with the given `calculation_fn` function.
    /// The function  will not be called until the result of a calculation is needed.
    pub fn new(calculation_fn: F) -> Self {
        Self {
            calculation_fn,
            values: HashMap::new(),
        }
    }

    /// Gets a mutable reference to a contained calculated value based on the `key`.
    /// Runs the calculation function if this method call is the first time the value
    /// with the given `key` is accessed.
    pub fn value_mut(&mut self, key: K) -> &mut V {
        match self.values.entry(key) {
            Vacant(e) => {
                let calculation = (self.calculation_fn)(e.key());
                e.insert(calculation)
            }
            Occupied(e) => e.into_mut(),
        }
    }

    /// Gets a shared reference to the contained calculated value based on the `key`
    /// if it has already been calculated.
    pub fn value<Q: ?Sized>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.values.get(key)
    }
}

#[cfg(test)]
mod tests {
    use crate::cache::{Cache, KeyedCache};

    #[test]
    fn cache_value_is_only_calculated_once() {
        let mut counter = 0;
        let mut sut = Cache::new(|| {
            counter += 1;
        });

        sut.value_mut();
        sut.value_mut();

        assert_eq!(counter, 1);
    }

    #[test]
    fn cache_value_is_correct() {
        let mut sut = Cache::new(|| 42);
        assert_eq!(42, *sut.value_mut());
    }

    #[test]
    fn cache_value_is_not_available_before_access() {
        let sut = Cache::new(|| 42);

        assert_eq!(None, sut.value());
    }

    #[test]
    fn cache_value_is_available_after_first_access() {
        let mut sut = Cache::new(|| 42);

        sut.value_mut();

        assert_eq!(Some(&42), sut.value());
    }

    #[test]
    fn cache_mut() {
        let mut sut = Cache::new(|| 42);
        let val = sut.value_mut();
        assert_eq!(&mut 42, val);

        *val = 5;

        assert_eq!(5, *sut.value_mut());
    }

    #[test]
    fn keyed_cache_value_is_only_calculated_once() {
        let mut counter = 0;
        let mut sut = KeyedCache::new(|x| {
            counter += 1;
            x + 5
        });

        sut.value_mut(5);
        sut.value_mut(5);
        sut.value_mut(10);
        sut.value_mut(10);

        assert_eq!(counter, 2);
    }

    #[test]
    fn keyed_cache_value_is_correct() {
        let mut sut = KeyedCache::new(|w: &String| {
            let mut result = String::from("Hello ");
            result.push_str(w);
            result
        });
        assert_eq!("Hello World", sut.value_mut(String::from("World")));
    }

    #[test]
    fn keyed_cache_value_is_not_available_before_access() {
        let sut = KeyedCache::new(|_: &i32| 42);

        assert_eq!(None, sut.value(&69));
    }

    #[test]
    fn keyed_cache_value_is_available_after_first_access() {
        let mut sut = KeyedCache::new(|k: &i32| *k + 42);

        sut.value_mut(69);

        assert_eq!(Some(&111), sut.value(&69));
    }
}
