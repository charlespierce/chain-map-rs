use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::{BuildHasher, Hash};
use std::iter::FromIterator;

pub trait ReadOnlyMap {
    type Key: Hash + Eq;
    type Value;

    fn contains_key<Q>(&self, k: &Q) -> bool
    where
        Self::Key: Borrow<Q>,
        Q: Hash + Eq + ?Sized;

    fn get<Q>(&self, k: &Q) -> Option<&Self::Value>
    where
        Self::Key: Borrow<Q>,
        Q: Hash + Eq + ?Sized;
}

#[derive(Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct ChainMap<M> {
    inner: Vec<M>,
}

impl<M: ReadOnlyMap> ChainMap<M> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        ChainMap {
            inner: Vec::with_capacity(capacity),
        }
    }

    pub fn contains_key<Q>(&self, k: &Q) -> bool
    where
        M::Key: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.inner.iter().any(|map| map.contains_key(k))
    }

    pub fn get<Q>(&self, k: &Q) -> Option<&M::Value>
    where
        M::Key: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.inner.iter().find_map(|map| map.get(k))
    }

    pub fn push_map(&mut self, map: M) {
        self.inner.push(map)
    }

    pub fn insert_map(&mut self, index: usize, element: M) {
        self.inner.insert(index, element)
    }
}

impl<M> Default for ChainMap<M> {
    fn default() -> Self {
        ChainMap { inner: Vec::new() }
    }
}

impl<M> FromIterator<M> for ChainMap<M> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = M>,
    {
        ChainMap {
            inner: Vec::from_iter(iter),
        }
    }
}

impl<M> Extend<M> for ChainMap<M> {
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = M>,
    {
        self.inner.extend(iter);
    }
}

impl<K, V, S> ReadOnlyMap for HashMap<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    type Key = K;
    type Value = V;

    fn contains_key<Q>(&self, k: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.contains_key(k)
    }

    fn get<Q>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.get(k)
    }
}
