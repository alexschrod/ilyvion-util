use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use std::hash::Hash;

pub struct Cache<F, V>
    where F: FnOnce() -> V
{
    calculation_fn: Option<F>,
    value: Option<V>,
}

impl<F, V> Cache<F, V>
    where F: FnOnce() -> V
{
    pub fn new(calculation: F) -> Cache<F, V> {
        Cache {
            calculation_fn: Some(calculation),
            value: None,
        }
    }

    pub fn value_mut(&mut self) -> &mut V {
        let calculation_fn = &mut self.calculation_fn;
        self.value
            .get_or_insert_with(|| (calculation_fn.take().unwrap())())
    }

    pub fn value(&mut self) -> &V {
        self.value_mut()
    }
}

impl<F, V> AsMut<V> for Cache<F, V>
    where F: FnOnce() -> V
{
    fn as_mut(&mut self) -> &mut V {
        self.value_mut()
    }
}

pub struct KeyedCache<F, K, V>
    where F: FnMut(&K) -> V, K: Hash + Eq
{
    calculation_fn: F,
    values: HashMap<K, V>,
}

impl<F, K, V> KeyedCache<F, K, V>
    where F: FnMut(&K) -> V, K: Hash + Eq
{
    pub fn new(calculation_fn: F) -> KeyedCache<F, K, V> {
        KeyedCache {
            calculation_fn,
            values: HashMap::new(),
        }
    }

    pub fn value_mut(&mut self, key: K) -> &mut V {
        match self.values.entry(key) {
            Vacant(e) => {
                let calculation = (self.calculation_fn)(e.key());
                e.insert(calculation)
            }
            Occupied(e) => {
                e.into_mut()
            }
        }
    }

    pub fn value(&mut self, key: K) -> &V {
        self.value_mut(key)
    }
}

#[cfg(test)]
mod tests {
    use crate::cache::{Cache, KeyedCache};

    #[test]
    fn cache_value_is_only_calculated_once()
    {
        let mut counter = 0;
        let mut sut = Cache::new(|| {
            counter += 1;
            3
        });

        sut.value();
        sut.value();

        assert_eq!(counter, 1);
    }

    #[test]
    fn cache_value_is_correct() {
        let mut sut = Cache::new(|| 42);
        assert_eq!(42, *sut.value());
    }

    #[test]
    fn cache_as_mut() {
        let mut sut = Cache::new(|| 42);
        let val = sut.as_mut();
        assert_eq!(&mut 42, val);

        *val = 5;

        assert_eq!(5, *sut.value_mut());
    }

    #[test]
    fn keyed_cache_value_is_only_calculated_once()
    {
        let mut counter = 0;
        let mut sut = KeyedCache::new(|x| {
            counter += 1;
            x + 5
        });

        sut.value(5);
        sut.value(5);
        sut.value(10);
        sut.value(10);

        assert_eq!(counter, 2);
    }

    #[test]
    fn keyed_cache_value_is_correct() {
        let mut sut = KeyedCache::new(|w: &String| {
            let mut result = String::from("Hello ");
            result.push_str(w);
            result
        });
        assert_eq!("Hello World", sut.value(String::from("World")));
    }
}
