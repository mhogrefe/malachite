/// Remembers values produced by an iterator.
///
/// After wrapping an iterator with an `IteratorCache`, you can retrieve a reference to the $n$th
/// element of an iterator, and then retrieve a reference to the $m$th element in constant time for
/// any $m \leq n$.
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
    /// let xs = IteratorCache::new([1, 2, 3].iter());
    /// ```
    pub fn new(xs: I) -> IteratorCache<I> {
        IteratorCache {
            xs,
            cache: Vec::new(),
            done: false,
        }
    }

    /// Retrieves the $n$th element of an iterator (the first element is at index 0).
    ///
    /// If the index is higher than any other previously-requested index, the iterator is advanced
    /// to that index, or until it runs out. If the iterator has previously been advanced past the
    /// index, the requested element is returned from the cache in constant time. If the iterator is
    /// too short to have an element at the index, `None` is returned.
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
