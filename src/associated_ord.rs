pub struct AssociatedOrd<T, O: Ord> {
    pub ord: O,
    pub value: T,
}

impl<T, O: Ord> Ord for AssociatedOrd<T, O> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.ord.cmp(&other.ord)
    }
}

impl<T, O: Ord> PartialOrd for AssociatedOrd<T, O> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.ord.partial_cmp(&other.ord)
    }
}

impl<T, O: Ord> PartialEq for AssociatedOrd<T, O> {
    fn eq(&self, other: &Self) -> bool {
        self.ord.eq(&other.ord)
    }
}

impl<T, O: Ord> Eq for AssociatedOrd<T, O> {}

impl<T, O: Ord> AssociatedOrd<T, O> {
    pub fn new(value: T, ord: O) -> Self {
        Self { value, ord }
    }

    pub fn as_value(self) -> T {
        self.value
    }
}
