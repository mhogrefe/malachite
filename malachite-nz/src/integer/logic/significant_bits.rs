use integer::Integer;
use malachite_base::num::SignificantBits;

impl SignificantBits for Integer {
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
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::SignificantBits;
    /// use malachite_base::num::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!(Integer::ZERO.significant_bits(), 0);
    ///     assert_eq!(Integer::from(100).significant_bits(), 7);
    ///     assert_eq!(Integer::from(-100).significant_bits(), 7);
    /// }
    /// ```
    fn significant_bits(&self) -> u64 {
        self.abs.significant_bits()
    }
}
