use integer::Integer;

impl Integer {
    /// Determines whether `self` is even.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::Zero;
    /// use malachite_nz::integer::Integer;
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
        match *self {
            Integer { ref abs, .. } => abs.is_even(),
        }
    }

    /// Determines whether `self` is odd.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::Zero;
    /// use malachite_nz::integer::Integer;
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
        match *self {
            Integer { ref abs, .. } => abs.is_odd(),
        }
    }
}
