use integer::Integer;
use malachite_base::num::logic::traits::CountOnes;

impl Integer {
    /// Counts the number of ones in the binary expansion of an `Integer`. If the `Integer` is
    /// negative, the number of ones is infinite, so `None` is returned.
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
    /// assert_eq!(Integer::ZERO.checked_count_ones(), Some(0));
    /// // 105 = 1101001b
    /// assert_eq!(Integer::from(105).checked_count_ones(), Some(4));
    /// assert_eq!(Integer::from(-105).checked_count_ones(), None);
    /// // 10^12 = 1110100011010100101001010001000000000000b
    /// assert_eq!(Integer::trillion().checked_count_ones(), Some(13));
    /// ```
    pub fn checked_count_ones(&self) -> Option<u64> {
        if self.sign {
            Some(self.abs.count_ones())
        } else {
            None
        }
    }
}
