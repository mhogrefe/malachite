use integer::Integer;

impl Integer {
    /// Returns the smallest number of bits necessary to represent the absolute value of an
    /// `Integer`. 0 has zero significant bits.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_native;
    ///
    /// use malachite_base::traits::Zero;
    /// use malachite_native::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!(Integer::zero().significant_bits(), 0);
    ///     assert_eq!(Integer::from(100).significant_bits(), 7);
    ///     assert_eq!(Integer::from(-100).significant_bits(), 7);
    /// }
    /// ```
    pub fn significant_bits(&self) -> u64 {
        self.abs.significant_bits()
    }
}
