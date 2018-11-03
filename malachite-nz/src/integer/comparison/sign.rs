use integer::Integer;
use malachite_base::num::Sign;
use std::cmp::Ordering;

impl Sign for Integer {
    /// Returns the sign of an `Integer`. Interpret the result as the result of a comparison to
    /// zero, so that `Equal` means zero, `Greater` means positive, and `Less` means negative.
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
    /// use malachite_base::num::{Sign, Zero};
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering;
    ///
    /// fn main() {
    ///     assert_eq!(Integer::ZERO.sign(), Ordering::Equal);
    ///     assert_eq!(Integer::from(123).sign(), Ordering::Greater);
    ///     assert_eq!(Integer::from(-123).sign(), Ordering::Less);
    /// }
    /// ```
    fn sign(&self) -> Ordering {
        if self.sign {
            if self.abs == 0 {
                Ordering::Equal
            } else {
                Ordering::Greater
            }
        } else {
            Ordering::Less
        }
    }
}
