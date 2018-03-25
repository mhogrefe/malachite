use integer::Integer;
use malachite_base::misc::{CheckedFrom, WrappingFrom};

impl<'a> CheckedFrom<&'a Integer> for u64 {
    /// Converts an `Integer` to a `u64`, returning `None` if the `Integer` is negative or too
    /// large.
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
    /// use malachite_base::misc::CheckedFrom;
    /// use malachite_nz::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(format!("{:?}", u64::checked_from(&Integer::from(123))), "Some(123)");
    ///     assert_eq!(format!("{:?}", u64::checked_from(&Integer::from(-123))), "None");
    ///     assert_eq!(format!("{:?}",
    ///         u64::checked_from(&Integer::from_str("1000000000000000000000").unwrap())), "None");
    ///     assert_eq!(format!("{:?}",
    ///         u64::checked_from(&Integer::from_str("-1000000000000000000000").unwrap())), "None");
    /// }
    /// ```
    fn checked_from(value: &Integer) -> Option<u64> {
        match *value {
            Integer { sign: false, .. } => None,
            Integer {
                sign: true,
                ref abs,
            } => u64::checked_from(abs),
        }
    }
}

impl<'a> WrappingFrom<&'a Integer> for u64 {
    /// Converts an `Integer` to a `u64`, wrapping mod 2<pow>64</pow>.
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
    /// use malachite_base::misc::WrappingFrom;
    /// use malachite_nz::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(u64::wrapping_from(&Integer::from(123)), 123);
    ///     assert_eq!(u64::wrapping_from(&Integer::from(-123)), 18446744073709551493);
    ///     assert_eq!(
    ///         u64::wrapping_from(&Integer::from_str("1000000000000000000000").unwrap()),
    ///         3875820019684212736);
    ///     assert_eq!(
    ///         u64::wrapping_from(&Integer::from_str("-1000000000000000000000").unwrap()),
    ///         14570924054025338880);
    /// }
    /// ```
    fn wrapping_from(value: &Integer) -> u64 {
        match *value {
            Integer {
                sign: true,
                ref abs,
            } => u64::wrapping_from(abs),
            Integer {
                sign: false,
                ref abs,
            } => u64::wrapping_from(abs).wrapping_neg(),
        }
    }
}
