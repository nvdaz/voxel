use std::{collections::BTreeSet, hash::Hash, mem, ops::RangeBounds};

use indexmap::IndexSet;

use crate::{associated_ord::AssociatedOrd, prelude::*};

pub trait Queue<T> {
    fn push(&mut self, value: impl Into<T>);
    fn pop(&mut self) -> Option<T>;
    fn len(&self) -> usize;
}

#[derive(Resource)]
pub struct UnorderedQueue<T: PartialEq + Eq + Hash> {
    inner: IndexSet<T>,
}

impl<T: PartialEq + Eq + Hash> Queue<T> for UnorderedQueue<T> {
    fn push(&mut self, value: impl Into<T>) {
        self.inner.insert(value.into());
    }

    fn pop(&mut self) -> Option<T> {
        self.inner.pop()
    }

    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<T: PartialEq + Eq + Hash> Default for UnorderedQueue<T> {
    fn default() -> Self {
        Self {
            inner: IndexSet::new(),
        }
    }
}

impl<T: PartialEq + Eq + Hash> UnorderedQueue<T> {
    pub fn drain<R: RangeBounds<usize>>(&mut self, range: R) -> impl Iterator<Item = T> + '_ {
        self.inner.drain(range)
    }
}

#[derive(Resource)]
pub struct OrderedQueue<T: Ord> {
    inner: BTreeSet<T>,
}

impl<T: Ord> Queue<T> for OrderedQueue<T> {
    fn push(&mut self, value: impl Into<T>) {
        self.inner.insert(value.into());
    }

    fn pop(&mut self) -> Option<T> {
        self.inner.pop_first()
    }

    fn len(&self) -> usize {
        self.inner.len()
    }
}

#[derive(Resource)]
pub struct DistanceOrderedQueue<T, R>
where
    T: Into<R>,
    R: DistanceOrd,
{
    inner: BTreeSet<AssociatedOrd<T, R::AsOrd>>,
    center: R,
}

impl<T, R> Default for DistanceOrderedQueue<T, R>
where
    T: Into<R>,
    R: Default + DistanceOrd,
{
    fn default() -> Self {
        Self {
            inner: BTreeSet::new(),
            center: R::default(),
        }
    }
}

impl<T, R> Queue<T> for DistanceOrderedQueue<T, R>
where
    T: Copy + Into<R>,
    R: Copy + DistanceOrd,
{
    fn push(&mut self, value: impl Into<T>) {
        let value = value.into();
        let ord = value.into().distance_ord(self.center);

        self.inner.insert(AssociatedOrd::new(value, ord));
    }

    fn pop(&mut self) -> Option<T> {
        self.inner.pop_first().map(AssociatedOrd::as_value)
    }

    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<T, R> DistanceOrderedQueue<T, R>
where
    T: Copy + Into<R>,
    R: Copy + DistanceOrd,
{
    pub fn update_center(&mut self, center: R) {
        self.inner = mem::take(&mut self.inner)
            .into_iter()
            .map(|mut associated_ord| {
                associated_ord.ord = associated_ord.value.into().distance_ord(center);
                associated_ord
            })
            .collect();
        self.center = center;
    }
}
