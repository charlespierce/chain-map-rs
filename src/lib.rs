use std::borrow::Borrow;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::hash::{BuildHasher, Hash};
use std::iter::FromIterator;
use std::ops::Index;

#[derive(Clone, Debug)]
pub struct ChainMap<K, V, S = RandomState> {
    inner: Vec<HashMap<K, V, S>>,
}

impl<K, V, S> ChainMap<K, V, S> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        ChainMap {
            inner: Vec::with_capacity(capacity),
        }
    }

    pub fn push_map(&mut self, map: HashMap<K, V, S>) {
        self.inner.push(map)
    }
}

impl<K, V, S> ChainMap<K, V, S>
where
    K: Hash + Eq,
    S: BuildHasher,
{
    pub fn contains_key<Q>(&self, k: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.inner.iter().any(|map| map.contains_key(k))
    }

    pub fn get<Q>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.inner.iter().find_map(|map| map.get(k))
    }
}

impl<K, V, S> Default for ChainMap<K, V, S> {
    fn default() -> Self {
        ChainMap { inner: Vec::new() }
    }
}

impl<K, Q, V, S> Index<&Q> for ChainMap<K, V, S>
where
    K: Eq + Hash + Borrow<Q>,
    Q: Eq + Hash + ?Sized,
    S: BuildHasher,
{
    type Output = V;

    fn index(&self, k: &Q) -> &V {
        self.get(k).expect("no entry found for key")
    }
}

impl<K, V, S> FromIterator<HashMap<K, V, S>> for ChainMap<K, V, S> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = HashMap<K, V, S>>,
    {
        ChainMap {
            inner: Vec::from_iter(iter),
        }
    }
}

impl<K, V, S> Extend<HashMap<K, V, S>> for ChainMap<K, V, S> {
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = HashMap<K, V, S>>,
    {
        self.inner.extend(iter)
    }
}

impl<K, V, S> PartialEq for ChainMap<K, V, S>
where
    K: Eq + Hash,
    V: PartialEq,
    S: BuildHasher,
{
    fn eq(&self, other: &ChainMap<K, V, S>) -> bool {
        self.inner.eq(&other.inner)
    }
}

impl<K, V, S> Eq for ChainMap<K, V, S>
where
    K: Eq + Hash,
    V: Eq,
    S: BuildHasher,
{
}
