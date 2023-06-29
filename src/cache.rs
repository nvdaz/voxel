use std::{collections::BTreeMap, hash::Hash};

use bevy::utils::HashMap;
use futures_lite::future::{block_on, poll_once};
use futures_util::future::{BoxFuture, Shared};

use crate::prelude::*;

pub trait Cache<'a, K, V>
where
    K: 'a,
    V: 'a,
{
    type Iterator: Iterator<Item = (&'a K, &'a V)>;

    fn len(&self) -> usize;
    fn get(&self, key: &K) -> Option<&V>;
    fn insert(&mut self, key: K, value: V);
    fn iter(&'a self) -> Self::Iterator;
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T);
}

pub trait SizedCache<'a, K, V>: Cache<'a, K, V>
where
    K: 'a,
    V: 'a,
{
    fn max_size(&self) -> usize;
}

#[derive(Resource)]
pub struct IndefiniteCache<K, V>
where
    K: Eq + Hash,
{
    cache: HashMap<K, V>,
}

impl<K, V> IndefiniteCache<K, V>
where
    K: Eq + Hash,
{
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    // pub fn inner(&mut self) -> &mut HashMap<K, V> {
    //     &mut self.cache
    // }
}

impl<K, V> Default for IndefiniteCache<K, V>
where
    K: Eq + Hash,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, K, V> Cache<'a, K, V> for IndefiniteCache<K, V>
where
    K: Eq + Hash + 'a,
    V: 'a,
{
    type Iterator = bevy::utils::hashbrown::hash_map::Iter<'a, K, V>;

    fn len(&self) -> usize {
        self.cache.len()
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.cache.get(key)
    }

    fn insert(&mut self, key: K, value: V) {
        self.cache.insert(key, value);
    }

    fn iter(&'a self) -> Self::Iterator {
        self.cache.iter()
    }

    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        self.cache.extend(iter);
    }
}

pub type FutureCache<K, V> = IndefiniteCache<K, Shared<BoxFuture<'static, V>>>;

impl<K: Copy + Eq + Hash, V: Clone> FutureCache<K, V> {
    pub fn drain_completed(&mut self) -> impl Iterator<Item = (K, V)> + '_ {
        let mut completed = HashMap::new();

        self.cache.retain(|position, task| {
            if let Some(result) = block_on(poll_once(task)) {
                completed.insert(*position, result);
                false
            } else {
                true
            }
        });

        completed.into_iter()
    }
}

#[derive(Resource)]
pub struct OrderedCache<K, V> {
    max_size: usize,
    cache: BTreeMap<K, V>,
}

impl<K, V> OrderedCache<K, V> {
    pub fn new(max_size: usize) -> Self {
        Self {
            max_size,
            cache: BTreeMap::new(),
        }
    }
}

impl<'a, K, V> Cache<'a, K, V> for OrderedCache<K, V>
where
    K: Eq + Hash + Ord + 'a,
    V: 'a,
{
    type Iterator = std::collections::btree_map::Iter<'a, K, V>;

    fn len(&self) -> usize {
        self.cache.len()
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.cache.get(key)
    }

    fn insert(&mut self, key: K, value: V) {
        self.cache.insert(key, value);

        // if self.len() > self.max_size() {
        //     self.cache.pop_last();
        // }
    }

    fn iter(&'a self) -> Self::Iterator {
        self.cache.iter()
    }

    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        self.cache.extend(iter);
    }
}

impl<'a, K: Eq + Hash + Ord + 'a, V: 'a> SizedCache<'a, K, V> for OrderedCache<K, V> {
    fn max_size(&self) -> usize {
        self.max_size
    }
}
