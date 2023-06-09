use std::{collections::VecDeque, hash::Hash};

use bevy::utils::HashSet;

use crate::prelude::*;

pub trait Queue<T> {
    fn push(&mut self, value: impl Into<T>);
}

#[derive(Resource)]
pub struct UnorderedQueue<T: PartialEq + Eq + Hash> {
    inner: HashSet<T>,
}

impl<T: PartialEq + Eq + Hash> Queue<T> for UnorderedQueue<T> {
    fn push(&mut self, value: impl Into<T>) {
        self.inner.insert(value.into());
    }
}

impl<T: PartialEq + Eq + Hash> Default for UnorderedQueue<T> {
    fn default() -> Self {
        Self {
            inner: HashSet::new(),
        }
    }
}

impl<T: PartialEq + Eq + Hash> UnorderedQueue<T> {
    pub fn drain(&mut self) -> impl Iterator<Item = T> + '_ {
        self.inner.drain()
    }
}

#[derive(Resource)]
pub struct OrderedQueue<T> {
    inner: VecDeque<T>,
}

impl<T> Queue<T> for OrderedQueue<T> {
    fn push(&mut self, value: impl Into<T>) {
        self.inner.push_back(value.into());
    }
}

impl<T> Default for OrderedQueue<T> {
    fn default() -> Self {
        Self {
            inner: VecDeque::new(),
        }
    }
}

impl<T> OrderedQueue<T> {
    pub fn pop(&mut self) -> Option<T> {
        self.inner.pop_front()
    }

    pub fn drain(&mut self) -> impl Iterator<Item = T> + '_ {
        self.inner.drain(..)
    }
}
