use std::{hash::Hash, sync::Arc};

use dashmap::DashMap;
use futures_util::future::{BoxFuture, Shared};


pub enum FutureCacheResult<T> {
    Hit(Arc<T>),
    Waiting(Shared<BoxFuture<'static, Arc<T>>>),
}

pub struct FutureTaskCache<K, V> {
    futures: DashMap<K, Shared<BoxFuture<'static, Arc<V>>>>,
    results: DashMap<K, Arc<V>>,
}

impl<K, V> FutureTaskCache<K, V>
where
    K: Eq + Hash,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, key: &K) -> Option<FutureCacheResult<V>> {
        if let Some(result) = self.results.get(key) {
            Some(FutureCacheResult::Hit(result.clone()))
        } else if let Some(future) = self.futures.get(key) {
            Some(FutureCacheResult::Waiting(future.clone()))
        } else {
            None
        }
    }

    pub fn insert_future(&self, key: K, future: Shared<BoxFuture<'static, Arc<V>>>) {
        self.futures.insert(key, future);
    }

    pub fn insert_result(&self, key: K, result: Arc<V>) {
        self.results.insert(key, result);
    }

    pub fn remove_future(&self, key: &K) {
        self.futures.remove(key);
    }

    pub fn remove_result(&self, key: &K) {
        self.results.remove(key);
    }
}

impl<K: Eq + Hash, V> Default for FutureTaskCache<K, V> {
    fn default() -> Self {
        Self {
            futures: DashMap::default(),
            results: DashMap::default(),
        }
    }
}
