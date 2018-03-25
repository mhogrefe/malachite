use integer::Integer;
use malachite_base::misc::{CheckedFrom, WrappingFrom};

impl<'a> CheckedFrom<&'a Integer> for u32 {
    /// Converts an `Integer` to a `u32`, returning `None` if the `Integer` is negative or too
    /// large.
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
    /// use malachite_base::misc::CheckedFrom;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!(format!("{:?}", u32::checked_from(&Integer::from(123))), "Some(123)");
    ///     assert_eq!(format!("{:?}", u32::checked_from(&Integer::from(-123))), "None");
    ///     assert_eq!(format!("{:?}", u32::checked_from(&Integer::trillion())), "None");
    ///     assert_eq!(format!("{:?}", u32::checked_from(&(-Integer::trillion()))), "None");
    /// }
    /// ```
    fn checked_from(value: &Integer) -> Option<u32> {
        match *value {
            Integer { sign: false, .. } => None,
            Integer {
                sign: true,
                ref abs,
            } => u32::checked_from(abs),
        }
    }
}

impl<'a> WrappingFrom<&'a Integer> for u32 {
    /// Converts an `Integer` to a `u32`, wrapping mod 2<sup>32</sup>.
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
    /// use malachite_base::misc::WrappingFrom;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!(u32::wrapping_from(&Integer::from(123)), 123);
    ///     assert_eq!(u32::wrapping_from(&Integer::from(-123)), 4294967173);
    ///     assert_eq!(u32::wrapping_from(&Integer::trillion()), 3567587328);
    ///     assert_eq!(u32::wrapping_from(&(-Integer::trillion())), 727379968);
    /// }
    /// ```
    fn wrapping_from(value: &Integer) -> u32 {
        match *value {
            Integer {
                sign: true,
                ref abs,
            } => u32::wrapping_from(abs),
            Integer {
                sign: false,
                ref abs,
            } => u32::wrapping_from(abs).wrapping_neg(),
        }
    }
}
