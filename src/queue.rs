use std::{collections::BTreeSet, hash::Hash, marker::PhantomData, mem, ops::RangeBounds};

use indexmap::IndexSet;

use crate::{associated_ord::AssociatedOrd, prelude::*};

pub trait Queue<T> {
    fn push(&mut self, value: T);
    fn pop(&mut self) -> Option<T>;
    fn remove(&mut self, value: &T);
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[derive(Resource)]
pub struct UnorderedQueue<T: PartialEq + Eq + Hash, M> {
    inner: IndexSet<T>,
    marker: PhantomData<M>,
}

impl<T: PartialEq + Eq + Hash, M> Queue<T> for UnorderedQueue<T, M> {
    fn push(&mut self, value: T) {
        self.inner.insert(value);
    }

    fn pop(&mut self) -> Option<T> {
        self.inner.pop()
    }

    fn remove(&mut self, value: &T) {
        self.inner.remove(value);
    }

    fn len(&self) -> usize {
        self.inner.len()
    }

    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl<T: PartialEq + Eq + Hash, M> Default for UnorderedQueue<T, M> {
    fn default() -> Self {
        Self {
            inner: IndexSet::new(),
            marker: PhantomData,
        }
    }
}

impl<T: PartialEq + Eq + Hash, M> UnorderedQueue<T, M> {
    pub fn drain<R: RangeBounds<usize>>(&mut self, range: R) -> impl Iterator<Item = T> + '_ {
        self.inner.drain(range)
    }
}

#[derive(Resource)]
pub struct OrderedQueue<T: Ord> {
    inner: BTreeSet<T>,
}

impl<T: Ord> Queue<T> for OrderedQueue<T> {
    fn push(&mut self, value: T) {
        self.inner.insert(value);
    }

    fn pop(&mut self) -> Option<T> {
        self.inner.pop_first()
    }

    fn remove(&mut self, value: &T) {
        self.inner.remove(value);
    }

    fn len(&self) -> usize {
        self.inner.len()
    }

    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

#[derive(Resource)]
pub struct DistanceOrderedQueue<T, M>
where
    T: DistanceOrd,
{
    inner: BTreeSet<AssociatedOrd<T, T::AsOrd>>,
    center: T,
    marker: PhantomData<M>,
}

impl<T, M> Default for DistanceOrderedQueue<T, M>
where
    T: Default + DistanceOrd,
{
    fn default() -> Self {
        Self {
            inner: BTreeSet::new(),
            center: T::default(),
            marker: PhantomData,
        }
    }
}

impl<T, M> Queue<T> for DistanceOrderedQueue<T, M>
where
    T: Copy + DistanceOrd,
{
    fn push(&mut self, value: T) {
        let ord = value.distance_ord(self.center);

        self.inner.insert(AssociatedOrd::new(value, ord));
    }

    fn remove(&mut self, value: &T) {
        let ord = value.distance_ord(self.center);
        let associated_ord = AssociatedOrd::new(*value, ord);
        self.inner.remove(&associated_ord);
    }

    fn pop(&mut self) -> Option<T> {
        self.inner.pop_first().map(AssociatedOrd::as_value)
    }

    fn len(&self) -> usize {
        self.inner.len()
    }

    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl<T, M> DistanceOrderedQueue<T, M>
where
    T: Copy + DistanceOrd,
{
    pub fn update_center(&mut self, center: T) {
        self.inner = mem::take(&mut self.inner)
            .into_iter()
            .map(|mut associated_ord| {
                associated_ord.ord = associated_ord.value.distance_ord(center);
                associated_ord
            })
            .collect();
        self.center = center;
    }
}
