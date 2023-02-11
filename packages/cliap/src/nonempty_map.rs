use delegate::delegate;
use std::{
    borrow::Borrow,
    collections::{
        hash_map::{
            Drain, Entry, IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys, RandomState, Values,
            ValuesMut,
        },
        HashMap, TryReserveError,
    },
    hash::{BuildHasher, Hash},
    ops::Index,
};

#[derive(Debug, Clone)]
pub struct NonEmptyHashMap<K, V, S = RandomState> {
    inner: HashMap<K, V, S>,
}

impl<K, V> NonEmptyHashMap<K, V, RandomState>
where
    K: Eq + Hash,
{
    fn new(k: K, v: V) -> Self {
        let mut inner = HashMap::new();
        inner.insert(k, v);
        Self { inner }
    }
}

impl<K, V, S> NonEmptyHashMap<K, V, S>
where
    K: Eq + Hash,
{
    delegate! {
        to self.inner {
            pub fn iter_mut(&mut self) -> IterMut<'_, K, V>;
            pub fn iter(&self) -> Iter<'_, K, V>;
            pub fn into_iter(self) -> IntoIter<K, V>;
            pub fn keys(&self) -> Keys<'_, K, V>;
            pub fn into_keys(self) -> IntoKeys<K, V>;
            pub fn values(&self) -> Values<'_, K, V>;
            pub fn values_mut(&mut self) -> ValuesMut<'_, K, V>;
            pub fn into_values(self) -> IntoValues<K, V>;
            pub fn len(&self) -> usize;
            pub fn drain(&mut self) -> Drain<'_, K, V>;
            pub fn retain<F>(&mut self, f: F)
            where
                F: FnMut(&K, &mut V) -> bool;
            pub fn clear(&mut self);
            pub fn hasher(&self) -> &S;
        }
    }
}

impl<K, V, S> NonEmptyHashMap<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    delegate! {
        to self.inner {
            pub fn reserve(&mut self, additional: usize);
            pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError>;
            pub fn shrink_to_fit(&mut self);
            pub fn shrink_to(&mut self, min_capacity: usize);
            pub fn entry(&mut self, key: K) -> Entry<'_, K, V>;
            pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
            where
                K: Borrow<Q>,
                Q: Hash + Eq;
            pub fn get_key_value<Q: ?Sized>(&self, k: &Q) -> Option<(&K, &V)>
            where
                K: Borrow<Q>,
                Q: Hash + Eq;
            pub fn contains_key<Q: ?Sized>(&self, k: &Q) -> bool
            where
                K: Borrow<Q>,
                Q: Hash + Eq;
            pub fn get_mut<Q: ?Sized>(&mut self, k: &Q) -> Option<&mut V>
            where
                K: Borrow<Q>,
                Q: Hash + Eq;
            pub fn insert(&mut self, k: K, v: V) -> Option<V>;
            pub fn remove<Q: ?Sized>(&mut self, k: &Q) -> Option<V>
            where
                K: Borrow<Q>,
                Q: Hash + Eq;
        }
    }
}

impl<K, V, S> PartialEq for NonEmptyHashMap<K, V, S>
where
    K: Eq + Hash,
    V: PartialEq,
    S: BuildHasher,
{
    fn eq(&self, other: &Self) -> bool {
        self.inner.eq(&other.inner)
    }
}

impl<K, Q: ?Sized, V, S> Index<&Q> for NonEmptyHashMap<K, V, S>
where
    K: Eq + Hash + Borrow<Q>,
    Q: Eq + Hash,
    S: BuildHasher,
{
    type Output = V;

    /// Returns a reference to the value corresponding to the supplied key.
    ///
    /// # Panics
    ///
    /// Panics if the key is not present in the `HashMap`.
    #[inline]
    fn index(&self, key: &Q) -> &V {
        self.inner.index(key)
    }
}
