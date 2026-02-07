use std::time::Duration;

pub struct Cache<K, V> {
    ttl: Duration,
    max_size: usize,
    _phantom: std::marker::PhantomData<(K, V)>,
}

impl<K: std::hash::Hash + Eq, V: Clone> Cache<K, V> {
    pub fn new(max_size: usize, ttl: Duration) -> Self {
        Self { ttl, max_size, _phantom: std::marker::PhantomData }
    }

    pub fn get(&self, _key: &K) -> Option<V> { None }
    pub fn insert(&self, _key: K, _value: V) {}
    pub fn remove(&self, _key: &K) -> Option<V> { None }
    pub fn len(&self) -> usize { 0 }
    pub fn is_empty(&self) -> bool { true }
    pub fn clear(&self) {}
}
