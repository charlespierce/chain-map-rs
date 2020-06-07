//! The [`ChainMap`] type groups a chain of [`HashMap`]s together in precedence
//! order and provides a single, unified view into the values. The semantics
//! for keys are the same as for a [`HashMap`], however the value associated
//! with a given key is the value of that key in the highest-precedence map
//! that contains the key.
//!
//! # Precedence
//!
//! Maps added to the [`ChainMap`] earlier have precedence over those added
//! later. So the first map added to the chain will have the highest
//! precedence, while the most recent map added will have the lowest.
//!
//! # Performance
//!
//! Each read of the [`ChainMap`] will read the chain of maps in order, so each
//! operation will complete in worst-case O(N), with `N` the number of maps in
//! the chain. As a result, this should only be used for cases where the number
//! of reads is low compared to the number of elements in each map.
//!
//! # Examples
//!
//! ```
//! use std::collections::HashMap;
//! use chain_map::ChainMap;
//!
//! let mut first_map = HashMap::new();
//! first_map.insert("first", 10);
//!
//! let mut second_map = HashMap::new();
//! second_map.insert("first", 20);
//! second_map.insert("second", 20);
//!
//! let mut third_map = HashMap::new();
//! third_map.insert("first", 30);
//! third_map.insert("second", 30);
//! third_map.insert("third", 30);
//!
//! let mut chain: ChainMap<_, _> =
//!     vec![first_map, second_map, third_map].into_iter().collect();
//! assert_eq!(chain.get("first"), Some(&10));
//! assert_eq!(chain["second"], 20);
//! assert!(chain.contains_key("third"));
//! assert!(!chain.contains_key("fourth"));
//! ```
//!
//! [`ChainMap`]: struct.ChainMap.html
//! [`HashMap`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html

use std::borrow::Borrow;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::hash::{BuildHasher, Hash};
use std::iter::FromIterator;
use std::ops::Index;

#[derive(Clone, Debug)]
/// The `ChainMap` type. See [the module level documentation](index.html) for
/// more.
pub struct ChainMap<K, V, S = RandomState> {
    inner: Vec<HashMap<K, V, S>>,
}

impl<K, V, S> ChainMap<K, V, S> {
    /// Creates an empty `ChainMap`.
    ///
    /// The chain is initially created with a capacity of 0, so it will not
    /// allocated until a [`HashMap`] is inserted into the chain.
    ///
    /// # Examples
    ///
    /// ```
    /// use chain_map::ChainMap;
    /// let mut chain: ChainMap<&str, i32> = ChainMap::new();
    /// ```
    ///
    /// [`HashMap`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a `ChainMap` with the specified capacity.
    ///
    /// Will be able to hold at least `capacity` [`HashMap`]s without
    /// reallocating. If `capacity` is 0, the chain will not allocate.
    ///
    /// # Examples
    ///
    /// ```
    /// use chain_map::ChainMap;
    /// let mut chain: ChainMap<&str, i32> = ChainMap::with_capacity(10);
    /// ```
    ///
    /// [`HashMap`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
    pub fn with_capacity(capacity: usize) -> Self {
        ChainMap {
            inner: Vec::with_capacity(capacity),
        }
    }

    /// Appends a map to the lowest-precedence end of the chain
    ///
    /// # Panics
    ///
    /// Panics if the number of maps in the chain overflows a [`usize`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use chain_map::ChainMap;
    ///
    /// let mut hash = HashMap::new();
    /// hash.insert("key", "value");
    ///
    /// let mut chain = ChainMap::new();
    /// chain.push_map(hash);
    /// ```
    ///
    /// [`usize`]: https://doc.rust-lang.org/std/primitive.usize.html
    pub fn push_map(&mut self, map: HashMap<K, V, S>) {
        self.inner.push(map)
    }
}

impl<K, V, S> ChainMap<K, V, S>
where
    K: Hash + Eq,
    S: BuildHasher,
{
    /// Returns `true` if the `ChainMap` contains a value for the given key.
    ///
    /// As with [`HashMap::contains_key`], the supplied key may be any borrowed
    /// form of the key type, but `Hash` and `Eq` on the borrowed form _must_
    /// match those for the key type.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use chain_map::ChainMap;
    ///
    /// let mut hash = HashMap::new();
    /// hash.insert("key", "value");
    ///
    /// let mut chain = ChainMap::new();
    /// chain.push_map(hash);
    /// assert!(chain.contains_key("key"));
    /// ```
    ///
    /// [`HashMap::contains_key`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.contains_key
    pub fn contains_key<Q>(&self, k: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.inner.iter().any(|map| map.contains_key(k))
    }

    /// Returns the highest-precedence value associated with the given key.
    ///
    /// As with [`HashMap::get`], the supplied key may be any borrowed form of
    /// the key type, but `Hash` and `Eq` on the borrowed form _must_ match
    /// those for the key type.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use chain_map::ChainMap;
    ///
    /// let mut hash = HashMap::new();
    /// hash.insert("key", "value");
    ///
    /// let mut chain = ChainMap::new();
    /// chain.push_map(hash);
    /// assert_eq!(chain.get("key"), Some(&"value"));
    /// ```
    ///
    /// [`HashMap::get`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.get
    pub fn get<Q>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.inner.iter().find_map(|map| map.get(k))
    }

    /// Returns the highest-precedence key-value pair associated with the given
    /// key.
    ///
    /// As with [`HashMap::get_key_value`], the supplied key may be any
    /// borrowed form of the key type, but `Hash` and `Eq` on the borrowed form
    /// _must_ match those for the key type.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use chain_map::ChainMap;
    ///
    /// let mut hash = HashMap::new();
    /// hash.insert("key", "value");
    ///
    /// let mut chain = ChainMap::new();
    /// chain.push_map(hash);
    /// assert_eq!(chain.get_key_value("key"), Some((&"key", &"value")));
    /// ```
    ///
    /// [`HashMap::get_key_value`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.get_key_value
    pub fn get_key_value<Q>(&self, k: &Q) -> Option<(&K, &V)>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.inner.iter().find_map(|map| map.get_key_value(k))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_map_adds_to_chain() {
        let mut first_map = HashMap::new();
        first_map.insert("first", 1);

        let mut chain = ChainMap::new();
        chain.push_map(first_map);

        assert_eq!(chain.get("first"), Some(&1));
        assert_eq!(chain.get("second"), None);

        let mut second_map = HashMap::new();
        second_map.insert("second", 2);

        chain.push_map(second_map);

        assert_eq!(chain.get("second"), Some(&2));
    }

    #[test]
    fn contains_key_searches_all_maps() {
        let mut first_map = HashMap::new();
        first_map.insert("first", 1);

        let mut second_map = HashMap::new();
        second_map.insert("second", 2);

        let mut third_map = HashMap::new();
        third_map.insert("third", 3);

        let chain: ChainMap<_, _> = vec![first_map, second_map, third_map].into_iter().collect();
        assert!(chain.contains_key("first"));
        assert!(chain.contains_key("second"));
        assert!(chain.contains_key("third"));
        assert!(!chain.contains_key("fourth"));
    }

    #[test]
    fn get_follows_precedence_order() {
        let mut first_map = HashMap::new();
        first_map.insert("first", 1);

        let mut second_map = HashMap::new();
        second_map.insert("first", 1);
        second_map.insert("second", 2);

        let mut third_map = HashMap::new();
        third_map.insert("first", 3);
        third_map.insert("second", 3);
        third_map.insert("third", 3);

        let chain: ChainMap<_, _> = vec![first_map, second_map, third_map].into_iter().collect();

        assert_eq!(chain.get("first"), Some(&1));
        assert_eq!(chain.get("second"), Some(&2));
        assert_eq!(chain.get("third"), Some(&3));
        assert_eq!(chain.get("fourth"), None);
    }

    #[test]
    fn get_key_value_follows_precedence_order() {
        let mut first_map = HashMap::new();
        first_map.insert("first", 1);

        let mut second_map = HashMap::new();
        second_map.insert("first", 1);
        second_map.insert("second", 2);

        let mut third_map = HashMap::new();
        third_map.insert("first", 3);
        third_map.insert("second", 3);
        third_map.insert("third", 3);

        let chain: ChainMap<_, _> = vec![first_map, second_map, third_map].into_iter().collect();

        assert_eq!(chain.get_key_value("first"), Some((&"first", &1)));
        assert_eq!(chain.get_key_value("second"), Some((&"second", &2)));
        assert_eq!(chain.get_key_value("third"), Some((&"third", &3)));
        assert_eq!(chain.get_key_value("fourth"), None);
    }

    #[test]
    fn index_follows_precedence_order() {
        let mut first_map = HashMap::new();
        first_map.insert("first", 1);

        let mut second_map = HashMap::new();
        second_map.insert("first", 1);
        second_map.insert("second", 2);

        let mut third_map = HashMap::new();
        third_map.insert("first", 3);
        third_map.insert("second", 3);
        third_map.insert("third", 3);

        let chain: ChainMap<_, _> = vec![first_map, second_map, third_map].into_iter().collect();

        assert_eq!(chain["first"], 1);
        assert_eq!(chain["second"], 2);
        assert_eq!(chain["third"], 3);
    }

    #[test]
    #[should_panic]
    fn index_panics_when_key_is_not_present() {
        let chain: ChainMap<&str, i32> = ChainMap::new();

        let _ = chain["notset"];
    }

    #[test]
    fn extend_adds_to_end_of_chain() {
        let mut first_map = HashMap::new();
        first_map.insert("first", 1);

        let mut second_map = HashMap::new();
        second_map.insert("first", 1);
        second_map.insert("second", 2);

        let mut third_map = HashMap::new();
        third_map.insert("first", 3);
        third_map.insert("second", 3);
        third_map.insert("third", 3);

        let mut chain: ChainMap<_, _> =
            vec![first_map, second_map, third_map].into_iter().collect();

        assert_eq!(chain.get("first"), Some(&1));
        assert_eq!(chain.get("second"), Some(&2));
        assert_eq!(chain.get("third"), Some(&3));
        assert_eq!(chain.get("fourth"), None);

        let mut fourth_map = HashMap::new();
        fourth_map.insert("first", 4);
        fourth_map.insert("second", 4);
        fourth_map.insert("third", 4);
        fourth_map.insert("fourth", 4);

        chain.extend(vec![fourth_map]);

        assert_eq!(chain.get("first"), Some(&1));
        assert_eq!(chain.get("second"), Some(&2));
        assert_eq!(chain.get("third"), Some(&3));
        assert_eq!(chain.get("fourth"), Some(&4));
    }
}
