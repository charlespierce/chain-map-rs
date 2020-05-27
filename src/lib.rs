use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::{BuildHasher, Hash};
use std::iter::FromIterator;

pub trait Map {
    type Key: Hash + Eq;
    type Value;

    fn has_key<Q>(&self, k: &Q) -> bool
    where
        Self::Key: Borrow<Q>,
        Q: Hash + Eq + ?Sized;

    fn get_value<Q>(&self, k: &Q) -> Option<&Self::Value>
    where
        Self::Key: Borrow<Q>,
        Q: Hash + Eq + ?Sized;
}

pub struct ChainMap<M> {
    inner: Vec<M>,
}

impl<M: Map> ChainMap<M> {
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
        self.inner.iter().any(|map| map.has_key(k))
    }

    pub fn get<Q>(&self, k: &Q) -> Option<&M::Value>
    where
        M::Key: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.inner.iter().find_map(|map| map.get_value(k))
    }

    pub fn push_map(&mut self, map: M) {
        self.inner.push(map);
    }
}

impl<M: Map> Default for ChainMap<M> {
    fn default() -> Self {
        ChainMap { inner: Vec::new() }
    }
}

impl<M: Map> From<M> for ChainMap<M> {
    fn from(map: M) -> Self {
        ChainMap { inner: vec![map] }
    }
}

impl<M: Map> FromIterator<M> for ChainMap<M> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = M>,
    {
        ChainMap {
            inner: Vec::from_iter(iter),
        }
    }
}

impl<K, V, S> Map for HashMap<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    type Key = K;
    type Value = V;

    fn has_key<Q>(&self, k: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.contains_key(k)
    }

    fn get_value<Q>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.get(k)
    }
}

#[cfg(feature = "index-map")]
impl<K, V, S> Map for indexmap::IndexMap<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    type Key = K;
    type Value = V;

    fn has_key<Q>(&self, k: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.contains_key(k)
    }

    fn get_value<Q>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.get(k)
    }
}
