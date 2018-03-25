use integer::Integer;
use malachite_base::misc::{CheckedFrom, WrappingFrom};
use malachite_base::num::{One, SignificantBits};
use natural::Natural;

impl<'a> CheckedFrom<&'a Integer> for i64 {
    /// Converts an `Integer` to an `i64`, returning `None` if the `Integer` is outside the range of
    /// an `i64`.
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
    ///     assert_eq!(format!("{:?}", i64::checked_from(&Integer::from(123))), "Some(123)");
    ///     assert_eq!(format!("{:?}", i64::checked_from(&Integer::from(-123))), "Some(-123)");
    ///     assert_eq!(
    ///         format!("{:?}",
    ///         i64::checked_from(&Integer::from_str("1000000000000000000000000").unwrap())),
    ///         "None");
    ///     assert_eq!(format!("{:?}",
    ///         i64::checked_from(&Integer::from_str("-1000000000000000000000000").unwrap())),
    ///         "None");
    /// }
    /// ```
    fn checked_from(value: &Integer) -> Option<i64> {
        if value.significant_bits() < 64 || *value == -((Natural::ONE << 63u32).into_integer()) {
            Some(i64::wrapping_from(value))
        } else {
            None
        }
    }
}

impl<'a> WrappingFrom<&'a Integer> for i64 {
    /// Converts an `Integer` to a `i64`, wrapping mod 2<pow>64</pow>.
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
    ///     assert_eq!(i64::wrapping_from(&Integer::from(123)), 123);
    ///     assert_eq!(i64::wrapping_from(&Integer::from(-123)), -123);
    ///     assert_eq!(
    ///         i64::wrapping_from(&Integer::from_str("1000000000000000000000000").unwrap()),
    ///         2003764205206896640);
    ///     assert_eq!(
    ///         i64::wrapping_from(&Integer::from_str("-1000000000000000000000000").unwrap()),
    ///         -2003764205206896640);
    /// }
    /// ```
    fn wrapping_from(value: &Integer) -> i64 {
        u64::wrapping_from(value) as i64
    }
}
