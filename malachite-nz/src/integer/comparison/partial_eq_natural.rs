use crate::integer::Integer;
use crate::natural::Natural;

impl PartialEq<Natural> for Integer {
    /// Determines whether an [`Integer`] is equal to a [`Natural`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where n = `min(self.significant_bits(), other.significant_bits())`
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert!(Integer::from(123) == Natural::from(123u32));
    /// assert!(Integer::from(123) != Natural::from(5u32));
    /// ```
    fn eq(&self, other: &Natural) -> bool {
        self.sign && self.abs == *other
    }
}

impl PartialEq<Integer> for Natural {
    /// Determines whether a [`Natural`] is equal to an [`Integer`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where n = `min(self.significant_bits(), other.significant_bits())`
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert!(Natural::from(123u32) == Integer::from(123));
    /// assert!(Natural::from(123u32) != Integer::from(5));
    /// ```
    fn eq(&self, other: &Integer) -> bool {
        other.sign && *self == other.abs
    }
}
