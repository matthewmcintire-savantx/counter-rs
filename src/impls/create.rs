use crate::Counter;

use num_traits::{One, Zero};

use std::hash::Hash;
use std::iter;
use std::ops::AddAssign;

impl<T, N> Default for Counter<T, N>
where
    T: Hash + Eq,
    N: Default,
{
    fn default() -> Self {
        Self {
            map: Default::default(),
            zero: Default::default(),
        }
    }
}

impl<T, N> iter::FromIterator<T> for Counter<T, N>
where
    T: Hash + Eq,
    N: AddAssign + Zero + One,
{
    /// Produce a `Counter` from an iterator of items. This is called automatically
    /// by [`Iterator::collect()`].
    ///
    /// [`Iterator::collect()`]:
    /// https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.collect
    ///
    /// ```rust
    /// # use counter::Counter;
    /// # use std::collections::HashMap;
    /// let counter = "abbccc".chars().collect::<Counter<_>>();
    /// let expect = [('a', 1), ('b', 2), ('c', 3)].iter().cloned().collect::<HashMap<_, _>>();
    /// assert_eq!(counter.into_map(), expect);
    /// ```
    ///
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Counter::<T, N>::init(iter)
    }
}

impl<T, N> iter::FromIterator<(T, N)> for Counter<T, N>
where
    T: Hash + Eq,
    N: AddAssign + Zero,
{
    /// Creates a counter from `(item, count)` tuples.
    ///
    /// The counts of duplicate items are summed.
    /// ```rust
    /// # use counter::Counter;
    /// # use std::collections::HashMap;
    /// let counter = [('a', 1), ('b', 2), ('c', 3), ('a', 4)].iter()
    ///     .cloned().collect::<Counter<_>>();
    /// let expect = [('a', 5), ('b', 2), ('c', 3)].iter()
    ///     .cloned().collect::<HashMap<_, _>>();
    /// assert_eq!(counter.into_map(), expect);
    /// ```
    fn from_iter<I: IntoIterator<Item = (T, N)>>(iter: I) -> Self {
        let mut cnt = Counter::new();
        for (item, item_count) in iter {
            let entry = cnt.map.entry(item).or_insert_with(N::zero);
            *entry += item_count;
        }
        cnt
    }
}
