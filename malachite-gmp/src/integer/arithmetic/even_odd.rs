use integer::Integer;

impl Integer {
    /// Determines whether `self` is even.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_gmp;
    ///
    /// use malachite_base::traits::Zero;
    /// use malachite_gmp::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!(Integer::ZERO.is_even(), true);
    ///     assert_eq!(Integer::from(123).is_even(), false);
    ///     assert_eq!(Integer::from(-0x80).is_even(), true);
    ///     assert_eq!(Integer::trillion().is_even(), true);
    ///     assert_eq!((-Integer::trillion() - 1u32).is_even(), false);
    /// }
    /// ```
    pub fn is_even(&self) -> bool {
        self.to_u32_wrapping() & 1 == 0
    }

    /// Determines whether `self` is odd.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_gmp;
    ///
    /// use malachite_base::traits::Zero;
    /// use malachite_gmp::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!(Integer::ZERO.is_odd(), false);
    ///     assert_eq!(Integer::from(123).is_odd(), true);
    ///     assert_eq!(Integer::from(-0x80).is_odd(), false);
    ///     assert_eq!(Integer::trillion().is_odd(), false);
    ///     assert_eq!((-Integer::trillion() - 1u32).is_odd(), true);
    /// }
    /// ```
    pub fn is_odd(&self) -> bool {
        self.to_u32_wrapping() & 1 != 0
    }
}
