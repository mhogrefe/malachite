use integer::Integer;

impl Integer {
    /// Returns the number of trailing zeros in the binary expansion of an `Integer` (equivalently,
    /// the multiplicity of 2 in its prime factorization) or `None` is the `Integer` is 0.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::ZERO.trailing_zeros(), None);
    /// assert_eq!(Integer::from(3).trailing_zeros(), Some(0));
    /// assert_eq!(Integer::from(-72).trailing_zeros(), Some(3));
    /// assert_eq!(Integer::from(100).trailing_zeros(), Some(2));
    /// assert_eq!((-Integer::trillion()).trailing_zeros(), Some(12));
    /// ```
    pub fn trailing_zeros(&self) -> Option<u64> {
        self.abs.trailing_zeros()
    }
}
