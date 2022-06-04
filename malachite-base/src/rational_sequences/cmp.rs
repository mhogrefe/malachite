use rational_sequences::RationalSequence;
use std::cmp::Ordering;

impl<T: Eq + Ord> PartialOrd for RationalSequence<T> {
    /// Compares a [`RationalSequence`] to another [`RationalSequence`].
    ///
    /// See [here](RationalSequence::cmp) for more information.
    #[inline]
    fn partial_cmp(&self, other: &RationalSequence<T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Eq + Ord> Ord for RationalSequence<T> {
    /// Compares a [`RationalSequence`] to another [`RationalSequence`].
    ///
    /// The comparison is made lexicographically with respect to the element type's ordering.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.component_len()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rational_sequences::RationalSequence;
    ///
    /// assert!(
    ///     RationalSequence::from_slice(&[1, 2]) <
    ///         RationalSequence::from_slices(&[1, 2], &[1])
    /// );
    /// assert!(
    ///     RationalSequence::from_slice(&[1, 2, 3]) <
    ///         RationalSequence::from_slices(&[1, 2], &[3, 4])
    /// );
    /// ```
    fn cmp(&self, other: &RationalSequence<T>) -> Ordering {
        if self == other {
            Ordering::Equal
        } else {
            Iterator::cmp(self.iter(), other.iter())
        }
    }
}
