// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use alloc::vec::Vec;
/// Remembers values produced by an iterator.
///
/// After wrapping an iterator with an `IteratorCache`, you can retrieve a reference to the $n$th
/// element of an iterator, and then retrieve a reference to the $m$th element in constant time for
/// any $m \leq n$ (not counting the time it took to first get the $n$th element).
#[derive(Clone, Debug)]
pub struct IteratorCache<I: Iterator> {
    xs: I,
    cache: Vec<I::Item>,
    done: bool,
}

impl<I: Iterator> IteratorCache<I> {
    /// Creates a new `IteratorCache`.
    ///
    /// This function does not allocate any memory or advance the iterator.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::iterators::iterator_cache::IteratorCache;
    ///
    /// IteratorCache::new([1, 2, 3].iter());
    /// ```
    pub const fn new(xs: I) -> IteratorCache<I> {
        IteratorCache {
            xs,
            cache: Vec::new(),
            done: false,
        }
    }

    /// Retrieves the $n$th element of an iterator. Indexing starts at 0.
    ///
    /// If the index is higher than any other previously-requested index, the iterator is advanced
    /// to that index, or until it runs out. If the iterator has previously been advanced past the
    /// index, the requested element is returned from the cache in constant time. If the iterator is
    /// too short to have an element at the index, `None` is returned.
    ///
    /// If you know that the element is present, and want to only take an immutable reference to
    /// `self`, consider using [`assert_get`](Self::assert_get) instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is 1 if `get` has previously been
    /// called with an index at least this large, or `index` otherwise.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::iterators::iterator_cache::IteratorCache;
    ///
    /// let mut xs = IteratorCache::new([1, 2, 3].iter().cloned());
    /// assert_eq!(xs.get(1), Some(&2));
    /// assert_eq!(xs.get(0), Some(&1));
    /// assert_eq!(xs.get(3), None);
    /// assert_eq!(xs.get(2), Some(&3));
    /// ```
    pub fn get(&mut self, index: usize) -> Option<&I::Item> {
        if !self.done && index >= self.cache.len() {
            self.cache
                .extend((&mut self.xs).take(index - self.cache.len() + 1));
            if index >= self.cache.len() {
                self.done = true;
                return None;
            }
        }
        self.cache.get(index)
    }

    /// Retrieves the $n$th element of an iterator, while asserting that the iterator has already
    /// reached that element. Indexing starts at 0.
    ///
    /// If the iterator has not advanced that far, or if it has fewer than $n + 1$ elements, this
    /// function panics.
    ///
    /// The purpose of this function is to allow the caller to get an element immutably, assuming
    /// that the caller knows that the element is present. The [`get`](Self::get) function has to
    /// take a mutable reference.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::iterators::iterator_cache::IteratorCache;
    ///
    /// let mut xs = IteratorCache::new([1, 2, 3].iter().cloned());
    /// // Force the iterator to iterate to completion
    /// xs.get(3);
    /// assert_eq!(xs.assert_get(1), &2);
    /// assert_eq!(xs.assert_get(0), &1);
    /// assert_eq!(xs.assert_get(2), &3);
    /// ```
    pub fn assert_get(&self, index: usize) -> &I::Item {
        self.cache.get(index).unwrap()
    }

    /// Returns the total number of elements in the iterator, if the iterator has been completely
    /// consumed.
    ///
    /// If the iterator has not been completely consumed yet, returns `None`.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::iterators::iterator_cache::IteratorCache;
    ///
    /// let mut xs = IteratorCache::new([1, 2, 3].iter().cloned());
    /// assert_eq!(xs.known_len(), None);
    /// assert_eq!(xs.get(1), Some(&2));
    /// assert_eq!(xs.known_len(), None);
    /// assert_eq!(xs.get(0), Some(&1));
    /// assert_eq!(xs.get(3), None);
    /// assert_eq!(xs.known_len(), Some(3));
    /// assert_eq!(xs.get(2), Some(&3));
    /// ```
    pub fn known_len(&self) -> Option<usize> {
        if self.done {
            Some(self.cache.len())
        } else {
            None
        }
    }
}
