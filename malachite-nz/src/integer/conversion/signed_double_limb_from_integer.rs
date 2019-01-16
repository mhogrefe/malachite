use integer::Integer;
use malachite_base::misc::{CheckedFrom, WrappingFrom};
use malachite_base::num::{One, PrimitiveInteger, SignificantBits};
use natural::Natural;
use platform::{DoubleLimb, SignedDoubleLimb};

impl CheckedFrom<Integer> for SignedDoubleLimb {
    /// Converts an `Integer` to a `SignedDoubleLimb`, taking the `Integer` by value and returning `None` if
    /// the `Integer` is outside the range of a `SignedDoubleLimb`.
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
    ///     assert_eq!(format!("{:?}", i64::checked_from(Integer::from(123))), "Some(123)");
    ///     assert_eq!(format!("{:?}", i64::checked_from(Integer::from(-123))), "Some(-123)");
    ///     assert_eq!(
    ///         format!("{:?}",
    ///         i64::checked_from(Integer::from_str("1000000000000000000000000").unwrap())),
    ///         "None");
    ///     assert_eq!(format!("{:?}",
    ///         i64::checked_from(Integer::from_str("-1000000000000000000000000").unwrap())),
    ///         "None");
    /// }
    /// ```
    fn checked_from(value: Integer) -> Option<SignedDoubleLimb> {
        SignedDoubleLimb::checked_from(&value)
    }
}

impl<'a> CheckedFrom<&'a Integer> for SignedDoubleLimb {
    /// Converts an `Integer` to a `SignedDoubleLimb`, taking the `Integer` by reference and returning `None` if
    /// the `Integer` is outside the range of a `SignedDoubleLimb`.
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
    fn checked_from(value: &Integer) -> Option<SignedDoubleLimb> {
        if value.significant_bits() < u64::from(SignedDoubleLimb::WIDTH)
            || *value == -(Natural::ONE << (SignedDoubleLimb::WIDTH - 1))
        {
            Some(SignedDoubleLimb::wrapping_from(value))
        } else {
            None
        }
    }
}

impl WrappingFrom<Integer> for SignedDoubleLimb {
    /// Converts an `Integer` to a `SignedDoubleLimb`, taking the `Integer` by value and wrapping mod
    /// 2<pow>64</pow>.
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
    ///     assert_eq!(i64::wrapping_from(Integer::from(123)), 123);
    ///     assert_eq!(i64::wrapping_from(Integer::from(-123)), -123);
    ///     assert_eq!(
    ///         i64::wrapping_from(Integer::from_str("1000000000000000000000000").unwrap()),
    ///         2003764205206896640);
    ///     assert_eq!(
    ///         i64::wrapping_from(Integer::from_str("-1000000000000000000000000").unwrap()),
    ///         -2003764205206896640);
    /// }
    /// ```
    fn wrapping_from(value: Integer) -> SignedDoubleLimb {
        SignedDoubleLimb::wrapping_from(&value)
    }
}

impl<'a> WrappingFrom<&'a Integer> for SignedDoubleLimb {
    /// Converts an `Integer` to a `SignedDoubleLimb`, taking the `Integer` by reference and wrapping mod
    /// 2<pow>64</pow>.
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
    fn wrapping_from(value: &Integer) -> SignedDoubleLimb {
        DoubleLimb::wrapping_from(value) as SignedDoubleLimb
    }
}
